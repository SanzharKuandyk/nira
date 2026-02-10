use crate::blueprint::Blueprint;

pub fn generate(bp: &Blueprint) -> String {
    let mut output = String::new();

    // Header
    output.push_str("You are implementing code for this project. Follow these rules:\n\n");

    // The blueprint itself, wrapped in XML tags
    output.push_str("<blueprint>\n");
    output.push_str(&bp.raw);
    output.push_str("\n</blueprint>\n\n");

    // AI agent rules
    output.push_str("<rules>\n");
    output.push_str("1. Read the Blueprint above carefully before writing any code.\n");
    output.push_str("2. Follow the interface contracts exactly — types, method signatures, rules.\n");
    output.push_str("3. Place files according to the File Skeleton. Do not invent new directories.\n");
    output.push_str("4. If you need a new type, tell me — I'll add it to Layer 2 first.\n");
    output.push_str("5. If you need to change an interface, STOP and explain why before changing it.\n");
    output.push_str("6. When you finish a task, tell me:\n");
    output.push_str("   - What files you created/modified\n");
    output.push_str("   - Any new types or interfaces you introduced\n");
    output.push_str("   - What should be updated in the Blueprint\n");
    output.push_str("7. Work on ONE task from the Task Queue at a time.\n");
    output.push_str("</rules>\n");

    output
}

pub fn generate_for_task(bp: &Blueprint, task_num: usize) -> Option<String> {
    let active_tasks = bp.tasks.all_active_numbered();
    let (_, task) = active_tasks.iter().find(|(num, _)| *num == task_num)?;

    let mut output = String::new();

    // Header
    output.push_str("You are implementing a specific task for this project.\n\n");

    // The blueprint
    output.push_str("<blueprint>\n");
    output.push_str(&bp.raw);
    output.push_str("\n</blueprint>\n\n");

    // The current task
    output.push_str("<current_task>\n");
    output.push_str(&format!("Task #{}: {}\n\n", task_num, task.text));

    if let Some(context) = &task.context {
        output.push_str(&format!("Context: {}\n\n", context));
    }

    if let Some(files) = &task.files {
        output.push_str(&format!("Files: {}\n\n", files));
    }

    if let Some(approach) = &task.approach {
        output.push_str(&format!("Approach: {}\n\n", approach));
    }

    output.push_str("</current_task>\n\n");

    // AI agent rules (focused on this task)
    output.push_str("<rules>\n");
    output.push_str("1. Read the Blueprint and understand the full context.\n");
    output.push_str("2. Focus ONLY on the current task specified above.\n");
    output.push_str("3. Follow the interface contracts from Layer 2 exactly.\n");
    output.push_str("4. Place files according to Layer 3 (File Skeleton).\n");
    output.push_str("5. If you need to change an interface, STOP and explain why.\n");
    output.push_str("6. When done, report:\n");
    output.push_str("   - What files you created/modified\n");
    output.push_str("   - Any new types or interfaces\n");
    output.push_str("   - What to update in the Blueprint\n");
    output.push_str("</rules>\n");

    Some(output)
}
