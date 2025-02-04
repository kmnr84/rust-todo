use std::collections::VecDeque;
use std::error::Error;
use std::env;
use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

const FILE_NAME: &str = "tasks.txt"; // タスクを保存するファイル名

fn main() -> Result<(), Box<dyn Error>> {
    let mut tasks: VecDeque<String> = load_tasks()?;

    // コマンドライン引数の処理
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "add" => {
                if args.len() < 3 {
                    println!("Usage: rust-todo add <task>");
                    return Ok(());
                }
                let task = args[2..].join(" "); // 引数を結合してひとつのタスクにする
                // tasks.push_back(task.clone()); // loop で待機しながら組むようなときはメモリに残す
                save_task(&task)?; // タスクをファイルに保存
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
                    save_all_tasks(&tasks)?;
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
fn load_tasks() -> Result<VecDeque<String>, Box<dyn Error>> {
    let mut tasks = VecDeque::new();
    if Path::new(FILE_NAME).exists() {
        let file = File::open(FILE_NAME)?;
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            tasks.push_back(line?);
        }
    }
    Ok(tasks)
}

/// タスクをファイルに追加する
fn save_task(task: &str) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().append(true).create(true).open(FILE_NAME)?;
    writeln!(file, "{}", task)?;
    Ok(())
}

/// すべてのタスクをファイルに保存する
fn save_all_tasks(tasks: &VecDeque<String>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(FILE_NAME)?;
    for task in tasks {
        writeln!(file, "{}", task)?;
    }
    Ok(())
}
