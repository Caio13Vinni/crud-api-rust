use postgres::Error as PostgresError;
use postgres::{Client, NoTls};
use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

#[macro_use]
extern crate serde_derive;

//Model: User struct with id, name, email
#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

//DB URL
fn db_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL não definida")
}

//Constantes
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const CREATED_RESPONSE: &str = "HTTP/1.1 201 CREATED\r\nContent-Type: application/json\r\n\r\n";
const BAD_REQUEST: &str = "HTTP/1.1 400 BAD REQUEST\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND\r\nContent-Type: application/json\r\n\r\n";
const NO_CONTENT_RESPONSE: &str = "HTTP/1.1 204 NO CONTENT\r\n\r\n";
const INTERNAL_SERVER_ERROR: &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

// main function
fn main() {
    //set db
    if let Err(e) = set_database() {
        println!("Database error: {}", e);
        return;
    }

    //start server and print port
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server running on port 8080");

    //handle the client
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }

            Err(e) => {
                println!("Connection error: {}", e);
            }
        }
    }
}

//handle_client function
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 4096];

    match stream.read(&mut buffer) {
        Ok(size) => {
            let request = String::from_utf8_lossy(&buffer[..size]);

            let (status, body) = match request.as_ref() {
                r if r.starts_with("POST /users ") => handle_post_request(r),
                r if r.starts_with("GET /users/") => handle_get_request(r),
                r if r.starts_with("PUT /users/") => handle_put_request(r),
                r if r.starts_with("GET /users ") => handle_get_all_request(),
                r if r.starts_with("DELETE /users/") => handle_delete_request(r),

                _ => (NOT_FOUND.to_string(), "Route not found".to_string()),
            };

            if let Err(e) = stream.write_all(format!("{}{}", status, body).as_bytes()) {
                println!("Response error: {}", e);
            }
        }

        Err(e) => {
            println!("Read error: {}", e);
        }
    }
}
//CONTROLLERS

//handle_post_request function
fn handle_post_request(request: &str) -> (String, String) {
    match (
        get_user_request_body(request),
        Client::connect(&db_url(), NoTls),
    ) {
        (Ok(user), Ok(mut client)) => {
            if user.name.is_empty() || user.email.is_empty() {
                return (
                    BAD_REQUEST.to_string(),
                    "Name and email are required".to_string(),
                );
            }

            match client.query_one(
                "
                INSERT INTO users (name,email)
                VALUES ($1,$2)
                RETURNING id
                ",
                &[&user.name, &user.email],
            ) {
                Ok(row) => {
                    let id: i32 = row.get(0);

                    let response = User {
                        id: Some(id),
                        name: user.name,
                        email: user.email,
                    };

                    (
                        CREATED_RESPONSE.to_string(),
                        serde_json::to_string(&response).unwrap(),
                    )
                }

                Err(e) => {
                    println!("Insert error {}", e);

                    (
                        INTERNAL_SERVER_ERROR.to_string(),
                        "Insert error".to_string(),
                    )
                }
            }
        }

        (Err(_), _) => (BAD_REQUEST.to_string(), "Invalid JSON".to_string()),

        (_, Err(_)) => (
            INTERNAL_SERVER_ERROR.to_string(),
            "Database connection error".to_string(),
        ),
    }
}

//handle_get_request function
fn handle_get_request(request: &str) -> (String, String) {
    match (
        get_id(request).parse::<i32>(),
        Client::connect(&db_url(), NoTls),
    ) {
        (Ok(id), Ok(mut client)) => {
            match client.query_opt(
                "
                SELECT id,name,email
                FROM users
                WHERE id=$1
                ",
                &[&id],
            ) {
                Ok(Some(row)) => {
                    let user = User {
                        id: Some(row.get(0)),
                        name: row.get(1),
                        email: row.get(2),
                    };

                    (
                        OK_RESPONSE.to_string(),
                        serde_json::to_string(&user).unwrap(),
                    )
                }

                Ok(None) => (NOT_FOUND.to_string(), "User not found".to_string()),

                Err(_) => (
                    INTERNAL_SERVER_ERROR.to_string(),
                    "Database error".to_string(),
                ),
            }
        }

        _ => (BAD_REQUEST.to_string(), "Invalid id".to_string()),
    }
}

