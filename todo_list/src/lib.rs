mod subcmd;
mod test_utils;
mod todo;

use serde_json::Error as JSONError;
use std::cmp::max;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Error as IOError};
use todo::{Todo, TodoList};

fn elide(s: &str, width: usize) -> String {
    debug_assert!(width > 3);

    if s.chars().count() > width {
        s.chars().take(width - 3).collect::<String>() + "..."
    } else {
        s.to_string()
    }
}

fn display(todo_list: &TodoList) {
    let id_width = 5;
    let title_width = 20;
    let description_width = 20;
    let status_width = 10;
    let label_width = 20;

    let headers = format!(
        "{:>id_width$} {:<title_width$} {:<description_width$} {:<status_width$} {:<label_width$}",
        "id", "title", "description", "status", "label"
    );
    let underline = "-".repeat(80);

    let fmt_todo = |todo: &Todo| -> String {
        debug_assert!(
            id_width + title_width + description_width + status_width + label_width <= 76 + 4
        );

        format!(
        "{:>id_width$} {:<title_width$} {:<description_width$} {:<status_width$} {:<label_width$}",
        todo.id,
        elide(&todo.title, title_width),
        elide(&todo.description, description_width),
        elide(&todo.status.to_string(), status_width),
        elide(&todo.label, label_width)
    )
    };

    println!(
        "{}\n{}\n{}",
        headers,
        underline,
        todo_list
            .list()
            .iter()
            .map(fmt_todo)
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[cfg(test)]
mod display_tests {
    use super::display;
    use crate::todo::{Status, Todo, TodoList};

    #[test]
    fn should_display_todo_list_properly() {
        let todo_list = TodoList::new(
            vec![
                Todo {
                    id: 0,
                    title: String::from("title0"),
                    description: String::from("description0"),
                    status: Status::ToDo,
                    label: String::from(""),
                },
                Todo {
                    id: 1,
                    title: String::from("title1").repeat(10),
                    description: String::from("description1").repeat(10),
                    status: Status::InProgress,
                    label: String::from("label1").repeat(10),
                },
                Todo {
                    id: 2,
                    title: String::from("title2"),
                    description: String::from("description2"),
                    status: Status::Done,
                    label: String::from("label2"),
                },
            ]
            .into_iter(),
        );
        display(&todo_list);
    }
}
