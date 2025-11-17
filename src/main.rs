mod cli;
mod config;
mod project;
mod registry;
mod stack;
mod verbs;

use crate::cli::{Cli, Commands};
use crate::project::{ProjectContext, ResolvedCommand};
use crate::registry::Registry;
use crate::verbs::Verb;
use anyhow::{Context, Result, anyhow, bail};
use chrono::Local;
use clap::Parser;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, ExitStatus};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut registry = Registry::load()?;
    let command = cli.command;

    if let Some(verb) = command.verb() {
        let ctx = ProjectContext::from_current_dir()?;
        execute_with_context(ctx, verb, &mut registry)?;
        return Ok(());
    }

    match command {
        Commands::Info => {
            let ctx = ProjectContext::from_current_dir()?;
            show_info(&ctx);
        }
        Commands::Detect => {
            let ctx = ProjectContext::from_current_dir()?;
            print_detection(&ctx);
        }
        Commands::Recent => {
            show_recent(&registry);
        }
        Commands::Resume { project } => {
            handle_resume(project, &mut registry)?;
        }
        Commands::Switch { project, path_only } => {
            handle_switch(project, path_only, &registry)?;
        }
        Commands::Init { force } => {
            let ctx = ProjectContext::from_current_dir()?;
            handle_init(&ctx, force)?;
        }
        Commands::Doctor => {
            let ctx = ProjectContext::from_current_dir()?;
            run_doctor(&ctx)?;
        }
        Commands::Version => {
            println!("axl {}", env!("CARGO_PKG_VERSION"));
        }
        // Verb commands handled by earlier branch
        Commands::Dev
        | Commands::Build
        | Commands::Test
        | Commands::Clean
        | Commands::Reset
        | Commands::Open
        | Commands::Logs => unreachable!("verb commands handled earlier"),
    }

    Ok(())
}

fn execute_with_context(ctx: ProjectContext, verb: Verb, registry: &mut Registry) -> Result<()> {
    let resolved = ctx.resolved_command(verb).ok_or_else(|| {
        anyhow!("no command defined for '{verb}'. Run `axl init` to scaffold axl.toml")
    })?;

    ensure_requirements(&resolved.requires)?;

    println!("▶ {} ({})", resolved.cmd, resolved.origin_label());
    println!("   dir: {}", ctx.root.display());
    io::stdout().flush().ok();

    let status = spawn_command(&resolved, &ctx.root)?;
    registry.update_usage(&ctx.root, ctx.stack, ctx.project_name(), verb)?;

    if !status.success() {
        if let Some(code) = status.code() {
            std::process::exit(code);
        } else {
            bail!("command terminated by signal");
        }
    }

    Ok(())
}

