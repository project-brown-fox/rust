use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkerRequest {
    pub id: i32,
    pub cmd: WorkerCommand,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WorkerCommand {
    Echo { msg: String },
    Stop,
}
