use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post, put, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::fs;
use tokio::{fs::File, io::AsyncReadExt, io::AsyncWriteExt};
use uuid::Uuid;

const STORAGE_FILE_PATH: &str = "./tasks";

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    name: String,
    priority: String,
    details: String,
}

#[derive(Debug, Deserialize)]
struct CreateTask {
    name: String,
    priority: String,
    details: String,
}

#[derive(Debug, Serialize)]
struct TotalTasks {
    task_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateTask {
    name: Option<String>,
    priority: Option<String>,
    details: Option<String>,
}

impl Default for Task {
    fn default() -> Self {
        Task {
            name: String::new(),
            priority: String::new(),
            details: String::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    //CRUD operations: https://www.codecademy.com/article/what-is-crud
    let app = Router::new()      
        .route("/", get(root))
        .route("/tasks", post(create_task)) //create
        .route("/tasks/:task_id", get(show_task)) //read
        .route("/tasks", get(list_tasks)) //read
        .route("/tasks/:task_id", put(update_task)) //update
        .route("/tasks/:task_id", delete(delete_task)); //delete
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> String {
    let file_count = match fs::read_dir(STORAGE_FILE_PATH) {
        Ok(entries) => entries.count(),
        Err(err) => {
            eprintln!("Failed to read directory {}: {}", STORAGE_FILE_PATH, err);
            return "Failed to read tasks directory".to_string();
        }
    };

    format!("Welcome to Your Virtual To-Do List!\nTotal number of tasks to complete: {}\nLet's get organized!!", file_count)
}

async fn create_task(Json(payload): Json<CreateTask>) -> (StatusCode, Json<Task>) {
    let uuid = Uuid::new_v4();
    let file_name = format!("{}.json", uuid);
    let file_path = format!("{}/{}", STORAGE_FILE_PATH, file_name);

    let task = Task {
        name: payload.name,
        priority: payload.priority,
        details: payload.details,
    };

    match serde_json::to_vec(&task) {
        Ok(json_bytes) => {
            if let Ok(mut file) = File::create(&file_path).await {
                if file.write_all(&json_bytes).await.is_ok() {
                    println!("JSON data written to file: {}", file_name);
                    (StatusCode::CREATED, Json(task))
                } else {
                    eprintln!("Failed to write to file: {}", file_path);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(task))
                }
            } else {
                eprintln!("Failed to create file: {}", file_path);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(task))
            }
        }
        Err(err) => {
            eprintln!("Failed to serialize task: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Task::default()))
        }
    }
}

async fn show_task(Path(filename): Path<String>) -> String {
    let file_path = format!("{}/{}", STORAGE_FILE_PATH, filename);

    match File::open(&file_path).await {
        Ok(mut file) => {
            let mut content = String::new();
            if file.read_to_string(&mut content).await.is_ok() {
                println!("Task shown successfully: {}", file_path);
                format!("Task {}:\n{}", filename, content)
            } else {
                eprintln!("Failed to read task content from {}", file_path);
                "Failed to read task content".to_string()
            }
        }
        Err(err) => {
            eprintln!("Failed to open task file {}: {}", file_path, err);
            format!("Failed to show task {}", filename)
        }
    }
}

async fn list_tasks() -> (StatusCode, Json<TotalTasks>){
    let paths = match fs::read_dir(STORAGE_FILE_PATH) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to read task directory: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(TotalTasks { task_ids: Vec::new() }));
        }
    };
    let mut filenames = Vec::new();
    for path in paths {
        let file_name = path.unwrap().path().file_name().unwrap().to_string_lossy().to_string();
        filenames.push(file_name);
    }
    let todo_list = TotalTasks {
        task_ids: filenames,
    };
    
    (StatusCode::CREATED, Json(todo_list))
}

async fn update_task(
    Path(filename): Path<String>,
    Json(payload): Json<UpdateTask>,
) -> (StatusCode, Json<Task>) {
    let file_path = format!("{}/{}", STORAGE_FILE_PATH, filename);

    let existing_task_str = match fs::read_to_string(&file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Failed to read existing task from file {}: {}", file_path, err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Task::default()));
        }
    };

    let mut existing_task: Task = match serde_json::from_str(&existing_task_str) {
        Ok(task) => task,
        Err(err) => {
            eprintln!("Failed to deserialize existing task JSON: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(Task::default()));
        }
    };

    if let Some(name) = payload.name {
        existing_task.name = name;
    }

    if let Some(priority) = payload.priority {
        existing_task.priority = priority;
    }

    if let Some(details) = payload.details {
        existing_task.details = details;
    }

    let json_bytes = match serde_json::to_vec(&existing_task) {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("Failed to serialize updated task: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(existing_task));
        }
    };

    if let Ok(mut file) = File::create(&file_path).await {
        if file.write_all(&json_bytes).await.is_ok() {
            println!("Successfully updated task {}", filename);
            (StatusCode::OK, Json(existing_task))
        } else {
            eprintln!("Failed to write updated task to file {}", file_path);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(existing_task))
        }
    } else {
        eprintln!("Failed to create file for updated task {}", file_path);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(existing_task))
    }
}

async fn delete_task(Path(filename): Path<String>) -> StatusCode {
    let file_path = format!("{}/{}", STORAGE_FILE_PATH, filename);

    if let Err(err) = fs::remove_file(&file_path) {
        eprintln!("Failed to delete file {}: {}", file_path, err);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }    

    println!("File deleted successfully: {}", file_path);
    StatusCode::OK
}