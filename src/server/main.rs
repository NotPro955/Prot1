use std::net::{TcpListener, TcpStream,SocketAddr};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender,Receiver};
use std::io::{Read,Write};

// fn clinet_list(&mut _stream: TcpStream){
//     con
// }

fn client_handler(mut stream: TcpStream){
    println!("Client connected: {:?}",stream.peer_addr().unwrap());
    let (send,recv) = mpsc::channel();
    // loop{
    //     let mut buffer = [0;50];
    //     stream.read(&mut buffer).expect("Failed to read");
    //     let request = String::from_utf8_lossy(&buffer[..]);
    //     println!("Recieved: {}",request);

    //     let mut response = String::new();
    //     std::io::stdin()
    //         .read_line(&mut response)
    //         .expect("Unable to read line");
    //     if response == "exit"{
    //         break;
    //     }
    //     let response = response.as_bytes();
    //     stream.write(response).expect("Unable to send to client");
    // }
    thread::spawn(move||{
        loop{
            let mut incoming = [0 as u8;20];
            stream.read(&mut incoming).expect("Failed to read Message");
            let msg = String::from_utf8_lossy(&incoming).to_string();
            send.send(msg).expect("Unable to send the message");
        }
    });
    for res in recv{
        println!("Client: {res}");
    };
}

fn client_list(list: &Vec<SocketAddr>){
    println!("Client list
{:?}",list);
}
fn main(){
    let listener = TcpListener::bind("127.0.0.1:8989").unwrap();
    let mut list:Vec<SocketAddr> = Vec::new();
    for stream in listener.incoming(){
        match stream{
            Ok(stream)=>{
                let client_addr = stream.peer_addr().unwrap();
                list.push(client_addr);
                client_list(&mut list);
                thread::spawn( || client_handler(stream));
            }
            Err(_)=>{
                eprintln!("Connection error");
            }
        }
    }
}