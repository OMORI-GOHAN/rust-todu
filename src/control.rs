// 控制程序
use crate::task::Task;
use crate::task_manager::{add, delete, change_description, change_completed, find_task};
use crate::storage::save_tasks;
use std::error::Error;
use std::io;

pub fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("输入合法内容！");
    input.trim().to_string()
}

// 查看任务
pub fn show_tasks(tasks: &[Task]) {
    for item in tasks {
        item.show();
    }
}

pub fn change_task(tasks: &mut Vec<Task>, target: &str, change_type: &str) {
    match find_task(tasks, target) {
        Some(local) => {
            match change_type {
                "1" => {
                    println!("请输入新的任务描述");
                    let new_description = user_input();
                    change_description(tasks, local, new_description);
                },
                "2" => {
                    change_completed(tasks, local);
                },
                _ => {
                    println!("请输入合法内容！");
                }
            }
        }
        None => {
            println!("找不到该任务！");
        }
    };

}


pub fn control(tasks: &mut Vec<Task>) -> Result<bool, Box<dyn Error>> {
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
            println!("输入添加的任务");
            let description = user_input();
            add(tasks, description);
            Ok(true)
        },
        "2" => {
            show_tasks(tasks);
            println!("输入你想要删除的任务的描述/ID/创建时间");
            let target = user_input();
            delete(tasks, &target);
            Ok(true)
        },
        "3" => {
            show_tasks(tasks);
            Ok(true)
        },
        "4" => {
            println!("
            选择您要执行的操作：
            1.修改任务描述
            2.修改任务完成与否
            ");
            let change_type = user_input();
            show_tasks(tasks);
            println!("输入你想要修改的任务的描述/ID/创建时间");
            let target = user_input();
            change_task(tasks, &target, &change_type);
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