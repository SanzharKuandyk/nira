use crate::blueprint::{Blueprint, TaskItem, TaskQueue, TaskStatus};
use regex::Regex;
use std::path::PathBuf;

pub fn parse(content: &str, path: PathBuf) -> Blueprint {
    // Extract project name from "# Blueprint: ProjectName"
    let project_name = extract_project_name(content);

    // Check each layer for real content
    let has_intent = check_layer(content, "Layer 1: Intent Map");
    let has_contracts = check_layer(content, "Layer 2: Interface Contracts");
    let has_skeleton = check_layer(content, "Layer 3: File Skeleton");

    // Parse tasks from Layer 4
    let tasks = parse_tasks(content);

    Blueprint {
        raw: content.to_string(),
        path,
        has_intent,
        has_contracts,
        has_skeleton,
        tasks,
        project_name,
    }
}

/// Extract project name from "# Blueprint: ProjectName" heading
fn extract_project_name(content: &str) -> Option<String> {
    let re = Regex::new(r"(?i)^#\s+Blueprint:\s+(.+)$").unwrap();
    for line in content.lines() {
        if let Some(caps) = re.captures(line) {
            return Some(caps[1].trim().to_string());
        }
    }
    None
}

/// Check if a layer has real content (not just placeholders)
fn check_layer(content: &str, layer_heading: &str) -> bool {
    if let Some((start, end)) = find_section(content, layer_heading) {
        let section = &content[start..end];
        return section_has_content(section);
    }
    false
}

/// Find byte range of a section by heading (case-insensitive substring match)
/// Returns (start_of_content, end_of_content) excluding the heading line itself
pub fn find_section(content: &str, heading: &str) -> Option<(usize, usize)> {
    let heading_lower = heading.to_lowercase();

    // Find all line boundaries in the original string
    let mut line_starts: Vec<usize> = vec![0];
    for (i, _) in content.match_indices('\n') {
        line_starts.push(i + 1);
    }
    line_starts.push(content.len()); // End of file

    let lines: Vec<&str> = content.lines().collect();

    // Find the heading line
    let mut heading_line_idx = None;
    let mut heading_level = 0;

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.to_lowercase().contains(&heading_lower) && trimmed.starts_with('#') {
            // Count # symbols to determine level
            heading_level = trimmed.chars().take_while(|c| *c == '#').count();
            heading_line_idx = Some(idx);
            break;
        }
    }

    let heading_line_idx = heading_line_idx?;

    // Find the end: next heading of same or higher level
    let mut end_line_idx = lines.len();
    for idx in (heading_line_idx + 1)..lines.len() {
        let trimmed = lines[idx].trim();
        if trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|c| *c == '#').count();
            if level <= heading_level {
                end_line_idx = idx;
                break;
            }
        }
    }

    // Get byte offsets from line_starts
    let start_byte = line_starts[heading_line_idx + 1];
    let end_byte = if end_line_idx < line_starts.len() - 1 {
        line_starts[end_line_idx]
    } else {
        content.len()
    };

    Some((start_byte, end_byte))
}

/// Check if a section has real content (not just placeholders or empty)
fn section_has_content(section: &str) -> bool {
    let placeholders = vec![
        "[name]",
        "[what it does]",
        "[actor",
        "[action]",
        "[hard part",
        "[non-goal",
        "[field",
        "[method",
        "replace with your diagram",
        "TODO",
    ];

    let mut has_meaningful_content = false;

    for line in section.lines() {
        let trimmed = line.trim();

        // Skip empty lines, comments, table headers/separators
        if trimmed.is_empty()
            || trimmed.starts_with("<!--")
            || trimmed.starts_with("-->")
            || trimmed.starts_with('|')
            || trimmed.starts_with('-')
                && trimmed
                    .chars()
                    .all(|c| c == '-' || c == ' ' || c == '|')
        {
            continue;
        }

        // Check if line contains only placeholders
        let is_placeholder = placeholders
            .iter()
            .any(|p| trimmed.to_lowercase().contains(&p.to_lowercase()));

        if !is_placeholder && !trimmed.is_empty() {
            has_meaningful_content = true;
            break;
        }
    }

    has_meaningful_content
}

/// Parse all tasks from Layer 4
fn parse_tasks(content: &str) -> TaskQueue {
    let mut done = Vec::new();
    let mut in_progress = Vec::new();
    let mut next_up = Vec::new();
    let mut icebox = Vec::new();

    // Find each task section
    if let Some((start, end)) = find_section(content, "### DONE") {
        done = parse_task_list(&content[start..end], TaskStatus::Done);
    }

    if let Some((start, end)) = find_section(content, "### IN PROGRESS") {
        in_progress = parse_task_list(&content[start..end], TaskStatus::InProgress);
    }

    if let Some((start, end)) = find_section(content, "### NEXT UP") {
        next_up = parse_task_list(&content[start..end], TaskStatus::NextUp);
    }

    if let Some((start, end)) = find_section(content, "### ICEBOX") {
        icebox = parse_task_list(&content[start..end], TaskStatus::Icebox);
    }

    TaskQueue {
        done,
        in_progress,
        next_up,
        icebox,
    }
}

/// Parse a list of tasks from a section
fn parse_task_list(section: &str, status: TaskStatus) -> Vec<TaskItem> {
    let mut tasks = Vec::new();
    let task_re = Regex::new(r"^- \[([ x])\]\s+(.+)$").unwrap();

    let lines: Vec<&str> = section.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim_start();

        if let Some(caps) = task_re.captures(line) {
            let text = caps[2].trim();
            // Remove bold markers if present
            let text = text
                .trim_start_matches("**")
                .trim_end_matches("**")
                .to_string();

            // Look ahead for metadata
            let mut context = None;
            let mut files = None;
            let mut approach = None;
            let mut j = i + 1;

            while j < lines.len() {
                let metadata_line = lines[j].trim_start();

                // Stop if we hit another task or empty line followed by task
                if metadata_line.starts_with("- [") {
                    break;
                }

                // Parse metadata fields
                if let Some(rest) = metadata_line.strip_prefix("- **Context:**") {
                    context = Some(rest.trim().to_string());
                } else if let Some(rest) = metadata_line.strip_prefix("- **Files:**") {
                    files = Some(rest.trim().to_string());
                } else if let Some(rest) = metadata_line.strip_prefix("- **Approach:**") {
                    approach = Some(rest.trim().to_string());
                } else if let Some(rest) = metadata_line.strip_prefix("- **Depends on:**") {
                    // Store in approach field for next_up tasks
                    if approach.is_none() {
                        approach = Some(format!("Depends on: {}", rest.trim()));
                    }
                } else if metadata_line.is_empty() {
                    // Empty line might signal end of metadata
                    if j + 1 < lines.len() && lines[j + 1].trim_start().starts_with("- [") {
                        break;
                    }
                }

                j += 1;
            }

            tasks.push(TaskItem {
                text,
                status,
                context,
                files,
                approach,
                line_number: i + 1, // 1-indexed
            });

            i = j;
        } else {
            i += 1;
        }
    }

    tasks
}
