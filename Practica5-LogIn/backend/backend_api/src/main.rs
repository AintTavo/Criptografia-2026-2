// Librearías estandar
use std::fs;
use std::path::Path;
use std::env;

// Crates Externas
// API
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::{get, post}};
use serde::{Deserialize, Serialize};
use serde_json::json;

// SQL Connection
use rusqlite::{params, Connection, Result};

// Hashing
use sha3::{Digest, Sha3_256};

// Terminal Colors
use colored::Colorize;

// JWT Token management
use jsonwebtoken::{encode, EncodingKey, decode, DecodingKey, Header, TokenData, Validation, errors::Error };
use chrono::{Utc, Duration};


// -> Structs for requests
#[derive(Deserialize, Serialize, Debug)]
struct SignInRequest {
    username : String,
    email : String,
    password : String,
}


#[derive(Deserialize, Serialize, Debug)]
struct LogInRequest {
    email : String,
    password : String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ValRequest {
    jwt : String,
}



// Struct para JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // ID del usuario (Subject)
    pub exp: usize,     // Tiempo de expiración (Timestamp)
    pub iat: usize,     // Tiempo de creación
}

/*  
    --------------------------------------------------------------------------------
      Function Main
    --------------------------------------------------------------------------------
*/  
#[tokio::main]
async fn main() {
    println!("{}", "Initializing server...".green());

    println!("{} Loading Enviroment variables", "[Server] :".green().bold());
    dotenvy::dotenv().ok();

    let db_path = &env::var("DATABASE_FILE").expect("Error: DATABASE_FILE not defined in .env");
    let sql_path = &env::var("SQL_FILE_PATH").expect("Error: SQL_FILE_PATH not defined in .env");

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
        .route("/val", get(handle_request_val))
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
async fn handle_request_signin(Json(payload) : Json<SignInRequest>) -> impl IntoResponse {
    println!("{} Processing sign in request", "[Signin/POST] :".yellow());
    let conn = Connection::open("sqlite.db").unwrap_or_else(|_| panic!("Connection failed"));
    println!("{} Conexion opened with SQLite db", "[Signin/POST] :".yellow());

    let mut hasher = Sha3_256::new();
    hasher.update(payload.password.as_bytes());
    let hashed_pass = hasher.finalize();

    let result = conn.execute(
        "INSERT INTO users (username, email, password) VALUES (?1, ?2, ?3)", 
        params![payload.username, payload.email, hashed_pass.as_slice()],
    );
 

    let error_response : String;
    match result {
        Ok(_) => {
            let secret = env::var("JWT_SECRET").expect("Error: JWT_SECRET not defined in .env");
            let jwt = generate_jwt(&payload.email, &secret);

            match jwt {
                Ok(token_string) => {
                    println!("{} JWT Created Succesfully", "[Login/GET] :".blue());
                    return (StatusCode::OK, Json(json!({
                        "status" : "Success",
                        "jwt" : token_string
                    })));
                },
                Err(_) => {
                    println!("{} JWT was not created", "[Login/GET] :".blue());
                    error_response = "Error creating the JWT".to_string();
                }
            }
        },
        Err(_) => {
            error_response = "An Error had happened while singing in".to_string();
        }
    }
    
    return (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "status" : "failed",
            "msg_err" : error_response
        }))
    )
}


// -> Request for log in
async fn handle_request_login(Json(payload) : Json<LogInRequest>) -> impl IntoResponse {
    println!("{} Processing log in request", "[Login/GET] :".blue());
    let conn = Connection::open("sqlite.db").unwrap_or_else(|_| panic!("Connection failed"));
    println!("{} Connection opened with SQLite db", "[Login/GET] :".blue());

    let mut stmt = conn.prepare("SELECT email, password FROM users WHERE email = ?1").unwrap_or_else(|_| panic!("Error building the query"));
    let mut rows = stmt.query(params![payload.email]).unwrap();
    let error_response : String;
    
    match rows.next() {
        Ok(Some(row)) => {
            let email: String = row.get(0).unwrap();
            let password: Vec<u8> = row.get(1).unwrap();

            let mut hasher = Sha3_256::new();
            hasher.update(payload.password.as_bytes());
            let hashed_pass = hasher.finalize();

            let hashed_pass_vec = hashed_pass.as_slice();

            if password == hashed_pass_vec {
                println!("{} The password is correct", "[Login/GET] :".blue());
                println!("{} Creating JWT", "[Login/GET] :".blue());
                let secret = env::var("JWT_SECRET").expect("Error: JWT_SECRET not defined in .env");
                let jwt = generate_jwt(&email, &secret);

                match jwt {
                    Ok(token_string) => {
                        println!("{} JWT Created Succesfully", "[Login/GET] :".blue());
                        return (StatusCode::OK, Json(json!({
                            "status" : "Success",
                            "jwt" : token_string
                        })));
                    },
                    Err(_) => {
                        println!("{} JWT was not created", "[Login/GET] :".blue());
                        error_response = "Error creating the JWT".to_string();
                    }
                }
                
            }
            else {
                println!("{} The password is incorrect", "[Login/GET] :".blue());
                error_response = "The password is incorrect".to_string();
            }
        },
        Ok(None) => {
            println!("{} {}", "[Login/GET] :".blue(), "The email does not existed".red());
            error_response = "The email does not existed or is incorrect".to_string();
        },
        Err(err) => {
            println!("{} Error itarating rows: {:?}", "[Login/GET] :".red(), err);
            error_response = "Error itarating rows".to_string();
        }
    }
    
    return (
        StatusCode::UNAUTHORIZED,
        Json(json!({
                "status" : "failed",
                "err_msg" : error_response
        })));
}

// -> Request for token validation
async fn handle_request_val(Json(payload) : Json<ValRequest>) -> impl IntoResponse {
    println!("{} Processing token val request", "[Val/GET] :".purple());

    let error_response : String;

    println!("{} Loading secreat from JWT_SECRET", "[Val/GET] :".purple());
    let secret = env::var("JWT_SECRET").expect("Error: JWT_SECRET not defined in .env");
    let token = payload.jwt;
    
    println!("{} Validating JWT", "[Val/GET] :".purple());
    let val_result = validate_jwt(&token, &secret);
    
    match val_result {
        Ok(_) => {
            println!("{} The JWT was correct", "[Val/GET] :".purple());
            return (
                StatusCode::OK,
                Json(json!({
                    "status" : "success",
                    "val" : true
                }))
            );
        },
        Err(_) => {
            println!("{} The JWT was incorrect", "[Val/GET] :".purple());
            error_response = "The JWT has expired or is incorrect".to_string();
        }
    }
    
    return (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "status" : "failed",
            "err_msg" : error_response
        }))
    );
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

// -> Generation of JWT
pub fn generate_jwt(user_id: &str, secret: &str) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(4))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        iat: Utc::now().timestamp() as usize,
        exp: expiration as usize,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

// Función para validar el JWT
pub fn validate_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>, Error> {
    let validation = Validation::default();
    // Puedes configurar requisitos extra aquí, por ejemplo, validar el emisor (iss)
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    )
}

