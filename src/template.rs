use std::fs;
use std::path::PathBuf;

/// Find the templates directory. Check multiple locations:
/// 1. <exe_dir>/templates/ (relative to nira.exe - primary location)
/// 2. ./templates/ (relative to CWD - for development convenience)
fn find_templates_dir() -> Option<PathBuf> {
    // Try relative to executable first (primary location)
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_templates = exe_dir.join("templates");
            if exe_templates.exists() && exe_templates.is_dir() {
                return Some(exe_templates);
            }
        }
    }

    // Fallback: try current directory (useful during development)
    let cwd_templates = PathBuf::from("templates");
    if cwd_templates.exists() && cwd_templates.is_dir() {
        return Some(cwd_templates);
    }

    None
}

/// Load a template by name from the templates/ directory
pub fn get_template(name: &str) -> Option<String> {
    let templates_dir = find_templates_dir()?;
    let template_file = templates_dir.join(format!("{}.md", name));

    fs::read_to_string(template_file).ok()
}

/// List all available templates by reading .md files from templates/
pub fn list_templates() -> Vec<(String, String)> {
    let Some(templates_dir) = find_templates_dir() else {
        return vec![];
    };

    let Ok(entries) = fs::read_dir(&templates_dir) else {
        return vec![];
    };

    let mut templates = vec![];

    for entry in entries.flatten() {
        let path = entry.path();

        // Only process .md files
        if path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }

        // Get template name from filename
        let Some(name) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };

        // Read file to extract description from first line comment
        let description = if let Ok(content) = fs::read_to_string(&path) {
            extract_description(&content)
        } else {
            format!("Template: {}", name)
        };

        templates.push((name.to_string(), description));
    }

    // Sort by name for consistent ordering
    templates.sort_by(|a, b| a.0.cmp(&b.0));

    templates
}

/// Extract description from template file
/// Looks for: <!-- Description: text here -->
fn extract_description(content: &str) -> String {
    for line in content.lines().take(10) {
        let trimmed = line.trim();
        if trimmed.starts_with("<!-- Description:") {
            if let Some(desc_start) = trimmed.strip_prefix("<!-- Description:") {
                if let Some(desc) = desc_start.strip_suffix("-->") {
                    return desc.trim().to_string();
                }
            }
        }
    }

    // Default description if not found
    "Custom template".to_string()
}
