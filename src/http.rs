// use std::io::{Read, Write};
// use std::net::{TcpListener, TcpStream};

// #[allow(clippy::unused_io_amount)]
// fn handle_client(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
    
//     // Simple HTTP response
//     let response = b"HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, world!";
    
//     stream.write_all(response).unwrap();
//     stream.flush().unwrap();
// }

// fn main() {
//     let listener = TcpListener::bind("192.168.1.3:7878").unwrap();
//     println!("Listening on port 7878");

//     for stream in listener.incoming() {
//         match stream {
//             Ok(stream) => {
//                 handle_client(stream);
//             }
//             Err(e) => {
//                 eprintln!("Connection failed: {}", e);
//             }
//         }
//     }
// }
