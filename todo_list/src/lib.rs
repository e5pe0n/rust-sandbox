mod todo;

use serde_json::Error as JSONError;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Error as IOError};
use todo::{Todo, TodoList};

fn read_todo_list(path: &str) -> Result<TodoList, Box<dyn Error>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let todos: Vec<Todo> = serde_json::from_reader(buf)?;

    Ok(TodoList::new(todos.into_iter()))
}

#[cfg(test)]
mod read_todo_list_tests {
    use super::read_todo_list;
    use crate::todo::{Status, Todo, TodoList};

    #[test]
    fn should_read_todo_list() {
        let path = "test_assets/todo-list.json";

        let res = read_todo_list(path);
        match res {
            Ok(res) => assert_eq!(
                res,
                TodoList::new(
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
                            title: String::from("title1"),
                            description: String::from("description1"),
                            status: Status::InProgress,
                            label: String::from("label1"),
                        },
                        Todo {
                            id: 2,
                            title: String::from("title2"),
                            description: String::from("description2"),
                            status: Status::Done,
                            label: String::from("label2"),
                        },
                    ]
                    .into_iter()
                ),
            ),
            x => panic!("failed to read todo list: {:?} was returned", x),
        }
    }

    use super::IOError;

    #[test]
    fn should_return_io_error() {
        let path = "";

        let res = read_todo_list(path);
        match res {
            Err(err) if err.is::<IOError>() => (),
            x => panic!("did not return io::Error; {:?} was returned", x),
        }
    }

    use super::JSONError;

    #[test]
    fn should_return_json_error() {
        let path = "test_assets/todo-list-invalid.json";

        let res = read_todo_list(path);
        match res {
            Err(err) if err.is::<JSONError>() => (),
            x => panic!("did not return serde_json::Error; {:?} was returned", x),
        }
    }
}
