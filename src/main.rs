use tokio::io::AsyncWriteExt;
use std::error::Error;
use tokio::net::TcpStream;
use tokio::net::TcpListener;
use std::io;

#[tokio::main]
async fn main() {

    prototype_build_async_proc(10).await;
}


async fn conn_w_client() -> io::Result<()> {
    
    // Client List Innit
    let listener = TcpListener::bind("127.0.0.1:9998").await?;
    

    Ok(())
}

async fn read_response(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let mut msg = vec![0; 1024];
        loop {
            // Wait for the socket to be readable
            stream.readable().await?;
            // Try to read data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match stream.try_read(&mut msg) {
                Ok(n) => {
                    println!("Recieved {}", n);
                    msg.truncate(n);
                    break;
               }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    println!("BLOCKED BITCH");
                }
                Err(e) => {
                    println!("ERROR");
                }
            }
        }
        println!("GOT = {:?}", String::from_utf8(msg).expect("Found invalid UTF-8"));
        Ok(())
}


async fn prototype_build_async_proc(proc_array_size: i32) -> Result<(), Box<dyn Error>>{
    println!("Before Process");
    let mut m = 0;
    let mut vec = Vec::new();

    while m < proc_array_size {
        vec.push( 
            tokio::spawn(async {
                let mut stream = TcpStream::connect("127.0.0.1:9999").await.unwrap();
                // Write some data.
                stream.write_all(b"hello world!").await.unwrap();
                read_response(stream).await.unwrap();
            }
        ));
        m += 1;
    }

    // Do some other work
    for proc in &mut vec {
        let out = proc.await;
        println!("GOT MSG");
    }

    Ok(())
}