//handle get all request
fn handle_get_all_request() -> (String, String) {
    match Client::connect(&db_url(), NoTls) {
        Ok(mut client) => {
            match client.query(
                "
                SELECT id,name,email
                FROM users
                ",
                &[],
            ) {
                Ok(rows) => {
                    let users: Vec<User> = rows
                        .into_iter()
                        .map(|row| User {
                            id: Some(row.get(0)),
                            name: row.get(1),
                            email: row.get(2),
                        })
                        .collect();

                    (
                        OK_RESPONSE.to_string(),
                        serde_json::to_string(&users).unwrap(),
                    )
                }

                Err(_) => (
                    INTERNAL_SERVER_ERROR.to_string(),
                    "Database error".to_string(),
                ),
            }
        }

        Err(_) => (
            INTERNAL_SERVER_ERROR.to_string(),
            "Connection error".to_string(),
        ),
    }
}

//handle put request
fn handle_put_request(request: &str) -> (String, String) {
    match (
        get_id(request).parse::<i32>(),
        get_user_request_body(request),
        Client::connect(&db_url(), NoTls),
    ) {
        (Ok(id), Ok(user), Ok(mut client)) => {
            if user.name.is_empty() || user.email.is_empty() {
                return (
                    BAD_REQUEST.to_string(),
                    "Name and email required".to_string(),
                );
            }

            match client.execute(
                "
                UPDATE users
                SET name=$1,email=$2
                WHERE id=$3
                ",
                &[&user.name, &user.email, &id],
            ) {
                Ok(rows) => {
                    if rows == 0 {
                        (NOT_FOUND.to_string(), "User not found".to_string())
                    } else {
                        (OK_RESPONSE.to_string(), "User updated".to_string())
                    }
                }

                Err(_) => (
                    INTERNAL_SERVER_ERROR.to_string(),
                    "Database error".to_string(),
                ),
            }
        }

        _ => (BAD_REQUEST.to_string(), "Invalid data".to_string()),
    }
}

// handle delete request
fn handle_delete_request(request: &str) -> (String, String) {
    match (
        get_id(request).parse::<i32>(),
        Client::connect(&db_url(), NoTls),
    ) {
        (Ok(id), Ok(mut client)) => match client.execute("DELETE FROM users WHERE id=$1", &[&id]) {
            Ok(rows) => {
                if rows == 0 {
                    (NOT_FOUND.to_string(), "User not found".to_string())
                } else {
                    (NO_CONTENT_RESPONSE.to_string(), "".to_string())
                }
            }

            Err(_) => (
                INTERNAL_SERVER_ERROR.to_string(),
                "Database error".to_string(),
            ),
        },

        _ => (BAD_REQUEST.to_string(), "Invalid id".to_string()),
    }
}

// set db
fn set_database() -> Result<(), PostgresError> {
    let mut client = Client::connect(&db_url(), NoTls)?;

    client.execute(
        "
        CREATE TABLE IF NOT EXISTS users(
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR NOT NULL
        )
        ",
        &[],
    )?;

    Ok(())
}
 
//get_id function
fn get_id(request: &str) -> &str {
    request
        .split("/")
        .nth(2)
        .unwrap_or("")
        .split_whitespace()
        .next()
        .unwrap_or("")
}

//deserialize user from request body with the id
fn get_user_request_body(request: &str) -> Result<User, serde_json::Error> {
    let body = request.split("\r\n\r\n").last().unwrap_or("");

    serde_json::from_str(body)
}
