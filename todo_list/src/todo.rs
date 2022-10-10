use serde::Deserialize;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PartialTodo {
    pub title: String,
    pub description: String,
    pub label: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub status: Status,
    pub label: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct TodoList {
    list: Vec<Todo>,
}

impl TodoList {
    pub fn new(iter: impl Iterator<Item = Todo>) -> Self {
        TodoList {
            list: Vec::from_iter(iter),
        }
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

    fn list(&self) -> &Vec<Todo> {
        &self.list
    }
}

#[cfg(test)]
mod TodoList_tests {
    use super::PartialTodo;

    fn create_partial_todo0() -> PartialTodo {
        PartialTodo {
            title: String::from("title0"),
            description: String::from("description0"),
            label: String::from(""),
        }
    }

    mod add_tests {
        use super::super::{Status, Todo, TodoList};
        use super::create_partial_todo0;

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
