
use std::io;
use chrono::Local;


struct Task {   
    id: u64,
    description: String,
    completed: bool,
    date: String, // yyyy-mm-dd 

}


impl Task {
    fn new(id: u64, description: String, completed: bool, date: String) -> Task{
        Task { id, description, completed, date }
    }
    
    fn show(&self) {
        print!("
        任务ID：{}
        描述：{}
        完成与否：{}
        时间：{}
        ",self.id, self.description, self.completed, self.date);
    }
}

fn add(tasks:&mut Vec<Task>) {
    // 添加description
    println!("输入添加的任务");
    let mut description = String::new();
    io::stdin()
        .read_line(&mut description)
        .expect("输入合法内容！");
    let description = description
        .trim()
        .to_string();

    // 获取时间
    let now = Local::now();
    let date = String::from(now.format("%Y-%m-%d %H:%M:%S").to_string());
    let timestamp = now
        .timestamp_nanos_opt()
        .expect("无法获取纳秒级时间戳");
    
    // 添加task
    tasks.push(Task::new(timestamp as u64, description, false, date));
}

// 查看任务
fn show_tasks(tasks:&[Task]) {
    for item in tasks {
        item.show();
    }
}

fn delete(tasks:&mut Vec<Task>) {
    show_tasks(tasks);
    println!("输入你想要删除的任务的描述/ID/创建时间");
    let mut target = String::new();
    io::stdin()
        .read_line(&mut target)
        .expect("输入合法内容！");
    let target = target.trim();

    match target {
        
    }

}

fn check_type(target:&String) -> &str {
    let chars: Vec<char> = target.chars().collect();
    let string_type:&str;
    
    let have_num = false;
    let have_punctuation = false;
    let have_letter = false;
    
    for c in chars{
        if c.is_ascii_punctuation() {
            have_punctuation = true;
        }
    }
}


// 查找方法
fn search(tasks:&[Task], id:u64) -> usize {
    let mut left = 0;
    let mut right   = tasks.len() - 1;
    loop {
        if tasks[(left + right) / 2].id > id {
            left = (left + right) / 2 + 1;
        }else if tasks[(left + right) / 2].id < id {
            right = (left + right) / 2 - 1;
        }else {
            break;
        }
    }
    (left + right) / 2
}

fn control(tasks:&mut Vec<Task>) -> bool {
    print!("
    欢迎使用rust-task!
    请输入对应数组以选择模式
    1.添加任务
    2.删除任务
    3.查看任务
    
    Q.退出
    ");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("请输入合法内容！");
    let input = input.trim(); // 删去输入末尾的换行符

    match input {
        "1" => {
            add(tasks);
            true
        },
        "2" => {
            println!("删除任务");
            true
        },
        "3" => {
            show_tasks(tasks);
            true
        },

        "Q" | "q" => {
            println!("退出任务");
            false
        }

        _ => {
            println!(" 无效内容，请按照提示输入");
            true
        }
    }
}





fn main() {
    let mut tasks:Vec<Task> = Vec::new();

    loop {
        if !control(&mut tasks) {
            break;
        }
    } 

}    
