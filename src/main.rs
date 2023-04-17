use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
fn main() {
    let listener = 
            TcpListener::bind("localhost:4000").unwrap();
    
    for stream in listener.incoming(){
        let stream = stream.unwrap();
            handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();

    
    let (status,file_name) = if buffer.starts_with("GET / HTTP/1.1\r\n".as_bytes()){
        print!(
            "Request {}",String::from_utf8_lossy(&buffer[..])
        );

        ("200 OK","index.html")
        
        
       
    }else{
        print!(
            "bad Request ----->\n"
        );
        ("404 NOT FOUND","404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
   
}
