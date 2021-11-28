use tokio::net::TcpListener;
use tokio::net::TcpStream;
use std::io;
use std::error::Error;
use tokio::io::AsyncWriteExt;

async fn process_socket(mut socket:TcpStream) -> Result<(), Box<dyn Error>>{
    // do work with socket here
    let mut buf = vec![0; 4096];
    socket.readable().await?;
    match socket.try_read(&mut buf) {
        Ok(n) => {
            buf.truncate(n);
            socket.writable().await?;
            socket.write_all(b"Hello Back").await?;
            println!("Sent Hello Back");
        }
        Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
            println!("Blocked")            
        }
        Err(e) => {
            println!("Error")
        }
    }
    println!("GOT = {:?}", String::from_utf8(buf).expect("Found invalid UTF-8"));
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Stream
    let listener = TcpListener::bind("127.0.0.1:9999").await?;


    loop {
        let (socket, _) = listener.accept().await?;
        process_socket(socket).await;
    }
}
