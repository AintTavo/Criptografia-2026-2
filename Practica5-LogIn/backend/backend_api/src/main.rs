// Librearías estandar
use std::fs;
use std::path::Path;
use std::env;

// Crates Externas
// API
use axum::{Json, Router, http::StatusCode, response::IntoResponse, routing::{get, post}};
use tower_http::cors::{CorsLayer, Any};
use serde::{Deserialize, Serialize};
use serde_json::json;

// SQL Connection
use rusqlite::{Connection, Result, params};

// Hashing
use sha3::{Digest, Sha3_256};

// Terminal Colors
use colored::Colorize;

// JWT Token management
use jsonwebtoken::{encode, EncodingKey, decode, DecodingKey, Header, TokenData, Validation, errors::Error };
use chrono::{Utc, Duration};

// Lettre - Emails through rust
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, AsyncSmtpTransport, AsyncTransport, Tokio1Executor};

use url::Url;


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

#[derive(Deserialize, Serialize, Debug)]
struct RestoreInitRequest {
    email : String,
}

#[derive(Deserialize, Serialize, Debug)]
struct RestoreFinalRequest {
    new_pass : String,
    token : String,
}

#[derive(Deserialize, Serialize, Debug)]
struct VerifyRequest {
    token : String,
}



// Struct para JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // ID del usuario (Subject)
    pub exp: usize,     // Tiempo de expiración (Timestamp)
    pub iat: usize,     // Tiempo de creación
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestoreClaims {
    pub email : i32,
    pub pass : Vec<u8>,
    pub exp : usize,
    pub iat : usize,
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
    
    println!("{} Initializing db", "[Server] :".green().bold());
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

    let _ = conn.execute("ALTER TABLE users ADD COLUMN verified BOOLEAN DEFAULT 0", []);

    conn.close().unwrap_or_else(|_| panic!("Failed to close database."));
    println!("{} Database initialized successfully.", "[Server] :".green().bold());
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/login", post(handle_request_login))
        .route("/val", post(handle_request_val))
        .route("/signin", post(handle_request_signin))
        .route("/restore/init", post(handle_request_restore_init))
        .route("/restore", post(handle_request_restore_post))
        .route("/verify", post(handle_request_verify))
        .layer(cors);

    println!("{} API function created","[Server] :".green().bold());

    let port = env::var("PORT").expect("PORT not declared in de .env");
    let api_ip = format!("0.0.0.0:{}",port);
    let listener = tokio::net::TcpListener::bind(&api_ip).await.unwrap();
    println!("{} Listening on port {}", "[Server] :".green().bold(), port);

    
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
    let db_file = env::var("DATABASE_FILE").expect("Error: DATABASE_FILE not defined in .env");
    let conn = Connection::open(db_file).unwrap_or_else(|_| panic!("Connection failed"));
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
                    println!("{} Verification JWT Created Succesfully", "[Signin/POST] :".yellow());
                    let app_url = env::var("APP_URL").unwrap_or("http://localhost:5500".to_string());
                    let verify_url = format!("{}/frontend/gui/html/verify.html?token={}", app_url, token_string);
                    let _ = send_verification_email(&payload.email, &payload.username, &verify_url).await.map_err(|e| {
                        println!("{} SMTP ERROR: {:?}", "[Signin/POST] :".red(), e);
                    });
                    
                    return (StatusCode::OK, Json(json!({
                        "status" : "Success",
                        "msg" : "Revisa tu correo para verificar tu cuenta"
                    })));
                },
                Err(_) => {
                    println!("{} Verification JWT was not created", "[Signin/POST] :".yellow());
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
    println!("{} Processing log in request", "[Login/POST] :".blue());
    let db_file = env::var("DATABASE_FILE").expect("Error: DATABASE_FILE not defined in .env");
    let conn = Connection::open(db_file).unwrap_or_else(|_| panic!("Connection failed"));
    println!("{} Connection opened with SQLite db", "[Login/POST] :".blue());

    let mut stmt = conn.prepare("SELECT email, password, verified FROM users WHERE email = ?1").unwrap_or_else(|_| panic!("Error building the query"));
    let mut rows = stmt.query(params![payload.email]).unwrap();
    let error_response : String;
    
    match rows.next() {
        Ok(Some(row)) => {
            let email: String = row.get(0).unwrap();
            let password: Vec<u8> = row.get(1).unwrap();
            let verified: bool = row.get(2).unwrap_or(false);

            if !verified {
                println!("{} Account not verified", "[Login/POST] :".blue());
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "status" : "failed",
                        "msg_err" : "Por favor verifica tu correo electrónico antes de iniciar sesión."
                    })));
            }

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
    println!("{} Processing token val request", "[Val/POST] :".purple());

    let error_response : String;

    println!("{} Loading secreat from JWT_SECRET", "[Val/POST] :".purple());
    let secret = env::var("JWT_SECRET").expect("Error: JWT_SECRET not defined in .env");
    let token = payload.jwt;
    
    println!("{} Validating JWT", "[Val/GET] :".purple());
    let val_result = validate_jwt(&token, &secret);
    
    match val_result {
        Ok(token_data) => {
            println!("{} The JWT was correct", "[Val/GET] :".purple());

            if token_data.claims.exp == Utc::now().timestamp() as usize {
                return (
                    StatusCode::OK,
                    Json(json!({
                        "status" : "success",
                        "val" : false
                    }))
                );
            }
            
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
            "val" : false,
            "err_msg" : error_response
        }))
    );
}


