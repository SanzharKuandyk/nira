use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub raw: String,
    pub path: PathBuf,
    pub has_intent: bool,
    pub has_contracts: bool,
    pub has_skeleton: bool,
    pub tasks: TaskQueue,
    pub project_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskItem {
    pub text: String,
    pub status: TaskStatus,
    pub context: Option<String>,
    pub files: Option<String>,
    pub approach: Option<String>,
    pub line_number: usize,
}

#[derive(Debug, Clone)]
pub struct TaskQueue {
    pub done: Vec<TaskItem>,
    pub in_progress: Vec<TaskItem>,
    pub next_up: Vec<TaskItem>,
    pub icebox: Vec<TaskItem>,
}

impl TaskQueue {
    /// Returns in_progress + next_up + icebox with sequential numbering starting at 1
    /// (done tasks are not numbered)
    pub fn all_active_numbered(&self) -> Vec<(usize, &TaskItem)> {
        let mut result = Vec::new();
        let mut num = 1;

        for task in &self.in_progress {
            result.push((num, task));
            num += 1;
        }
        for task in &self.next_up {
            result.push((num, task));
            num += 1;
        }
        for task in &self.icebox {
            result.push((num, task));
            num += 1;
        }

        result
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Done,
    InProgress,
    NextUp,
    Icebox,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Done => write!(f, "✓"),
            TaskStatus::InProgress => write!(f, "→"),
            TaskStatus::NextUp => write!(f, "⋯"),
            TaskStatus::Icebox => write!(f, "❄"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub layer: u8,
    pub layer_name: String,
    pub status: ValidationStatus,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationStatus {
    Ok,
    Warning,
    Missing,
}

impl std::fmt::Display for ValidationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationStatus::Ok => write!(f, "✓"),
            ValidationStatus::Warning => write!(f, "⚠"),
            ValidationStatus::Missing => write!(f, "✗"),
        }
    }
}
