// SPDX-License-Identifier: MIT
// Copyright © 2025 Venkatesh V.K.

use std::error::Error;

use crate::{domain::tasks::Task, repository::BaseRepo};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "todo", version = "0.0.1")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a task with `title` and optional `description`
    Create {
        #[arg(short, long)]
        title: String,

        #[arg(long, default_value = None)]
        description: Option<String>,
    },
    /// List all the tasks
    List,
}

impl Commands {
    /// Perform the appropriate action based on the command provided
    pub fn run(
        self,
        conn: &mut impl BaseRepo<Item = Task>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Commands::Create { title, description } => {
                let task = Task::new(title, description);
                Self::create(task, conn)
            }
            Commands::List => Self::list(conn),
        }
    }

    fn create(task: Task, conn: &mut impl BaseRepo<Item = Task>) -> Result<(), Box<dyn Error>> {
        conn.save(task);
        Self::list(conn)?;
        Ok(())
    }

    fn list(conn: &mut impl BaseRepo<Item = Task>) -> Result<(), Box<dyn Error>> {
        let list: Vec<_> = conn.list().collect();
        println!("{:?}", list);
        Ok(())
    }
}
