use crate::{errors, models, security, Result, UsersDb, TasksDb};
use log::{error, info};
use warp::{
    http::{Response, StatusCode},
    reject, Reply,reply
};

use chrono::prelude::*;

pub async fn create_user(user: models::CreateUser, users_db: UsersDb) -> Result<impl Reply> {
    info!("Create user, received UserData: {:?}", user);
    let mut local_db = users_db.lock().await;

    if local_db.contains_key(&user.username) {
        error!("User already exists");
        return Err(reject::custom(errors::CustomError::UserExistsError(user.username)));
    }

    info!("Adding user to the database...");
    let key_count = local_db.keys().len();
    let created_user = models::User {
        user_id: key_count,
        username: user.username,
        password: security::get_hashed_password(&user.password)
    };
    local_db.insert(created_user.username.clone(), created_user.clone());

    info!("User {} added.", &created_user.username);
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body(serde_json::to_string(&created_user).unwrap()))
}

pub async fn login(login_user: models::LoginUser, users_db: UsersDb) -> Result<impl Reply> {
    info!("Received login request...");
    let cur_user_db = users_db.lock().await;
    let user = match cur_user_db.get(&login_user.username) {
        Some(k) => k,
        None => {
            error!("User '{}' not found in database", &login_user.username);
            return Err(reject::custom(errors::CustomError::InvalidCredentialsError));
        }
    };

    info!("User found, verifying password...");
    if !security::verify_password(&login_user.password, &user.password) {
        error!("Password incorrect for user: {}", &login_user.username);
        return Err(reject::custom(errors::CustomError::InvalidCredentialsError));
    }

    info!("Login success!");
    let token = security::get_jwt_for_user(user);
    Ok(Response::builder().status(StatusCode::OK).body(token))
}

pub async fn create_task_item(mut task_list_item: models::TaskItem, tasks_db: TasksDb, curr_user : String) -> Result<impl Reply> {
    info!("Received UserData: {:?}", task_list_item);

    let mut local_db = tasks_db.lock().await;
    let key_count = local_db.keys().len();
    task_list_item.item_id = Some(key_count);
    task_list_item.user_as = Some(curr_user);
    task_list_item.created_at =  Some(Utc::now().to_string());
    task_list_item.modified_at = None;
    local_db.insert(key_count, task_list_item.clone());
    

    info!("Task {} added.", &task_list_item.name);
    Ok(reply::with_status(
        reply::json(&task_list_item),
        StatusCode::CREATED,
    ))
}



pub async fn get_task_items(tasks_db: TasksDb , curr_user: String) -> Result<impl Reply> {
    let local_db = tasks_db.lock().await;
    let local_db: Vec<models::TaskItem> = local_db.values().cloned().collect();

    let c = Some(curr_user);
    let res: Vec<_> = local_db.into_iter()
    .filter(|t_struct| t_struct.user_as == c)
    .collect();

    Ok(reply::with_status(reply::json(&res), StatusCode::OK))
}



pub async fn update_task_by_id(id: usize, updated_data: models::UpdateTaskItem, tasks_db: TasksDb , curr_user : String) -> Result<impl Reply> {
    let mut local_db = tasks_db.lock().await;
    let mut task_item = match local_db.get(&id) {
        Some(item) => item.clone(),
        _ => {
            return Ok(reply::with_status(
                reply::json(&"{}"),
                StatusCode::NOT_FOUND,
            ));
        }
    };

    if task_item.user_as != Some(curr_user) {

        error!("Not authorized to make changes to: {}", &id);
        return Err(reject::custom(errors::CustomError::NotAuthorizedError));
    };

    match updated_data.name {
        Some(name) => {
            println!("updating name from {} to {}", task_item.name, name);
            task_item.name = name;
        }
        _ => {}
    };

    match updated_data.description {
        Some(description) => {
            println!(
                "updating description from {} to {}",
                task_item.description, description
            );
            task_item.description = description;
        }
        _ => {}
    };

    match updated_data.deadline {
        Some(deadline)  => {
            println!(
                "updating item_type from {:?} to {:?}",
                task_item.deadline, deadline
            );
            task_item.deadline = deadline;
        }
        _ => {}
    };

    task_item.modified_at = Some(Utc::now().to_string());


    *local_db.get_mut(&id).unwrap() = task_item.clone();

    Ok(reply::with_status(
        reply::json(&task_item),
        StatusCode::OK,
    ))
}



pub async fn delete_task_by_id(id: usize, tasks_db: TasksDb, curr_user : String) -> Result<impl Reply> {
    let mut local_db = tasks_db.lock().await;

    let task_item = match local_db.get(&id) {
        Some(item) => item.clone(),
        _ => {
            return Ok(reply::with_status(
                reply::json(&"{}"),
                StatusCode::NOT_FOUND,
            ));
        }
    };

    if task_item.user_as != Some(curr_user) {

        error!("Not authorized to make changes to: {}", &id);
        return Err(reject::custom(errors::CustomError::NotAuthorizedError));
    };

    println!("deleting shopping list item with id: {}", id);
    local_db.remove(&id);

    Ok(reply::with_status(
        reply::json(&id),
        StatusCode::OK,
    ))
}