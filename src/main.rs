// SPDX-License-Identifier: MIT
// Copyright © 2025 Venkatesh V.K.

use clap::Parser;
use cli::Cli;
use domain::tasks::Task;
use repository::{BaseRepo, vec_db::VecDB};

mod cli;
mod domain;
mod repository;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut db: VecDB<Task> = VecDB::new(None);
    add_init_task(&mut db);
    let cli = Cli::parse();

    if let Some(cmd) = cli.command {
        let _ = cmd
            .run(&mut db)
            .map_err(|op| println!("{:?}", op.to_string()));
    }

    Ok(())
}

fn add_init_task(conn: &mut impl BaseRepo<Item = Task>) -> () {
    let task = Task::new("title1".into(), Some("description".into()));
    conn.save(task);
    let task = Task::new("title2".into(), None);
    conn.save(task);
}
