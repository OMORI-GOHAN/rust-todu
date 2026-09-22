use chrono::Local;

use crate::task::Task;
use crate::control::user_input;


fn check_type(target: &str) -> &str {
    let chars: Vec<char> = target.chars().collect();
    let string_type:&str;
    
    let mut have_num: bool = false;
    let mut have_punctuation: bool = false;
    let mut have_alphabetic: bool = false;
    
    for c in chars{
        if c.is_ascii_punctuation() {
            have_punctuation = true;
        }else if c.is_ascii_digit() {
            have_num = true;
        }else if c.is_ascii_alphabetic() {
            have_alphabetic = true
        }
    }

    if have_alphabetic{
        string_type = "description";
    }else if have_punctuation {
        string_type = "date";
    }else if have_num {
        string_type = "id";
    }else{
        string_type = "illegal";
    }

    string_type
}


// 查找方法(id) by 二分查找
fn search_by_id(tasks: &[Task], id: u64) -> Option<usize> {
    let mut left = 0;
    let mut right   = tasks.len();
    loop {
        if left >= right {
            return None
        }
        if tasks[(left + right) / 2].id > id {
            right = (left + right) / 2;
        }else if tasks[(left + right) / 2].id < id {
            left = (left + right) / 2 + 1;
        }else {
            break;
        }
    }
    Some((left + right) / 2)
}
// 查找方法(date)
fn search_by_date(tasks: &[Task], date: &str) -> Option<usize> {

    for (index, task) in tasks.iter().enumerate(){
        if task.date == date {
            return Some(index);
        }
    }
    None
}

// 查找方法(description)
fn search_by_description(tasks: &[Task], description: &str) -> Option<usize> {

    for (index, task) in tasks.iter().enumerate() {
        if task.description == description {
            return  Some(index);
        }
    }
    None
}

pub fn find_task(tasks: &[Task], target: &str) -> Option<usize> {
    let local = match check_type(target) {
        "id" => {
            let id = target
                .parse::<u64>()
                .expect("id必须为数字");
            search_by_id(tasks, id)?
        }
        "date" => {
            search_by_date(tasks, target)?
        }
        "description" => {
            search_by_description(tasks, target)?
        }
        _ => {
            println!("无效的查找类型");
            return None;
        }
    };

    Some(local)
}

// 删除任务
pub fn delete(tasks: &mut Vec<Task>, target: String) {
    match find_task(tasks, target.as_str()) {
        Some(local) => {
            tasks.remove(local);
            println!("已删除!");
        }
        None => {
            println!("找不到该任务！");
        }
    }; 
}


pub fn change_task(tasks: &mut Vec<Task>, target: String) {
    match find_task(tasks, target.as_str()) {
        Some(local) => {

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
                    println!("已修改!");
                },
                "2" => {
                    tasks[local].completed = !tasks[local].completed;
                    println!("已修改!");
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


pub fn add(tasks: &mut Vec<Task>, description: String) {

    // 获取时间
    let now = Local::now();
    let date = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let timestamp = now
        .timestamp_nanos_opt()
        .expect("无法获取纳秒级时间戳");

    // 添加task
    tasks.push(Task::new(timestamp as u64, description, false, date));
}
