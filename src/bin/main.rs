use std::{fs, thread};
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::time::Duration;
use server::ThreadPool;

fn main() {
    let listener = 
            TcpListener::bind("localhost:4000").unwrap();
    
    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    let get = "GET / HTTP/1.1\r\n".as_bytes() ;
    let sleep = "GET /sleep HTTP/1.1\r\n".as_bytes() ;
    // "GET / HTTP/1.1\r\n".as_bytes() this is same as b"GET / HTTP/1.1\r\n"
    let (status,file_name) = if buffer.starts_with(get){
        print!(
            "Request {}",String::from_utf8_lossy(&buffer[..])
        );

        ("200 OK","index.html")
        
    }else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
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
