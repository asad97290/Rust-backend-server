use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = 
            TcpListener::bind("localhost:4000").unwrap();
            for stream in listener.incoming(){
                let stream = stream.unwrap();
                handle_connection(stream);
            }
}

fn handle_connection(mut strem: TcpStream){
    let mut buffer = [0;1024];
    
}
