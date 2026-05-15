// Librearías estandar
use std::fs;
use std::path::Path;

// Crates Externas
// API
use axum::{routing::{get,post,patch}, Json, Router};
use serde::{Deserialize, Serialize};

// SQL Connection
use rusqlite::{params, Connection, Result};

// Hashing
use sha3::{Digest, Sha3_256};

// Terminal Colors
use colored::Colorize;

// -> Structs for requests

#[derive(Deserialize, Serialize, Debug)]
struct SignInRequest {
    username : String,
    email : String,
    password : String,
}

/*  
    --------------------------------------------------------------------------------
      Function Main
    --------------------------------------------------------------------------------
*/  
#[tokio::main]
async fn main() {
    println!("{}", "Initializing server...".green());

    let db_path = "./sqlite.db";
    let sql_path = "../sql/db.sql";

    let exist = Path::new(db_path).exists();
    
    println!("{} Listening on port 3000", "[Server] :".green().bold());
    let conn = Connection::open(db_path).unwrap_or_else(
        |_| panic!("Failed to open database.")
    );

    if !exist {
        if !Path::new(sql_path).exists(){
            panic!("Doc of the sql configuratin does not exists.");
        }
        
        let set_up = fs::read_to_string(sql_path).unwrap_or_else(|_| panic!("Error reading the configuration file."));
        conn.execute_batch(&set_up).unwrap_or_else(|_| panic!("Error creating initial structure."));
        println!("{} Database initial setup completed.", "[Server] :".green().bold());
    }
    else {println!("{} Database already exists, skipping setup.", "[Server] :".green().bold());}

    conn.close().unwrap_or_else(|_| panic!("Failed to close database."));
    println!("{} Database initialized successfully.", "[Server] :".green().bold());
    
    let app = Router::new()
        .route("/login", get(handle_request_login))
        .route("/login", post(handle_request_login_token))
        .route("/signin", post(handle_request_signin))
        .route("/restore", get(handle_request_restore_get))
        .route("/restore", post(handle_request_restore_post));

    println!("{} API function created","[Server] :".green().bold());
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("{} Listening on port 3000", "[Server] :".green().bold());

    
    println!("{}", "Server started successfully".green());
    axum::serve(listener, app).await.unwrap();
    
}


/*  
    --------------------------------------------------------------------------------
    Function for request 
    --------------------------------------------------------------------------------
*/  

// -> Request for sign in
// Saves the user's credentials and creates a session token
async fn handle_request_signin(Json(payload) : Json<SignInRequest>) {
    println!("{} Processing sign in request", "[Signin/POST] :".yellow());
    let conn = Connection::open("sqlite.db").unwrap_or_else(|_| panic!("Connection failed"));
    println!("{} Conexion open with SQLite db", "[Signin/POST] :".yellow());

    let mut hasher = Sha3_256::new();
    hasher.update(payload.password.as_bytes());
    let hashed_pass = hasher.finalize();

    let result = conn.execute(
        "INSERT INTO users (username, email, password) VALUES (?1, ?2, ?3)", 
        params![payload.username, payload.email, hashed_pass.as_slice()],
    );

    if result.is_err() {
        println!("{} {}", "[Signin/POST] :".yellow(), "Error inserting the data to the table".red());
    }
    else{
        println!("{} {}", "[Signin/POST] :".yellow(), "Sucessfully inserting the data to the table".green());
    }
    
    println!("{} Saving user credentials", "[Signin/POST] :".yellow());
    conn.close().unwrap_or_else(|_| panic!("Failed to close connection"));
    println!("{} Conexion closed with SQLite db", "[Signin/POST] :".yellow());
}

// -> Request for log in
async fn handle_request_login() {
     println!("{} Processing log in request", "[Login/GET] :".blue());
}

// -> Request for log in
async fn handle_request_login_token() {
    println!("{} Processing token validation request", "[Login/POST] :".blue());
}

// -> Request for restore init
// When created take the user's email and send a recovery code to it
// And changes the token status to "pending"
async fn handle_request_restore_get() {
    println!("{} Processing restore init request", "[Restore/GET] :".red());
}

// -> Request for restore
// When the users enters the recovery code it verifies
// If the code is correct, it changes the token status to "used",
// and restore the password
async fn handle_request_restore_post() {
    println!("{} Processing restore request", "[Restore/POST] :".red());
}