// -> Request for restore init
// When created take the user's email and send a recovery code to it
// And changes the token status to "pending"
// 1. Cambiamos 'impl IntoResponse' por el tipo exacto de la tupla
async fn handle_request_restore_init(
    Json(payload): Json<RestoreInitRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    
    println!("{} Processing restore init request", "[Restore/POST] :".red());
    let error_val : u8;
    let error_response : String;

    let db_file = env::var("DATABASE_FILE").expect("Error: DATABASE_FILE not defined in .env");
    let unique_jwt : Option<String>;
    let user_id : i32;
    let username : String;
    let restore_url : String;
    
    {
        let conn = Connection::open(db_file).unwrap_or_else(|_| panic!("Connection failed"));
        let mut stmt = conn.prepare("SELECT id,username,password FROM users WHERE email = ?1").unwrap_or_else(|_| panic!("Error building the query"));
        let mut results = stmt.query(params![payload.email]).unwrap();
        
    
        println!("{} Gathering the data to create the restore token", "[Restore/GET] :".red());
        match results.next() {
            Ok(Some(row)) => { 
                user_id = row.get(0).unwrap();
                username = row.get(1).unwrap();
                let pass : Vec<u8> = row.get(2).unwrap();
                let secret = env::var("JWT_SECRET").expect("JWT_SECRET is not defined in .env");
                let jwt = generate_jwt_for_restore(&user_id, &pass, &secret).unwrap();
                unique_jwt = Some(jwt);
                println!("{} Restore token created successfully", "[Restore/GET] :".red());
            },
            Ok(None) => {
                error_val = 1;
                error_response = "The email is not registered".to_string();
                println!("{} {}", "[Restore/GET] :".red(), "User emails is not in the database".red());
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "status" : "failed",
                        "err_val" : error_val,
                        "err_msg" : error_response,
                        "val" : false
                    }))
                );
            },
            Err(_) => {
                error_val = 2;
                error_response = "Error with the database".to_string();
                println!("{} {}", "[Restore/GET] :".red(), "Database error".red());
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "status" : "failed",
                        "err_val" : error_val,
                        "err_msg" : error_response,
                        "val" : false
                    }))
                );
            }
        }
    
        drop(results);
        drop(stmt);
    
        match unique_jwt {
            Some(jwt) => {
                let result = conn.execute(
                    "INSERT INTO restore_token (user_id, token, timestamp, status) VALUES (?1, ?2, ?3, ?4)", 
                    params![user_id, jwt, Utc::now().date_naive().to_string(), "pending"]
                );
                println!("{} Adding token to db", "[Restore/GET] :".red());
                match result {
                    Ok(_) =>{
                        println!("{}  Token successfully added to db", "[Restore/GET] :".red());
                        let url_app = env::var("APP_URL").unwrap_or("http://localhost:5500".to_string());
                        restore_url = format!("{}/frontend/gui/html/restore.html?token={}", url_app, jwt);
                        println!("{} URL for restoring data successfully created", "[Restore/GET] :".red());
                    },
                    Err(_) => {
                        error_val = 4;
                        error_response = "The token has not been added to the db".to_string();
                        println!("{} {}", "[Restore/GET] :".red(), "Token was not added to db".red());
                        return (
                          StatusCode::UNAUTHORIZED,
                          Json(json!({
                              "status" : "failed",
                              "err_val" : error_val,
                              "err_msg" : error_response,
                              "val" : false
                          }))
                        );
                    }
                }
            },
            None => {
                error_val = 3;
                error_response = "Restore token was not created".to_string();
                println!("{} {}", "[Restore/GET] :".red(), "JWT was not created in previous steps".red());
                return (
                  StatusCode::UNAUTHORIZED,
                  Json(json!({
                      "status" : "failed",
                      "err_val" : error_val,
                      "err_msg" : error_response,
                      "val" : false
                  }))
                );
            }
        }
    }
    
    
    println!("{} Sending email ...", "[Restore/GET] :".red());
    match send_email(&payload.email, &username, &restore_url).await {
        Ok(_) => {
            println!("{} Email sended succesfully", "[Restore/GET] :".red());
        },
        Err(e) => {
            println!("{} SMTP ERROR: {:?}", "[Restore/GET] :".red(), e);
            error_val = 5;
            error_response = "An error ocurred while sending the email".to_string();
            println!("{} {}", "[Restore/GET] :".red(), "Email was not send".red());
            return (
              StatusCode::UNAUTHORIZED,
              Json(json!({
                  "status" : "failed",
                  "err_val" : error_val,
                  "err_msg" : error_response,
                  "val" : false
              }))
            );
        }
    }
    
    return (
        StatusCode::OK,
        Json(json!({
            "status" : "sucess",
            "err_val" : 0,
            "err_msg" : "",
            "val" : true
        }))
    )
}

