use std::collections::VecDeque;
use std::error::Error;
use std::env;
use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数の処理
    let args: Vec<String> = env::args().collect();
    let file_name: &str = "tasks.txt"; // デフォルトのファイル名

    execute_command(&args, file_name)?;
    Ok(())
}

fn execute_command(args: &[String], file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut tasks: VecDeque<String> = load_tasks(file_name)?;

    if args.len() > 1 {
        match args[1].as_str() {
            "add" => {
                if args.len() < 3 {
                    println!("Usage: rust-todo add <task>");
                    return Ok(());
                }
                let task = args[2..].join(" "); // 引数を結合してひとつのタスクにする
                // tasks.push_back(task.clone()); // loop で待機しながら組むようなときはメモリに残す
                save_task(&task,file_name)?; // タスクをファイルに保存
                println!("Task added: {}", task);
            }
            "list" => {
                println!("Tasks:");
                for (i, task) in tasks.iter().enumerate() {
                    println!("{}: {}", i + 1, task);
                }
            }
            "remove" => {
                if args.len() < 3 {
                    println!("Usage: rust-todo remove <task_number>");
                    return Ok(());
                }
                let task_num: usize = args[2].parse().unwrap_or(0);
                if task_num > 0 && task_num <= tasks.len() {
                    tasks.remove(task_num - 1);
                    save_all_tasks(&tasks, file_name)?;
                    println!("Task {} removed.", task_num);
                } else {
                    println!("Invalid task number.");
                }
            }
            _ => {
                println!("Invalid command. Use 'add', 'list', or 'remove'.");
            }
        }
    } else {
        println!("Usage: rust-todo <command> [options]");
    }

    Ok(())
}

/// タスクをファイルから読み込んで VecDeque にする
fn load_tasks(file_name: &str) -> Result<VecDeque<String>, Box<dyn Error>> {
    let mut tasks = VecDeque::new();
    if Path::new(file_name).exists() {
        let file = File::open(file_name)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            tasks.push_back(line?);
        }
    }
    Ok(tasks)
}

/// タスクをファイルに追加する
fn save_task(task: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).create(true).open(file_name)?;
    writeln!(file, "{}", task)?;
    Ok(())
}

/// すべてのタスクをファイルに保存する
fn save_all_tasks(tasks: &VecDeque<String>, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(file_name)?;
    for task in tasks {
        writeln!(file, "{}", task)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup(file_name: &str) {
        let _ = fs::remove_file(file_name); // テスト用ファイルを削除
    }

    fn teardown(file_name: &str) {
        let _ = fs::remove_file(file_name); // テスト用ファイルを削除
    }

    #[test]
    fn test_execute_command_add_task() {
        let test_file = "test_execute_command_add_task.txt"; // テスト用ファイル名
        setup(test_file);

        let args = vec![
            "rust-todo".to_string(),
            "add".to_string(),
            "テストタスク".to_string(),
        ];
        assert!(execute_command(&args, test_file).is_ok());

        let tasks = load_tasks(test_file).unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0], "テストタスク");

        teardown(test_file);
    }

    #[test]
    fn test_execute_command_list() {
        let test_file = "test_execute_command_list.txt"; // テスト用ファイル名
        setup(test_file);

        let args = vec!["rust-todo".to_string(), "list".to_string()];
        assert!(execute_command(&args, test_file).is_ok());

        let tasks = load_tasks(test_file).unwrap();
        assert!(tasks.is_empty()); // 空であるべき

        teardown(test_file);
    }

    #[test]
    fn test_execute_command_remove_tesk() {
        let test_file = "test_execute_command_remove_task.txt";
        setup(test_file);

        let add_args_1 = vec!["rust-todo".to_string(), "add".to_string(), "test1".to_string()];
        assert!(execute_command(&add_args_1, test_file).is_ok());
        let add_args_2 = vec!["rust-todo".to_string(), "add".to_string(), "test2".to_string()];
        assert!(execute_command(&add_args_2, test_file).is_ok());
        let add_args_3 = vec!["rust-todo".to_string(), "add".to_string(), "test3".to_string()];
        assert!(execute_command(&add_args_3, test_file).is_ok());

        let args = vec!["rust-todo".to_string(), "remove".to_string(), "2".to_string()];
        assert!(execute_command(&args, test_file).is_ok());

        let tasks = load_tasks(test_file).unwrap();
        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0], "test1");
        assert_eq!(tasks[1], "test3");

        teardown(test_file);
    }
}
