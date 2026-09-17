use crate::task::Task;

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
            search_by_id(tasks, id).expect("找不到该任务")
        }
        "date" => {
            search_by_date(tasks, target).expect("找不到该任务")
        }
        "description" => {
            search_by_description(tasks, target).expect("找不到该任务")
        }
        _ => {
            println!("无效的查找类型");
            return None;
        }
    };

    Some(local)
}
