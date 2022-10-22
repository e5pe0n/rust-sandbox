use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::ToDo => String::from("ToDo"),
                Self::InProgress => String::from("InProgress"),
                Self::Done => String::from("Done"),
            }
        )
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PartialTodo {
    pub title: String,
    pub description: String,
    pub label: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub label: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TodoList {
    list: Vec<Todo>,
}

impl TodoList {
    pub fn new(iter: impl Iterator<Item = Todo>) -> Self {
        TodoList {
            list: Vec::from_iter(iter),
        }
    }

    pub fn load(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let buf = BufReader::new(file);
        let todos: Vec<Todo> = serde_json::from_reader(buf)?;

        Ok(Self::new(todos.into_iter()))
    }

    fn get_next_id(&self) -> usize {
        let ids = self.list.iter().map(|x| x.id).collect::<Vec<_>>();
        (0..(self.list.len() + 1))
            .filter(|x| !ids.contains(x))
            .min()
            .unwrap()
    }

    pub fn add(&mut self, todo: &PartialTodo) {
        self.list.push(Todo {
            id: self.get_next_id(),
            title: todo.title.clone(),
            description: todo.description.clone(),
            status: Status::ToDo,
            label: todo.label.clone(),
        });
    }

    pub fn list(&self) -> &Vec<Todo> {
        &self.list
    }

    pub fn save(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let file = File::open(file_path)?;
        let buf = BufWriter::new(file);
        let json = serde_json::to_string(&self)?;
        serde_json::to_writer(buf, &json)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use serde_json::Error as JSONError;
    use std::io::Error as IOError;

    fn create_partial_todo0() -> PartialTodo {
        PartialTodo {
            title: String::from("title0"),
            description: String::from("description0"),
            label: String::from(""),
        }
    }

    mod load_tests {
        use super::*;

        #[rstest]
        #[case(
            "test_assets/todo-list.json",
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
        )]
        fn should_load_todo_list(#[case] file_path: &str, #[case] expected: TodoList) {
            let res = TodoList::load(file_path);
            assert!(res.is_ok(), "not Ok; res={:?}", res);

            assert_eq!(res.unwrap(), expected);
        }

        #[test]
        fn should_return_io_error() {
            let path = "";

            let res = TodoList::load(path);
            match res {
                Err(err) if err.is::<IOError>() => (),
                x => panic!("did not return io::Error; {:?} was returned", x),
            }
        }

        #[test]
        fn should_return_json_error() {
            let path = "test_assets/todo-list-invalid.json";

            let res = TodoList::load(path);
            assert!(res.is_err(), "not err; res={:?}", res);

            let err = res.unwrap_err();
            assert!(err.is::<JSONError>(), "not expected err; res={:?}", err);
        }
    }

    mod add_tests {
        use super::create_partial_todo0;
        use super::*;

        #[test]
        fn should_add_new_todo() {
            let mut todo_list = TodoList::new(Vec::new().into_iter());
            let todo = create_partial_todo0();

            todo_list.add(&todo);

            assert_eq!(
                todo_list.list,
                vec![Todo {
                    id: 0,
                    title: todo.title,
                    description: todo.description,
                    status: Status::ToDo,
                    label: todo.label,
                }]
            );
        }
    }

    mod get_id_tests {
        use super::create_partial_todo0;
        use crate::TodoList;

        #[test]
        fn should_return_expected_id() {
            let mut todo_list = TodoList::new(Vec::new().into_iter());
            let todo = create_partial_todo0();

            assert_eq!(todo_list.get_next_id(), 0);

            todo_list.add(&todo);

            assert_eq!(todo_list.get_next_id(), 1);
        }
    }
}
