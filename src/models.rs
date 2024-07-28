use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Clone)]
pub struct User {
    pub user_id: usize,
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateUser {
    pub username: String,
    pub password: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskItem {
    pub item_id: Option<usize>,
    pub name: String,
    pub description: String,
    pub created_at : Option<String>,
    pub modified_at : Option<String>,
    pub user_as : Option<String>,
    pub deadline : String
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateTaskItem {
    pub name: Option<String>,
    pub description: Option<String>,
    pub deadline: Option<String>
}