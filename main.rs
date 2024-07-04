use std::io::{self, Write};
use std::io::{Read};
use std::net::TcpStream;
use serde_json::Value;
use serde_json::json;
use std::thread;

use std::fs::File;





struct post_request  
{
    name: String,
    password: i32,
    ip: String,
}

struct get_request
{
    name: String,
    password: i32,
    ip: String,
}

impl get_request
{

    fn new() -> Self
    {
        get_request
        {
            name: "None".to_string(),
            password: 0,
            ip: "None".to_string(),
        }
    }

    fn reply_to_request(&mut self, rec_ip: &str, id: i32, safe_ips: &Vec<&str>)
    {
        if safe_ips.contains(&rec_ip)
        {
            let mut stream = TcpStream::connect(rec_ip);

        }
        else
        {
            println!("Unauthorized IP tried to send a GET request, denying request.");
        }
    }
}

impl post_request
{

    fn new() -> Self
    {
        post_request
        {
            name: "None".to_string(),
            password: 0,
            ip: "None".to_string(),
        }
    }

    fn send_request(&mut self, rec_ip: &str, safe_ips: &Vec<String>)
    {
        if safe_ips.contains(&rec_ip.to_string())
        {
            let mut stream = TcpStream::connect(rec_ip);
            let mut buffer = Vec::new();
            stream.read_to_end(buffer)?;
        }
        else
        {
            println!("Attempt to send data to unauthorized IP, denying request immediatly");
        }
    }

    // take input and add it to the database
    fn reply_to_request(&mut self, reciever: &str, safe_ips: &Vec<String>) -> Result<(), Box<dyn std::error::Error>>
    {
        if safe_ips.contains(&reciever.to_string())
        {
            let mut stream = TcpStream::connect(reciever);
            let mut buffer = Vec::new();

            stream.read_to_end(&mut buffer)?;

            let result: Value = serde_json::from_slice(&buffer)?;

            println!("Recieved Data: {} ", result);
            Ok(())



        }
        else
        {
            println!("Unauthorized IP tried to send or modify data, denying request.");
            Ok(())
        }
    }
}

// sim database

struct Human
{
    id: i32,
    name: String,
    password: i32,
}


fn main()
{
    let Jason = Human 
    {
        id: 1,
        name: "Jason".to_string(),
        password: 1234,
    };

    println!("Hooray!");
}





/*

NOTES


syntax for json 

let [name] = json!({
    "temp": variable,
    "te"
});

let [var] = [name.to_string]
stream.write_all(var.as_bytes()).unwrap()


// other

let listener = TcpListener::bind("0.0.0.0:7878").unwrap();

for stream in listener.incoming()
{
    let stream = stream.unwrap();
    thread::spawn(|| {
        handle_connection(stream);    
    });
}

*/