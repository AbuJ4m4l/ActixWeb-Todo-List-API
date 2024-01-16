use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize, Debug, Clone)]
struct Todo {
    name: String,
    description: String,
}

#[derive(Deserialize, Debug, Clone)]
struct TodoWithID {
    id: u32,
    name: String,
    description: String,
}

struct AppState {
    todos: Mutex<Vec<TodoWithID>>, // Use TodoWithID instead of Todo
}

async fn add(todo: web::Json<Todo>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let new_id = todos.len() as u32; // Generate a new ID
    todos.push(TodoWithID {
        id: new_id,
        name: todo.name.clone(),
        description: todo.description.clone(),
    });
    format!("Added: {:?}", todos)
}

async fn remove(
    id: web::Json<u32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();

    match todos.iter().position(|t| t.id == *id) { // Dereference id
        Some(index) => {
            todos.remove(index);
            format!("Removed: {:?}", todos)
        },
        None => "Todo not found".to_string(),
    }
}

async fn get(
    id: web::Json<u32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    match todos.iter().find(|t| t.id == *id) { // Change to find and dereference id
        Some(todo) => format!("{:?}", todo),
        None => "Todo not found".to_string(),
    }
}

async fn update(
    todo: web::Json<TodoWithID>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    if let Some(index) = todos.iter().position(|t| t.id == todo.id) {
        todos[index] = todo.into_inner(); // Correctly update the todo
        format!("Updated: {:?}", todos)
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
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
