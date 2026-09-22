// 控制程序
use crate::task::Task;
use crate::task_manager::{add, delete, change_task};
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
            delete(tasks, target);
            Ok(true)
        },
        "3" => {
            show_tasks(tasks);
            Ok(true)
        },
        "4" => {
            show_tasks(tasks);
            println!("输入你想要修改的任务的描述/ID/创建时间");
            let target = user_input();
            change_task(tasks, target);
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