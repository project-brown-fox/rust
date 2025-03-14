use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub user: User,
    pub project_id: i32,
    pub status: TaskStatus,
    pub due_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskCreate {
    pub title: String,
    pub user: User,
    pub project_id: i32,
    pub status: TaskStatus,
    pub due_date: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Open,
    InProgress,
    Closed,
}
