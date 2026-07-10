use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::{env, vec};

#[macro_use]
extern crate serde_derive;

//Modelo de Estrutura: id, name, email
#[derive(Serialize, Deserialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

// DB_url
const DB_URL : &str = !env("DATABASE_URL");

//CONSTANTES    

const OK_RESPONSE : &str = "HTTP/1.1 200 ok\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND : &str = "HTTP/1 404 NOT_FOUND\r\n\r\n";
const INTERNAL_SERVER_ERROR : &str = "HTTP/1.1 500 INTERNAL SERVER ERROR\r\n\r\n";

//Main
fn main(){
    //setar db
    if let Err(e) = set_database(){
        println!("Error {}", e);
        return;
    }

    //start server e print porta
    let listener = TcpListener::bind(format!(0.0.0.0:8080)).unwrap();
    println!("Server Start port at 8080");

    //handle the client
    for stream in listener.incoming(){
        match Stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                print!("Error: {}, e");
            }
        }
      }
}
//handle_client function
fn handle_client(mut stream: TcpStream){
    let mut buffer = [0; 1024];
    let mut request  = String:new();

    match stream.read(&mut buffer) {
        ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if request_with(POST /users) => handle_post_request(r),
                r if request_with(GET /users/) => handle_get_request(r),
                r if request_with(PUT /users) => handle_put_request(r),
                r if request_with(DELETE /users) => handle_delete_request(r),
                _ => (NOT_FOUND, "Not Found".to_string()),
            };

            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
            }
            Err(e) => {
                print!("Error {}", e);
            }
            
        }
        
    }

//set_database Function
fn set_database() -> Resul<(), PostgresError> {
    //connect to db
    let mut client = Client::connect(DB_URL, NoTls)?;

    //criar tabela
    client.execute(
        "CREATE TABLE IF NOT EXIST users(
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL,
        email VARCHAR NOT NULL
        )",
        &[]
    )?; 
}

//get_id function 
fn get_id(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

//desserialize user from request body with the id 
fn get_user_resquest_body(request: &str) -> Result<user, serde_json::Error>{
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}