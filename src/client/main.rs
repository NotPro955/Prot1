use std::net::TcpStream;
use std::io::{Read, Write};

fn server_connect() {
    match TcpStream::connect("localhost:8989") {
        Ok(mut server) => {
            println!("Successfully connected to server");
            loop{
                let mut msg = String::new();
                print!("To Server: ");
                std::io::stdin()
                    .read_line(&mut msg)
                    .expect("Unable to read line");
                let msg = msg.as_bytes();
                server.write(msg).unwrap();
                println!("");
                let mut data = [0 as u8; 6];
                server.read(&mut data).unwrap();
                let response = String::from_utf8_lossy(&data);
                println!("From Server: {}",response);
            }
        },
        Err(e) => println!("Failed to connect: {}", e),
    }
}   

fn help(){
    let command = r#"
    --------------------------------------------------------------
    help                    - shows the list of commmand available

    server                  - connects to the server

    exit                    - to exit the program
    --------------------------------------------------------------
    "#;
    println!("{}",command);
}

fn main(){
    println!("WELCOME");
    loop{
        let mut terminal = String::new();
        print!(">>");
        std::io::stdin()
            .read_line(&mut terminal)
            .expect("Unable to read command");
        let terminal = terminal.trim();
        match terminal{
            "help" => help(),
            "server" => server_connect(),
            "exit" => break,
            _ => println!("Command not found"),
        }
    }
}