fn spawn_command(command: &ResolvedCommand, dir: &Path) -> Result<ExitStatus> {
    let (shell, flag) = if cfg!(windows) {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let mut child = Command::new(shell);
    child.arg(flag).arg(&command.cmd).current_dir(dir);
    child.env("AXL_VERB", command.verb.as_str());
    child.env("AXL_ROOT", dir);
    child.envs(&command.env);
    child.stdin(std::process::Stdio::inherit());
    child.stdout(std::process::Stdio::inherit());
    child.stderr(std::process::Stdio::inherit());

    child
        .status()
        .with_context(|| format!("running '{}' in {}", command.cmd, dir.display()))
}

fn ensure_requirements(tools: &[String]) -> Result<()> {
    let missing = missing_requirements(tools);
    if missing.is_empty() {
        Ok(())
    } else {
        bail!("missing required tools: {}", missing.join(", "))
    }
}

fn missing_requirements(tools: &[String]) -> Vec<String> {
    tools
        .iter()
        .filter(|tool| which::which(tool).is_err())
        .cloned()
        .collect()
}

fn show_info(ctx: &ProjectContext) {
    println!("Project  : {}", ctx.project_name());
    println!("Root     : {}", ctx.root.display());
    println!("Stack    : {}", ctx.stack.label());
    println!("Detection: {}", ctx.detection.reason);
    if ctx.has_config() {
        println!("Config   : {}", ctx.config_path().display());
    } else {
        println!("Config   : (none)");
    }
    println!();
    println!("Verbs:");
    for verb in Verb::ALL {
        match ctx.resolved_command(verb) {
            Some(cmd) => println!("  {:<5} → {:<40} ({})", verb, cmd.cmd, cmd.origin_label()),
            None => println!("  {:<5} → <not defined>", verb),
        }
    }
}

fn print_detection(ctx: &ProjectContext) {
    println!("Stack    : {}", ctx.stack.label());
    println!("Detected : {}", ctx.detection.reason);
    println!("Root     : {}", ctx.root.display());
    if ctx.has_config() {
        println!("axl.toml : {}", ctx.config_path().display());
    }
}

fn show_recent(registry: &Registry) {
    let entries = registry.recent();
    if entries.is_empty() {
        println!("No projects in registry yet.");
        return;
    }

    println!(
        "{:<3} {:<24} {:<14} {:<8} {}",
        "#", "Project", "Stack", "Verb", "Last used"
    );
    for (idx, entry) in entries.into_iter().enumerate() {
        let verb = entry
            .last_used_verb
            .map(|v| v.to_string())
            .unwrap_or_else(|| "-".into());
        let timestamp = entry
            .last_used_at
            .map(|dt| {
                dt.with_timezone(&Local)
                    .format("%Y-%m-%d %H:%M")
                    .to_string()
            })
            .unwrap_or_else(|| "-".into());
        println!(
            "{:<3} {:<24} {:<14} {:<8} {}",
            idx + 1,
            entry.project_name,
            entry.stack.label(),
            verb,
            timestamp
        );
    }
}

fn handle_resume(project: Option<String>, registry: &mut Registry) -> Result<()> {
    let entry = registry
        .find(project.as_deref())
        .ok_or_else(|| anyhow!("no matching project in registry"))?;
    let verb = entry
        .last_used_verb
        .ok_or_else(|| anyhow!("project has no recorded verb to resume"))?;
    println!(
        "Resuming {} • {} ({})",
        entry.project_name,
        verb,
        entry.path.display()
    );
    let ctx = ProjectContext::from_path(&entry.path)?;
    execute_with_context(ctx, verb, registry)
}

fn handle_switch(project: Option<String>, path_only: bool, registry: &Registry) -> Result<()> {
    let entry = registry
        .find(project.as_deref())
        .ok_or_else(|| anyhow!("no matching project in registry"))?;
    if path_only {
        println!("{}", entry.path.display());
    } else {
        println!("cd {}", entry.path.display());
    }
    Ok(())
}

fn handle_init(ctx: &ProjectContext, force: bool) -> Result<()> {
    let path = ctx.config_path();
    if path.exists() && !force {
        bail!(
            "{} already exists. Use --force to overwrite.",
            path.display()
        );
    }

    let mut doc = String::new();
    doc.push_str("[project]\n");
    doc.push_str(&format!("name = \"{}\"\n", ctx.project_name()));
    doc.push_str(&format!("stack = \"{}\"\n\n", ctx.stack.keyword()));

    for verb in Verb::ALL {
        if let Some(default) = ctx.stack.default_command(verb) {
            let crate::stack::DefaultCommand { cmd, requires } = default;
            doc.push_str(&format!("[{}]\n", verb));
            let cmd_line = cmd.replace('"', "\\\"");
            doc.push_str(&format!("cmd = \"{}\"\n", cmd_line));
            if !requires.is_empty() {
                doc.push_str("requires = [");
                doc.push_str(
                    &requires
                        .iter()
                        .map(|r| format!("\"{r}\""))
                        .collect::<Vec<_>>()
                        .join(", "),
                );
                doc.push_str("]\n");
            }
            doc.push('\n');
        }
    }

    std::fs::write(path, doc).with_context(|| format!("writing {}", path.display()))?;
    println!("Created {}", path.display());
    Ok(())
}

fn run_doctor(ctx: &ProjectContext) -> Result<()> {
    println!("AXL doctor for {}", ctx.project_name());
    println!("Root   : {}", ctx.root.display());
    println!("Stack  : {}", ctx.stack.label());
    println!();

    let mut issues = Vec::new();
    for verb in Verb::ALL {
        if let Some(command) = ctx.resolved_command(verb) {
            let missing = missing_requirements(&command.requires);
            if missing.is_empty() {
                println!("[ok] {verb} → {}", command.cmd);
            } else {
                println!(
                    "[missing] {verb} ({}) → {}",
                    missing.join(", "),
                    command.cmd
                );
                issues.push((verb, missing));
            }
        } else {
            println!("[warn] {verb} has no mapped command");
        }
    }

    if issues.is_empty() {
        println!("\nEverything looks good.");
    } else {
        println!("\nMissing requirements detected:");
        for (verb, missing) in issues {
            println!("  - {verb}: {}", missing.join(", "));
        }
    }

    Ok(())
}
