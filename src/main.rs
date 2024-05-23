use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
// Uncomment this block to pass the first stage
#[tokio::main]
async fn main() {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        let stream = listener.accept().await;
        match stream {
            Ok((mut stream, _)) => {
                println!("accepted new connection");

                tokio::spawn(handle_stream(&mut stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_stream(stream: &mut TcpStream) {
    let mut buff = [0; 1024];
    loop {
        let bytes_read = stream.read(&mut buff).await.unwrap();
        if bytes_read == 0 {
            return;
        }
        stream.write(b"+PONG\r\n").await.unwrap();
    }
}
