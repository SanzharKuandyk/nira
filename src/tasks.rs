use crate::blueprint::{Blueprint, TaskStatus};
use std::io;
use std::path::Path;

pub fn list_tasks(bp: &Blueprint) {
    println!("Tasks for: {}\n", bp.path.display());

    // Done tasks (not numbered)
    if !bp.tasks.done.is_empty() {
        println!("✓ DONE ({}):", bp.tasks.done.len());
        for task in &bp.tasks.done {
            println!("  • {}", task.text);
        }
        println!();
    }

    // Active tasks (numbered)
    let active = bp.tasks.all_active_numbered();
    if active.is_empty() {
        println!("No active tasks. Add some with 'nira task add \"description\"'");
        return;
    }

    // Group by status
    for task in &bp.tasks.in_progress {
        let num = active.iter().find(|(_, t)| t.text == task.text).map(|(n, _)| n).unwrap();
        println!("→ IN PROGRESS #{}:", num);
        println!("  {}", task.text);
        if let Some(ctx) = &task.context {
            println!("  Context: {}", ctx);
        }
        if let Some(files) = &task.files {
            println!("  Files: {}", files);
        }
        println!();
    }

    if !bp.tasks.next_up.is_empty() {
        println!("⋯ NEXT UP:");
        for task in &bp.tasks.next_up {
            let num = active.iter().find(|(_, t)| t.text == task.text).map(|(n, _)| n).unwrap();
            println!("  {}. {}", num, task.text);
            if let Some(approach) = &task.approach {
                println!("     → {}", approach);
            }
        }
        println!();
    }

    if !bp.tasks.icebox.is_empty() {
        println!("❄ ICEBOX:");
        for task in &bp.tasks.icebox {
            let num = active.iter().find(|(_, t)| t.text == task.text).map(|(n, _)| n).unwrap();
            println!("  {}. {}", num, task.text);
        }
    }
}

pub fn add_task(path: &Path, description: &str) -> io::Result<()> {
    let content = std::fs::read_to_string(path)?;

    // Find the NEXT UP section
    let lines: Vec<&str> = content.lines().collect();
    let mut insert_line = None;

    for (idx, line) in lines.iter().enumerate() {
        if line.trim().starts_with("### NEXT UP") {
            // Find where to insert (after the heading, or before next section)
            let mut insert_idx = idx + 1;

            // Skip empty lines after heading
            while insert_idx < lines.len() && lines[insert_idx].trim().is_empty() {
                insert_idx += 1;
            }

            // Find the end of NEXT UP section (next ### heading or end of file)
            while insert_idx < lines.len() {
                let line = lines[insert_idx].trim();
                if line.starts_with("###") {
                    break;
                }
                insert_idx += 1;
            }

            insert_line = Some(insert_idx);
            break;
        }
    }

    let insert_line = insert_line.ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidData, "Could not find '### NEXT UP' section in blueprint")
    })?;

    // Build the new task entry
    let task_entry = format!(
        "\n- [ ] **{}**\n  - **Depends on:** \n  - **Files:** \n  - **Approach:** \n",
        description
    );

    // Insert the task
    let mut new_lines = lines[..insert_line].to_vec();
    new_lines.extend(task_entry.lines());
    new_lines.extend(&lines[insert_line..]);

    let new_content = new_lines.join("\n");
    std::fs::write(path, new_content)?;

    println!("✓ Added task to NEXT UP: {}", description);
    Ok(())
}

pub fn move_task(path: &Path, task_num: usize, target: TaskStatus) -> io::Result<()> {
    let content = std::fs::read_to_string(path)?;
    let bp = crate::parser::parse(&content, path.to_path_buf());

    // Find the task
    let active_tasks = bp.tasks.all_active_numbered();
    let (_, task) = active_tasks.iter()
        .find(|(num, _)| *num == task_num)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, format!("Task #{} not found", task_num)))?;

    // Find the target section
    let target_section = match target {
        TaskStatus::Done => "### DONE",
        TaskStatus::InProgress => "### IN PROGRESS",
        TaskStatus::NextUp => "### NEXT UP",
        TaskStatus::Icebox => "### ICEBOX",
    };

    let lines: Vec<&str> = content.lines().collect();

    // Find and remove the task from its current location
    let mut new_lines = Vec::new();
    let mut skip_until = None;
    let mut found_task = false;

    for (idx, line) in lines.iter().enumerate() {
        if let Some(skip_idx) = skip_until {
            if idx < skip_idx {
                continue;
            } else {
                skip_until = None;
            }
        }

        // Check if this line is our task
        if line.trim_start().starts_with("- [") && line.contains(&task.text) {
            found_task = true;
            // Skip this line and any following metadata lines
            let mut end_idx = idx + 1;
            while end_idx < lines.len() {
                let next_line = lines[end_idx].trim_start();
                if next_line.starts_with("- [") || next_line.starts_with("###") {
                    break;
                }
                if !next_line.is_empty() && next_line.starts_with("- ") {
                    end_idx += 1;
                } else if next_line.is_empty() {
                    end_idx += 1;
                } else {
                    break;
                }
            }
            skip_until = Some(end_idx);
            continue;
        }

        new_lines.push(*line);
    }

    if !found_task {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Task not found in file"));
    }

    // Find target section and insert
    let mut final_lines: Vec<String> = Vec::new();
    let mut inserted = false;

    for (idx, line) in new_lines.iter().enumerate() {
        final_lines.push(line.to_string());

        if line.trim().starts_with(target_section) && !inserted {
            // Skip empty lines
            let mut insert_idx = idx + 1;
            while insert_idx < new_lines.len() && new_lines[insert_idx].trim().is_empty() {
                final_lines.push(new_lines[insert_idx].to_string());
                insert_idx += 1;
            }

            // Format task based on target status
            let task_text = match target {
                TaskStatus::Done => {
                    format!("- [x] {}", task.text)
                }
                TaskStatus::InProgress => {
                    let mut text = format!("- [ ] **{}**", task.text);
                    text.push_str(&format!("\n  - **Context:** {}", task.context.as_deref().unwrap_or("")));
                    text.push_str(&format!("\n  - **Blocked?** no"));
                    text.push_str(&format!("\n  - **Files:** {}", task.files.as_deref().unwrap_or("")));
                    text
                }
                TaskStatus::NextUp => {
                    let mut text = format!("- [ ] **{}**", task.text);
                    text.push_str(&format!("\n  - **Depends on:** {}", task.approach.as_deref().unwrap_or("")));
                    text.push_str(&format!("\n  - **Files:** {}", task.files.as_deref().unwrap_or("")));
                    text.push_str(&format!("\n  - **Approach:** "));
                    text
                }
                TaskStatus::Icebox => {
                    format!("- [ ] {}", task.text)
                }
            };

            for task_line in task_text.lines() {
                final_lines.push(task_line.to_string());
            }
            final_lines.push(String::new());

            inserted = true;

            // Skip ahead past the lines we already added
            let skip_count = insert_idx - idx - 1;
            for _ in 0..skip_count {
                // Already added these empty lines
            }
        }
    }

    let new_content = final_lines.join("\n");
    std::fs::write(path, new_content)?;

    let status_name = match target {
        TaskStatus::Done => "DONE",
        TaskStatus::InProgress => "IN PROGRESS",
        TaskStatus::NextUp => "NEXT UP",
        TaskStatus::Icebox => "ICEBOX",
    };

    println!("✓ Moved task #{} to {}: {}", task_num, status_name, task.text);
    Ok(())
}
