use std::error::Error;
use task::Task;
use storage::read_tasks;
use control::control;

mod task;
mod storage;
mod task_manager;
mod control;

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