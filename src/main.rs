use std::error::Error;
use std::io;
use chrono::Local;
use task::Task;
use storage::{save_tasks, read_tasks};
use task_manager::find_task;

mod task;
mod storage;
mod task_manager;

fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("输入合法内容！");
    input.trim().to_string()
}

fn add(tasks: &mut Vec<Task>) {
    // 添加description
    println!("输入添加的任务");
    let description = user_input();

    // 获取时间
    let now = Local::now();
    let date = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let timestamp = now
        .timestamp_nanos_opt()
        .expect("无法获取纳秒级时间戳");

    // 添加task
    tasks.push(Task::new(timestamp as u64, description, false, date));
}

// 查看任务
fn show_tasks(tasks: &[Task]) {
    for item in tasks {
        item.show();
    }
}

// 删除任务
fn delete(tasks: &mut Vec<Task>) {
    show_tasks(tasks);
    println!("输入你想要删除的任务的描述/ID/创建时间");
    let local = find_task(tasks, user_input().as_str()).expect("找不到该任务");
    tasks.remove(local);
    println!("已删除!");
}


fn change_task(tasks: &mut Vec<Task>) {
    show_tasks(tasks);
    println!("输入你想要修改的任务的描述/ID/创建时间");
    let local = find_task(tasks, user_input().as_str()).expect("找不到该任务");

    tasks[local].show();

    println!("
    选择您要执行的操作：
    1.修改任务描述
    2.修改任务完成与否
    ");

    let input = user_input();

    match input.as_str() {
        "1" => {
            println!("请输入新的任务描述");
            let new_description = user_input();
            tasks[local].description = new_description;
        },
        "2" => {
            tasks[local].completed = !tasks[local].completed;
        },
        _ => {
            println!("请输入合法内容！");
        }
    }

    println!("已修改!");
}

// 控制程序
fn control(tasks: &mut Vec<Task>) -> Result<bool, Box<dyn Error>> {
    print!("
    欢迎使用rust-task!
    请输入对应数字以选择模式
    1.添加任务
    2.删除任务
    3.查看任务
    4.修改任务

    Q.退出
    ");

    let input = user_input();

    match input.as_str() {
        "1" => {
            add(tasks);
            Ok(true)
        },
        "2" => {
            delete(tasks);
            Ok(true)
        },
        "3" => {
            show_tasks(tasks);
            Ok(true)
        },
        "4" => {
            change_task(tasks);
            Ok(true)
        },
        "Q" | "q" => {
            println!("退出任务");
            save_tasks(tasks)?;
            Ok(false)
        },
        _ => {
            println!(" 无效内容，请按照提示输入");
            Ok(true)
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut tasks: Vec<Task> = Vec::new();

    println!("开始读取任务");
    read_tasks(&mut tasks)?;
    println!("读取任务完成");

    loop {
        if !control(&mut tasks)? {
            break;
        }
    }

    Ok(())
}