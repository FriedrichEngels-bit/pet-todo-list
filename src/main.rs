use colored::*;

enum Status {
    NotStarted,
    InProgress,
    Completed,
}

struct Todo {
    description: String,
    status: Status,
}

impl Todo {
    fn new(description: String) -> Todo {
        Todo {
            description,
            status: Status::NotStarted,
        }
    }

    fn mark_completed(&mut self) {
        self.status = Status::Completed;
    }
    fn mark_in_progress(&mut self) {
        self.status = Status::InProgress;
    }
}

struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { todos: Vec::new() }
    }

    fn add(&mut self, description: String) {
        let todo = Todo::new(description);
        self.todos.push(todo);
    }

    fn remove(&mut self, index: usize) -> Result<(), String> {
        if index < self.todos.len() {
            self.todos.remove(index);
            Ok(())
        } else {
            Err("Неверный ввод".red().to_string())
        }
    }

    fn complete(&mut self, index: usize) -> Result<(), String> {
        if index < self.todos.len() {
            self.todos[index].mark_completed();
            Ok(())
        } else {
            Err("Неверный ввод".red().to_string())
        }
    }

    fn in_progress(&mut self, index: usize) -> Result<(), String> {
        if index < self.todos.len() {
            self.todos[index].mark_in_progress();
            Ok(())
        } else {
            Err("Неверный ввод".red().to_string())
        }
    }

    fn list(&self) {
        if self.todos.is_empty() {
            println!("{}", "На данный момент задач нет.".cyan());
        } else {
            for (i, todo) in self.todos.iter().enumerate() {
                let status = match todo.status {
                    Status::NotStarted => "Не начата".yellow(),
                    Status::InProgress => "Исполняется".yellow(),
                    Status::Completed => "Завершена".green(),
                };
                println!("{}: {} [{}]", i + 1, todo.description, status);
            }
        }
    }
}

use std::io::{self, Write};

fn main() {
    let mut todo_list = TodoList::new();

    loop {
        println!("\n1. Добавить задачу");
        println!("2. Список задач");
        println!("3. Задача к исполнению");
        println!("4. Отметить как выполненна");
        println!("5. Удалить задачу");
        println!("6. Выход");

        print!("{}", "Выберите опцию: ".green());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let choice: u32 = input.trim().parse().unwrap_or(0);

        match choice {
            1 => {
                print!("{}", "Введите наименование задачи: ".blue());
                io::stdout().flush().unwrap();
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                todo_list.add(description.trim().to_string());
                println!("{}", "Задача добавленна!".blue());
            }
            2 => {
                todo_list.list();
            }
            3 => {
                todo_list.list();
                print!("{}", "Номер задачи для отметки В процессе: ".blue());
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                let index: usize = index.trim().parse().unwrap_or(0) - 1;

                match todo_list.in_progress(index) {
                    Ok(_) => println!("{}", "Задача отмечена".blue()),
                    Err(e) => println!("{}", e),
                }
            }
            4 => {
                todo_list.list();
                print!(
                    "{}",
                    "Введите номер задачи, которую хотите отметить как выполненную: ".green()
                );
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                let index: usize = index.trim().parse().unwrap_or(0) - 1;

                match todo_list.complete(index) {
                    Ok(_) => println!("{}", "Задача выполнена!".green()),
                    Err(e) => println!("{}", e),
                }
            }
            5 => {
                todo_list.list();
                print!("{}", "Введите номер задачи, которую хотите удалить: ".red());
                io::stdout().flush().unwrap();
                let mut index = String::new();
                io::stdin().read_line(&mut index).unwrap();
                let index: usize = index.trim().parse().unwrap_or(0) - 1;

                match todo_list.remove(index) {
                    Ok(_) => println!("{}", "Задача удалена!".red()),
                    Err(e) => println!("{}", e),
                }
            }
            6 => {
                println!("{}", "Удачи вам!, счастья, здоровья и успехов!".cyan());
                break;
            }
            _ => println!(
                "{}",
                "Неверный ввод, попробуйте ещё раз.".truecolor(255, 255, 0)
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_todo_creation() {
        let todo = Todo::new("Тест создания задачи".to_string());
        assert_eq!(todo.description, "Тест создания задачи");
        match todo.status {
            Status::NotStarted => (),
            _ => panic!("Статус должен быть NotStarted"),
        }
    }

    #[test]
    fn test_mark_completed() {
        let mut todo = Todo::new("Тест задачи".to_string());
        todo.mark_completed();
        match todo.status {
            Status::Completed => (),
            _ => panic!("Статус должен быть Completed"),
        }
    }
}
