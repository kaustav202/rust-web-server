use log::info;
use security::with_auth;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{Filter, Rejection};

mod errors;
mod handlers;
mod models;
mod security;

type UsersDb = Arc<Mutex<HashMap<String, models::User>>>;
type Result<T> = std::result::Result<T, Rejection>;

type TasksDb = Arc<Mutex<HashMap<usize, models::TaskItem>>>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    log4rs::init_file("logconfig.yml", Default::default()).expect("Log config file not found.");
    info!("Starting server...");
    let users_db: UsersDb = Arc::new(Mutex::new(HashMap::new()));
    let tasks_db: TasksDb = Arc::new(Mutex::new(HashMap::new()));

    let root = warp::path::end().map(|| "Welcome to the Rust REST API");

    let user_route = warp::path("user")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_users_db(users_db.clone()))
        .and_then(handlers::create_user);

    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_users_db(users_db.clone()))
        .and_then(handlers::login);
    

    let create_route = warp::path("create")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_tasks_db(tasks_db.clone()))
        .and(security::with_auth())
        .and_then(handlers::create_task_item);
    
    let get_all_route = warp::path("all")
        .and(warp::get())
        .and(with_tasks_db(tasks_db.clone()))
        .and(security::with_auth())
        .and_then(handlers::get_task_items);

    let update_route = warp::path!("update" / usize)
        .and(warp::put())
        .and(warp::body::json())
        .and(with_tasks_db(tasks_db.clone()))
        .and(security::with_auth())
        .and_then(handlers::update_task_by_id);

    let delete_route = warp::path!("delete" / usize)
        .and(warp::delete())
        .and(with_tasks_db(tasks_db.clone()))
        .and(with_auth())
        .and_then(handlers::delete_task_by_id);


    let routes = root
        .or(user_route)
        .or(login_route)
        .or(create_route)
        .or(get_all_route)
        .or(update_route)
        .or(delete_route)
        .with(warp::cors().allow_any_origin())
        .recover(errors::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
}

fn with_users_db(
    users_db: UsersDb,
) -> impl Filter<Extract = (UsersDb,), Error = Infallible> + Clone {
    warp::any().map(move || users_db.clone())
}

fn with_tasks_db(
    tasks_db: TasksDb,
) -> impl Filter<Extract = (TasksDb,), Error = Infallible> + Clone {
    warp::any().map(move || tasks_db.clone())
}