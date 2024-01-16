use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize, Debug, Clone)]
struct Todo {
    name: String,
    description: String,
}

struct AppState {
    todos: Mutex<Vec<Todo>>,
}

async fn add(todo: web::Json<Todo>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    todos.push(todo.into_inner());
    format!("Added: {:?}", *todos)
}

async fn remove(
    todo: web::Json<Todo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();

    match todos.iter().position(|t| t.name == todo.name) {
        Some(index) => {
            todos.remove(index);
            format!("Removed: {:?}", *todos)
        },
        None => "Todo not found".to_string(),
    }
}

async fn get(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    format!("{:?}", *todos)
}

async fn update(todo: web::Json<Todo>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    if let Some(index) = todos.iter().position(|t| t.name == todo.name) {
        todos[index] = todo.into_inner();
        format!("Updated: {:?}", *todos)
    } else {
        "Todo item not found".to_string()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todos: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/add", web::post().to(add))
            .route("/remove", web::post().to(remove))
            .route("/get", web::get().to(get))
            .route("/update", web::patch().to(update))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}