// -> Request for restore
// When the users enters the recovery code it verifies
// If the code is correct, it changes the token status to "used",
// and restore the password
async fn handle_request_restore_post(Json(payload) : Json<RestoreFinalRequest> ) -> impl IntoResponse {
    println!("{} Processing restore request", "[Restore/POST] :".red());
    let user_id : i32;
    let status : String;
    let timestamp : String;
    let db_path = env::var("DATABASE_FILE").expect("DATABASE_FILE not defined in .env");

    // Obtaining id from the table restore_token
    {
        let conn = Connection::open(&db_path).unwrap_or_else(|_| panic!("Connection failed"));
        let mut stmt = conn.prepare("SELECT user_id,timestamp,status FROM restore_token WHERE token=?1").unwrap_or_else(|_| panic!("Error building the query"));
        let mut results = stmt.query(params![payload.token]).unwrap();

        match results.next() {
            Ok(Some(row)) => {
                user_id = row.get(0).unwrap();
                timestamp = row.get(1).unwrap();
                status = row.get(2).unwrap();
                println!("{} Necessary data in db acquired", "[Restore/POST] :".red());
            },
            Ok(None) => {
                println!("{} {}", "[Restore/POST] :".red(),"The token does not exist".red());
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "status" : "failed",
                        "err_msg" : "The token does not exist",
                        "err_val" : 1,
                        "val" : false
                    }))
                );
            },
            Err(_) => {
                println!("{} {}", "[Restore/POST] :".red(),"Error iterating through rows".red());
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "status" : "failed",
                        "err_msg" : "Error iterating through rows",
                        "err_val" : 2,
                        "val" : false
                    }))
                );
            }
            
        }
        drop(results);
        drop(stmt);
        conn.close().expect("Conección no cerrada correctamente");
    }

    // Revisión de estatus
    println!("{} Checking token status", "[Restore/POST] :".red());
    if status != "pending"{
        println!("{} {}", "[Restore/POST] :".red(),"Not valid status".red());
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status" : "failed",
                "err_msg" : "Token already used or expired",
                "err_val" : 3,
                "val" : false
            }))
        );
    }
    println!("{} Status ok", "[Restore/POST] :".red());

    println!("{} Checking token expiricy", "[Restore/POST] :".red());
    // solo se puede ocupar dentro del mismo dia el token generado
    if timestamp != Utc::now().date_naive().to_string() {
        println!("{} {}", "[Restore/POST] :".red(),"Token expired, changing status to expired".red());
        let conn = Connection::open(&db_path).unwrap_or_else(|_| panic!("Connection failed"));
        let update = conn.execute("UPDATE restore_token SET status=?1 WHERE token=?2", params!["expired", payload.token]);
        match update {
            Ok(_) => {println!("{} {}", "[Restore/POST] :".red(),"Status changed".red())},
            Err(_) => {
                println!("{} {}", "[Restore/POST] :".red(),"Error updating the db".red());
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "status" : "failed",
                        "err_msg" : "Error updating status",
                        "err_val" : 66,
                        "val" : false
                    }))
                );
            }
        }
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "status" : "failed",
                "err_msg" : "Expired token",
                "err_val" : 7,
                "val" : false
            }))
        );
    }
    println!("{} Valid token", "[Restore/POST] :".red());

    // Updating pass from the new table
    {
        let conn = Connection::open(&db_path).unwrap_or_else(|_| panic!("Connection failed"));
        let mut hasher = Sha3_256::new();
        hasher.update(payload.new_pass.as_bytes());
        let hashed_pass = hasher.finalize();

        println!("{} Changing password", "[Restore/POST] :".red());
        let result = conn.execute("UPDATE users SET password=?1 WHERE id=?2", params![hashed_pass.as_slice(), user_id]);
        match result {
            Ok(_) => {
                println!("{} Password changed", "[Restore/POST] :".red());
                println!("{} Deleting old tokens", "[Restore/POST] :".red());
                let deletion = conn.execute("DELETE FROM restore_token WHERE user_id=?1", params![user_id]);
                match deletion {
                    Ok(_) => {println!("{} Old tokens successfully deleted", "[Restore/POST] :".red());},
                    Err(_) => {
                        println!("{} {}", "[Restore/POST] :".red(),"The old tokens hasnt been deleted".red());
                        return (
                            StatusCode::UNAUTHORIZED,
                            Json(json!({
                                "status" : "failed",
                                "err_msg" : "Restore password tokens not deleted",
                                "err_val" : 5,
                                "val" : false
                            }))
                        );
                    }
                }
            },
            Err(_) => {
                println!("{} {}", "[Restore/POST] :".red(),"Token used but has not change the db, changing status to used".red());
                let update = conn.execute("UPDATE restore_token SET status=?1 WHERE token=?2", params!["used",payload.token]);
                match update {
                    Ok(_) => {println!("{} {}", "[Restore/POST] :".red(),"Status changed".red())},
                    Err(_) => {
                        println!("{} {}", "[Restore/POST] :".red(),"Status couldnt be update to used".red());
                        return (
                            StatusCode::UNAUTHORIZED,
                            Json(json!({
                                "status" : "failed",
                                "err_msg" : "Updating status not used",
                                "err_val" : 6,
                                "val" : false
                            }))
                        );
                    }
                }
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "status" : "failed",
                        "err_msg" : "Error at updating db",
                        "err_val" : 4,
                        "val" : false
                    }))
                );
            }
        }
    }

    println!("{} {}", "[Restore/POST] :".red(), "Operation completed".green());
    return (
        StatusCode::OK,
        Json(json!({
            "status" : "success",
            "val" : true
        }))
    )
}


