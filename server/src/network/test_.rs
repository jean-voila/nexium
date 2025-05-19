// use reqwest;

use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

const PORT: u16 = 8080;

fn req() {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:8080") {
        println!("Connected to the server!");
    } else {
        println!("Couldn't connect to server...");
    }

    //////////////////////

    // let res = reqwest::blocking::get("http://127.0.1:8080")
    //     .expect("Failed to send request");

    // let t = res.headers().get("Content-Type");
    // if let Some(t) = t {
    //     println!("Content-Type: {}", t.to_str().unwrap());
    // } else {
    //     println!("No Content-Type header found");
    // }
    // println!("Status: {}", res.status());
    // println!("Headers: {:#?}", res.headers());
    // println!("Response: {:#?}", res);

    // let text = res.text().expect("Failed to read response");
    // println!("Response text: {}", text);
}

fn server() {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT))
        .expect("Failed to bind to address");
    println!("Server listening on port {}", PORT);

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                // Handle the client connection
                println!("New connection: {}", s.peer_addr().unwrap());
                // send HTTP 200 OK response with a content type of text/plain
                // and a body of "Hello, World!"
                // let response = b"HTTP/1.1 200 OK\r\n\r\nHello, World!";
                let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello, World!";
                s.write_all(response).expect("Failed to write to stream");
                s.flush().expect("Failed to flush stream");
                println!("Response sent to client");
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

fn main() {
    // blockchain::test::main();

    req();
    // server();
}
