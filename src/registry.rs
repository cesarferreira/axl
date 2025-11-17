use crate::stack::Stack;
use crate::verbs::Verb;
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use directories::BaseDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Registry {
    path: PathBuf,
    entries: Vec<ProjectEntry>,
}

impl Registry {
    pub fn load() -> Result<Self> {
        let dir = config_root();
        fs::create_dir_all(&dir).context("creating axl config directory")?;
        let path = dir.join("projects.json");
        let entries = if path.exists() {
            let data =
                fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
            serde_json::from_str(&data).with_context(|| format!("parsing {}", path.display()))?
        } else {
            Vec::new()
        };

        Ok(Self { path, entries })
    }

    pub fn update_usage(
        &mut self,
        root: &Path,
        stack: Stack,
        name: &str,
        verb: Verb,
    ) -> Result<()> {
        let now = Utc::now();
        let path = root.to_path_buf();

        let entry_index = self.entries.iter().position(|entry| entry.path == path);

        let len = self.entries.len();
        if entry_index.is_none() {
            self.entries.push(ProjectEntry {
                path: path.clone(),
                project_name: name.to_string(),
                stack,
                last_used_verb: None,
                last_used_at: None,
            });
        }
        let idx = entry_index.unwrap_or(len);
        let entry = self.entries.get_mut(idx).expect("entry exists");

        entry.project_name = name.to_string();
        entry.stack = stack;
        entry.last_used_verb = Some(verb);
        entry.last_used_at = Some(now);

        self.save()
    }

    pub fn recent(&self) -> Vec<&ProjectEntry> {
        let mut indices: Vec<usize> = (0..self.entries.len()).collect();
        indices.sort_by(|&a, &b| {
            self.entries[b]
                .last_used_at
                .cmp(&self.entries[a].last_used_at)
        });
        indices.into_iter().map(|idx| &self.entries[idx]).collect()
    }

    pub fn find(&self, query: Option<&str>) -> Option<&ProjectEntry> {
        let recent = self.recent();
        if let Some(search) = query.filter(|q| !q.is_empty()) {
            let search = search.to_lowercase();
            recent.into_iter().find(|entry| entry.matches(&search))
        } else {
            recent.into_iter().next()
        }
    }

    pub fn save(&self) -> Result<()> {
        let data = serde_json::to_string_pretty(&self.entries)?;
        fs::write(&self.path, data).with_context(|| format!("writing {}", self.path.display()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectEntry {
    pub path: PathBuf,
    pub project_name: String,
    pub stack: Stack,
    pub last_used_verb: Option<Verb>,
    pub last_used_at: Option<DateTime<Utc>>,
}

impl ProjectEntry {
    fn matches(&self, needle: &str) -> bool {
        self.project_name.to_lowercase().contains(needle)
            || self.path.to_string_lossy().to_lowercase().contains(needle)
    }
}

fn config_root() -> PathBuf {
    BaseDirs::new()
        .map(|dirs| dirs.config_dir().join("axl"))
        .unwrap_or_else(|| PathBuf::from(".axl"))
}
