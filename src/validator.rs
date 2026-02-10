use crate::blueprint::{Blueprint, ValidationResult, ValidationStatus};

pub fn validate(bp: &Blueprint) -> Vec<ValidationResult> {
    let mut results = Vec::new();

    // Layer 1: Intent Map
    results.push(if bp.has_intent {
        ValidationResult {
            layer: 1,
            layer_name: "Intent Map".to_string(),
            status: ValidationStatus::Ok,
            message: "Has meaningful content".to_string(),
        }
    } else {
        ValidationResult {
            layer: 1,
            layer_name: "Intent Map".to_string(),
            status: ValidationStatus::Missing,
            message: "Missing or incomplete - fill in PROJECT, ACTORS, CORE FLOWS, HARD PARTS".to_string(),
        }
    });

    // Layer 2: Interface Contracts
    results.push(if bp.has_contracts {
        ValidationResult {
            layer: 2,
            layer_name: "Interface Contracts".to_string(),
            status: ValidationStatus::Ok,
            message: "Has interface definitions".to_string(),
        }
    } else {
        ValidationResult {
            layer: 2,
            layer_name: "Interface Contracts".to_string(),
            status: ValidationStatus::Missing,
            message: "Missing or incomplete - define your data shapes, capabilities, and boundaries".to_string(),
        }
    });

    // Layer 3: File Skeleton
    results.push(if bp.has_skeleton {
        ValidationResult {
            layer: 3,
            layer_name: "File Skeleton".to_string(),
            status: ValidationStatus::Ok,
            message: "Has file structure defined".to_string(),
        }
    } else {
        ValidationResult {
            layer: 3,
            layer_name: "File Skeleton".to_string(),
            status: ValidationStatus::Missing,
            message: "Missing or incomplete - map your interfaces to files on disk".to_string(),
        }
    });

    // Layer 4: Task Queue
    let active_tasks = bp.tasks.in_progress.len() + bp.tasks.next_up.len();

    results.push(if active_tasks > 0 {
        ValidationResult {
            layer: 4,
            layer_name: "Task Queue".to_string(),
            status: ValidationStatus::Ok,
            message: format!("{} active tasks", active_tasks),
        }
    } else if bp.tasks.done.len() > 0 || bp.tasks.icebox.len() > 0 {
        ValidationResult {
            layer: 4,
            layer_name: "Task Queue".to_string(),
            status: ValidationStatus::Warning,
            message: "No active tasks - move something to IN PROGRESS or NEXT UP".to_string(),
        }
    } else {
        ValidationResult {
            layer: 4,
            layer_name: "Task Queue".to_string(),
            status: ValidationStatus::Missing,
            message: "No tasks defined - add tasks to guide implementation".to_string(),
        }
    });

    // Extra warnings for task quality
    for task in &bp.tasks.in_progress {
        if task.context.is_none() {
            results.push(ValidationResult {
                layer: 4,
                layer_name: "Task Quality".to_string(),
                status: ValidationStatus::Warning,
                message: format!("IN PROGRESS task '{}' missing Context", task.text),
            });
        }
        if task.files.is_none() {
            results.push(ValidationResult {
                layer: 4,
                layer_name: "Task Quality".to_string(),
                status: ValidationStatus::Warning,
                message: format!("IN PROGRESS task '{}' missing Files", task.text),
            });
        }
    }

    for task in &bp.tasks.next_up {
        if task.approach.is_none() {
            results.push(ValidationResult {
                layer: 4,
                layer_name: "Task Quality".to_string(),
                status: ValidationStatus::Warning,
                message: format!("NEXT UP task '{}' missing Approach", task.text),
            });
        }
    }

    results
}
