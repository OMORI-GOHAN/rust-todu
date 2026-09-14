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

fn add(tasks: &mut Vec<Task>) {
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
fn show_tasks(tasks: &[Task]) {
    for item in tasks {
        item.show();
    }
}

// 删除任务
fn delete(tasks: &mut Vec<Task>) {
    show_tasks(tasks);
    println!("输入你想要删除的任务的描述/ID/创建时间");
    let local = find_task(tasks).expect("找不到该任务");


    tasks.remove(local);
    println!("已删除!");
}

fn find_task(tasks: &[Task]) -> Option<usize> {
    let mut target = String::new();
    io::stdin()
        .read_line(&mut target)
        .expect("输入合法内容！");
    let target = target.trim();

    let local = match check_type(&target) {
        "id" => {
            let id = target
                .parse::<u64>()
                .expect("id必须为数字");
            search_by_id(tasks, id ).expect("找不到该任务")
        }
        "date" => {
            search_by_date(tasks, target).expect("找不到该任务")
        }
        "description" => {
            search_by_description(tasks, target).expect("找不到该任务")
        }
        _ => {
            println!("无效的查找类型");
            return None
        }
    };
    Some(local)
}

fn change_task(tasks: &mut Vec<Task>) {
    show_tasks(tasks);
    println!("输入你想要修改的任务的描述/ID/创建时间");
    let local = find_task(tasks).expect("找不到该任务");

    tasks[local].show();
    println!("
    选择您要执行的操作：
    1.修改任务描述
    2.修改任务完成与否
    ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("请输入合法内容");
    let input = input.trim();
    match input {
        "1" => {
            println!("请输入新的任务描述");
            let mut new_description = String::new();
            io::stdin()
                .read_line(&mut new_description)
                .expect("请输入合法内容");
            let new_description = new_description.trim();
            tasks[local].description = new_description.to_string();

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

// 检查输入的是id/描述/时间
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

fn control(tasks: &mut Vec<Task>) -> bool {
    print!("
    欢迎使用rust-task!
    请输入对应数字以选择模式
    1.添加任务
    2.删除任务
    3.查看任务
    4.修改任务
    
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
            delete(tasks);
            true
        },
        "3" => {
            show_tasks(tasks);
            true
        },
        "4" => {
            change_task(tasks);
            true
        }
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
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        if !control(&mut tasks) {
            break;
        }
    } 

}    
