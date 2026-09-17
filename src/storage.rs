use std::error::Error;
use std::fs::File;
use std::io::{self, ErrorKind, Read, Write};
use crate::task::Task;


// 保存至文件
pub fn save_tasks(tasks: &[Task]) -> Result<(),Box<dyn Error>> {
    let mut file = File::create("tasks.txt")?;
    for task in tasks {
        let line = format!(
            "{} | {} | {} | {}\n",
            task.id,
            task.description,
            task.completed,
            task.date
        );
        file.write_all(line.as_bytes())?;
    }
    Ok(())
}

// 读取文件
pub fn read_tasks(tasks: &mut Vec<Task>) -> Result<(), Box<dyn Error>> {
    let mut tasks_line = String::new();
    let mut f = match File::open("tasks.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("tasks.txt") {
                Ok(fc) => fc,
                Err(e) => return Err(e.into())
            },
            _ => return Err(error.into()),
        }
    };
    f.read_to_string(&mut tasks_line)?;
    for line in tasks_line.lines() {
        let parts:Vec<&str> = line.split(" | ").collect();
        if parts.len() != 4 {
            let file_broken_error = io::Error::new(ErrorKind::InvalidData, "文件损坏！");
            return Err(Box::new(file_broken_error));
        
        }
        let id = parts[0].parse::<u64>()?;
        let description = parts[1].to_string();
        let completed = parts[2].parse::<bool>()?;
        let date = parts[3].to_string();
        tasks.push(Task::new(id, description, completed, date));
    }
    Ok(())
}