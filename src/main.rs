use FrontswitchWebServer::ThreadPool;
use std::fs;
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8000")?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream?;
        
        pool.execute(|| {
            if let Err(e) = handle_connection(stream) {
                eprintln!("Error handling connection: {}", e);
            }
        });
    }

    // This message will not be printed if the application runs indefinitely
    println!("Shutting down.");

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let user_ice = b"GET /user/ice HTTP/1.1\r\n";
    let user_atlas = b"GET /user/atlas69 HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(user_ice) {
        ("HTTP/1.1 200 OK", "help.html")
    } else if buffer.starts_with(user_atlas) {
        ("HTTP/1.1 200 OK", "atlas.html")
    } else{
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename)?;
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes())?;
    stream.flush()?;

    Ok(())
}
