# Rust To-Do List API

This project is a RESTful API implementation for managing a To-Do List using Rust and the Axum web framework. It provides endpoints to perform [CRUD](https://www.codecademy.com/article/what-is-crud) (Create, Read, Update, Delete) operations on tasks. Task files are stored as unique identifiers that contain JSON data detailing the tasks.

## Features

- **Create Task**: Add a new task to the To-Do list.
- **Read Task**: Retrieve task details by task ID or list all tasks.
- **Update Task**: Modify existing task details.
- **Delete Task**: Remove a task from the To-Do list.

## Technologies Used

- **Rust**: The primary programming language used to develop the API.
- **Axum**: A web framework for building asynchronous APIs in Rust. It provides utilities for routing (`Router`), handling HTTP requests (`get`, `post`, `put`, `delete`), and extracting data from requests (`extract`).
- **Serde**: A Rust library for serializing and deserializing data structures to and from JSON format (`Serialize`, `Deserialize`). This is used to handle JSON data in the API endpoints.
- **Tokio**: A runtime for writing reliable asynchronous applications in Rust. It provides asynchronous I/O (`AsyncReadExt`, `AsyncWriteExt`) necessary for handling file operations asynchronously.
- **Uuid**: A crate for generating Universally Unique Identifiers (UUIDs) in Rust. It is used to generate unique identifiers for tasks.
- **Standard Rust Libraries**: Utilized for file system operations (`std::fs`) and HTTP status code representation (`http::StatusCode`).

## API Endpoints

- `GET /`: Home endpoint displaying welcome message and total number of tasks on To-Do List.
- `POST /tasks`: Create a new task.
- `GET /tasks/:task_id`: Retrieve details of a specific task.
- `GET /tasks`: List all tasks.
- `PUT /tasks/:task_id`: Update details of a specific task.
- `DELETE /tasks/:task_id`: Delete a specific task.

## Getting Started

To run the API locally, follow these steps:

1. **Clone the repository**:
   *(SSH example)*
   ```bash
   git clone git@github.com:DenaliTonn/API.git
   ```

3. **Navigate to the project directory**:
   ```bash
   cd api
   ```

4. **Install dependencies**:
   Make sure you have Rust and Cargo installed. If not, refer to [Rust's official website](https://www.rust-lang.org/) for installation instructions.
   
5. **Build and run the application**:
   ```bash
   cargo build
   cargo run
   ```

6. **Use the API**:
   The API will be accessible at `http://localhost:3000`. You can use tools like `curl` to interact with the endpoints in a new terminal tab.

## Examples

### Display Home Endpoint
```bash
curl --header "Content-Type: application/json" "http://localhost:3000"
```

### Create a Task
```bash
curl --header "Content-Type: application/json" --request POST --data '{ "name": "Buy ice cream", "priority": "low", "details": "Out of ice cream. Need to go shopping." }' "http://localhost:3000/tasks"
```

### Retrieve a Task
```bash
curl --header "Content-Type: application/json" "http://localhost:3000/tasks/cb733f50-396c-4d0e-9848-3d4203a6245f.json"
```

### List All Tasks
```bash
curl --header "Content-Type: application/json" "http://localhost:3000/tasks"
```

### Update a Task (full or partial)
```bash
curl --header "Content-Type: application/json" --request PUT --data '{ "name": "Buy LOTS of ice cream", "priority": "low", "details": "Out of ice cream!!" }' "http://localhost:3000/tasks/cb733f50-396c-4d0e-9848-3d4203a6245f.json"
```
```bash
curl --header "Content-Type: application/json" --request PUT --data '{ "name": "Buy Ben and Jerry'\''s ice cream" }' "http://localhost:3000/tasks/cb733f50-396c-4d0e-9848-3d4203a6245f.json"
```

### Delete a Task
```bash
curl --header "Content-Type: application/json" --request DELETE "http://localhost:3000/tasks/cb733f50-396c-4d0e-9848-3d4203a6245f.json"
```
---
## Created By

Denali Tonn & Caroline Ellis <br />
CMSI-3550 Final Project <br />
