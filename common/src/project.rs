use serde::{Deserialize, Serialize};

use crate::{task::Task, user::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub status: ProjectStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub owner: User,
    pub task_ids: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectCreate {
    pub name: String,
    pub status: ProjectStatus,
    pub owner_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProjectStatus {
    Active,
    Completed,
    OnHold,
}
