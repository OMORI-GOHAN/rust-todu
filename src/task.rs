pub struct Task {   
    pub id: u64,
    pub description: String,
    pub completed: bool,
    pub date: String, // yyyy-mm-dd 

}


impl Task {
    pub fn new(id: u64, description: String, completed: bool, date: String) -> Task{
        Task { id, description, completed, date }
    }
    
    pub fn show(&self) {
        print!("
        任务ID：{}
        描述：{}
        完成与否：{}
        时间：{}
        ",self.id, self.description, self.completed, self.date);
    }
}