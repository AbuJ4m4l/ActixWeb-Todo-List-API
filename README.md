# Todo Application Using Actix-Web and Rust ⚙️

[![Twitter](https://img.shields.io/twitter/follow/MrXeyad?style=social)](https://twitter.com/MrXeyad)

This application uses the Actix-Web framework to create a simple web service for managing a Todo list.

## Structure 📕

- `Todo`: A structure defining a task without an ID.
- `TodoWithID`: A structure defining a task with an ID.
- `AppState`: The shared state containing the Todo list.

## API Endpoints 🔗

- `/add`: To add a new task.
  - Request method: POST
  - JSON request body:
    ```json
    {
      "name": "Task Name",
      "description": "Task Description"
    }
    ```
- `/remove`: To remove a task by ID.
  - Request method: POST
  - JSON request body:
    ```json
    {
      "id": 1
    }
    ```
- `/get`: To retrieve information about a task by ID.
  - Request method: GET
  - JSON request body:
    ```json
    {
      "id": 1
    }
    ```
- `/update`: To update a task.
  - Request method: PATCH
  - JSON request body:
    ```json
    {
      "id": 1,
      "name": "Updated Task Name",
      "description": "Updated Task Description"
    }
    ```

## How to Run the Application ▶️

1. Install Rust and Cargo.
2. Clone the Reposistory
3. Run the command `cargo run`.

## How to Use the Application ✅

Use tools like Postman or curl to send HTTP requests to the above endpoints, using the appropriate JSON request body for each.

---

⚠️ This project is for educational purposes and is not intended for production use.
