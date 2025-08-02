// SPDX-License-Identifier: MIT
// Copyright © 2025 Venkatesh V.K.

#![allow(dead_code)]

/// Represents our `task` in the application
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Task {
    /// Title of our task
    title: String,
    /// Detial description of our task which is optional
    description: Option<String>,
}

impl Task {
    /// Constructor method for creating new Task
    pub fn new(title: String, description: Option<String>) -> Self {
        Self { title, description }
    }

    /// Getter method for `title`
    pub fn title(&self) -> String {
        self.title.clone()
    }

    /// Getter method for `description`
    pub fn description(&self) -> Option<String> {
        self.description.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_is_created_successfully() {
        let name = "todo".to_string();
        let description = Some("this is task description".to_string());
        let task = Task {
            title: name.clone(),
            description: description.clone(),
        };

        let dup_stask: Task = Task::new(name, description);

        assert_eq!(task, dup_stask);
    }
}
