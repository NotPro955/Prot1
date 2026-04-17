use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read,Write};

fn client_handler(mut stream: TcpStream){
    println!("Client connected: {:?}",stream);
    loop{
        let mut buffer = [0;50];
        stream.read(&mut buffer).expect("Failed to read");
        let request = String::from_utf8_lossy(&buffer[..]);
        println!("Recieved: {}",request);
        thread::spawn(|| println!("This the thread"));
        let mut response = String::new();
        std::io::stdin()
            .read_line(&mut response)
            .expect("Unable to read line");
        let response = response.as_bytes();
        stream.write(response).expect("Unable to send to client");
    }
}

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8989").unwrap();

    for stream in listener.incoming(){
        match stream{
            Ok(stream)=>{
                thread::spawn( || client_handler(stream));
            }
            Err(_)=>{
                eprintln!("Connection error");
            }
        }
    }
}