// -> Request for account verification
async fn handle_request_verify(Json(payload) : Json<VerifyRequest>) -> impl IntoResponse {
    println!("{} Processing verify request", "[Verify/POST] :".green());
    let db_path = env::var("DATABASE_FILE").expect("DATABASE_FILE not defined in .env");
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET not defined in .env");

    let val_result = validate_jwt(&payload.token, &secret);
    
    match val_result {
        Ok(token_data) => {
            let email = token_data.claims.sub;
            let conn = Connection::open(&db_path).unwrap_or_else(|_| panic!("Connection failed"));
            let result = conn.execute("UPDATE users SET verified=1 WHERE email=?1", params![email]);
            
            if result.is_ok() {
                println!("{} Account verified successfully", "[Verify/POST] :".green());
                return (
                    StatusCode::OK,
                    Json(json!({
                        "status" : "success",
                        "msg" : "Cuenta verificada correctamente"
                    }))
                );
            }
        },
        Err(_) => {
            println!("{} The token is invalid or expired", "[Verify/POST] :".green());
        }
    }
    
    return (
        StatusCode::UNAUTHORIZED,
        Json(json!({
            "status" : "failed",
            "err_msg" : "Token inválido o expirado",
            "val" : false
        }))
    );
}

/* --------------------------------------------------------------------------------
    Tool functions : Funciones que sirven principalmente como herramientas para el resto del desarrollo.
    --------------------------------------------------------------------------------
*/  


