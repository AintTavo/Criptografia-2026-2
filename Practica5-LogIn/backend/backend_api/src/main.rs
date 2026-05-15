// Librearías estandar


// Crates Externas
// API
use axum::{routing::{get,post,patch}, Json, Router};
use serde::{Deserialize, Serialize};

// SQL Connection
use rusqlite::{Connection, Result};

// Hashing
use sha3::Sha3_256;

// Terminal Colors
use colored::Colorize;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
