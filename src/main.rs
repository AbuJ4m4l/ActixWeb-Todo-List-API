use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;
use std::sync::Mutex;

#[derive(Deserialize, Debug, Clone)]
struct Todo {
    name: String,
    description: String,
}

#[derive(Deserialize, Debug, Clone)]
struct TodoWithID {
    id: i32,
    name: String,
    description: String,
}

struct AppState {
    todos: Mutex<Vec<TodoWithID>>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Id {
    id: i32,
}

async fn add(todo: web::Json<Todo>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let new_id = todos.len() as i32;
    todos.push(TodoWithID { id: new_id, name: todo.name.clone(), description: todo.description.clone() }); 
     HttpResponse::Ok().finish()
}

async fn remove(
    id: web::Json<Id>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();

    match todos.iter().position(|t| Id { id: t.id } == Id { id: id.id }) {
        Some(index) => {
            todos.remove(index);
          HttpResponse::Ok().finish()
        },
        None => HttpResponse::NotFound().finish()
    }
}

async fn get(
    id: web::Json<Id>,
    data: web::Data<AppState>,
) -> impl Responder {
    let todos = data.todos.lock().unwrap();
    match todos.iter().find(|t| Id { id: t.id } == Id { id: id.id }) {
        Some(todo) => HttpResponse::Ok().json(format!("{:?}", todo)),
        None => HttpResponse::NotFound().finish()
    }
}

async fn update(
    todo: web::Json<TodoWithID>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    if let Some(index) = todos.iter().position(|t| t.id == todo.id) {
        todos[index] = todo.into_inner();
        HttpResponse::Ok().json(format!("Updated: {:?}", todos))
    } else {
        HttpResponse::NotFound().finish()
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