pub async fn send_email(
    email : &str,
    name : &str,
    url : &str
) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(url).expect("URL in existente");

    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER not defined in .env");
    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER not defined in .env");
    let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS not defined in .env");

    let remitent = format!("Example API of login and restore <{}>", smtp_user);
    let destinatary = format!("{} <{}>", name, email);

    let html_corpse = format!("<p>Hola {},</p>\
             <p>Por favor, haz clic en el siguiente enlace para verificar tu cuenta:</p>\
             <p><a href='{}' style='padding: 10px 20px; background-color: #4CAF50; color: white; text-decoration: none; border-radius: 5px;'>Restablecer contraseña</a></p>\
             <p>Si el botón no funciona, copia y pega esto en tu navegador: {}</p>
             <br><p><strong>En caso de que no lo hayas solicitado ignora este mensaje</strong></p>", name, url, url);

    let email = Message::builder()
        .from(remitent.parse()?)
        .to(destinatary.parse()?)
        .subject("Recuperación de contraseña")
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(html_corpse)?;
    
    let creds = Credentials::new(smtp_user,smtp_pass);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server)?
        .credentials(creds)
        .timeout(Some(std::time::Duration::from_secs(10)))
        .build();

    match tokio::time::timeout(std::time::Duration::from_secs(10), mailer.send(email)).await {
        Ok(res) => { res?; },
        Err(_) => {
            println!("{} SMTP ERROR: Connection timed out after 10 seconds. Possibly a firewall or TLS issue.", "[Restore/GET] :".red());
            return Err("SMTP Timeout".into());
        }
    }
    
    Ok(())
}

pub async fn send_verification_email(
    email : &str,
    name : &str,
    url : &str
) -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse(url).expect("URL in existente");

    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER not defined in .env");
    let smtp_user = env::var("SMTP_USER").expect("SMTP_USER not defined in .env");
    let smtp_pass = env::var("SMTP_PASS").expect("SMTP_PASS not defined in .env");

    let remitent = format!("SYS://SECURE <{}>", smtp_user);
    let destinatary = format!("{} <{}>", name, email);

    let html_corpse = format!("<p>Hola {},</p>\
             <p>Gracias por registrarte. Por favor, verifica tu cuenta dando clic en el siguiente enlace:</p>\
             <p><a href='{}' style='padding: 10px 20px; background-color: #00ffcc; color: black; text-decoration: none; border-radius: 5px;'>Verificar Cuenta</a></p>\
             <p>Si el botón no funciona, copia y pega esto en tu navegador: {}</p>
             <br><p><strong>En caso de que no te hayas registrado, ignora este mensaje.</strong></p>", name, url, url);

    let email = Message::builder()
        .from(remitent.parse()?)
        .to(destinatary.parse()?)
        .subject("Verifica tu cuenta en SYS://SECURE")
        .header(lettre::message::header::ContentType::TEXT_HTML)
        .body(html_corpse)?;
    
    let creds = Credentials::new(smtp_user,smtp_pass);

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&smtp_server)?
        .credentials(creds)
        .timeout(Some(std::time::Duration::from_secs(10)))
        .build();

    match tokio::time::timeout(std::time::Duration::from_secs(10), mailer.send(email)).await {
        Ok(res) => { res?; },
        Err(_) => {
            println!("{} SMTP ERROR: Connection timed out after 10 seconds. Possibly a firewall or TLS issue.", "[Restore/GET] :".red());
            return Err("SMTP Timeout".into());
        }
    }
    
    Ok(())
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


pub fn generate_jwt_for_restore(user_id: &i32, user_secret: &[u8], secret: &str) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(4))
        .expect("valid timestamp")
        .timestamp();

    let claims = RestoreClaims {
        email: user_id.to_owned(),
        pass: user_secret.to_owned(),
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



