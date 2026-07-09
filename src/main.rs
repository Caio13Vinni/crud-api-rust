use postgres::{Client, NoTls};
use postgres::Error as PostgresError;
use std::net::{TcpListener,TcpStream};
use std::io::{Read,Write};
use std::env;

#[macro_use]
extern crate serde_derive;

//Modelo de Estrutura: id, name, email
#[derive(Serialize, Deserialize)]
struct user {
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