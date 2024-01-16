
# Actix-Web Todo Application 🚀

This Rust-based web application uses Actix-Web to manage a Todo list. It's a simple API that allows users to add, remove, update, and get Todo items.

## Application Structure 🏗️

- `Todo`: A struct to define a task without an ID.
- `TodoWithID`: A struct to define a task with an ID.
- `AppState`: The shared application state, storing Todo items.

## API Endpoints 🌐

### 1. Add a Todo
- **Endpoint**: `/add`
- **Method**: POST
- **Body**:
  ```json
  {
    "name": "Your Task Name",
    "description": "Your Task Description"
  }
  ```
- **Action**: Adds a new Todo item to the list.

### 2. Remove a Todo
- **Endpoint**: `/remove`
- **Method**: POST
- **Body**:
  ```json
  {
    "id": 1
  }
  ```
- **Action**: Removes a Todo item by ID.

### 3. Get a Todo
- **Endpoint**: `/get`
- **Method**: GET
- **Body**:
  ```json
  {
    "id": 1
  }
  ```
- **Action**: Retrieves details of a Todo item by ID.

### 4. Update a Todo
- **Endpoint**: `/update`
- **Method**: PATCH
- **Body**:
  ```json
  {
    "id": 1,
    "name": "Updated Task Name",
    "description": "Updated Task Description"
  }
  ```
- **Action**: Updates the details of a Todo item by ID.

## Running the Application 🏃

1. Ensure Rust and Cargo are installed.
2. Clone the repository or copy the code into a `main.rs` file.
3. Run the application using `cargo run`.
4. The server will start at `http://0.0.0.0:8080`.

## Using the Application 🛠️

Send HTTP requests to the endpoints listed above using tools like Postman or curl. Include the appropriate JSON body for each request.

## Notes 📝

- This application is for educational purposes.
- It demonstrates basic CRUD operations using Actix-Web in Rust.

