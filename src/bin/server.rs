use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() {
    let listner = TcpListener::bind("127.0.0.1:55331").unwrap();

    for connection in listner.incoming() {
        process_connection(connection.unwrap());
    }
}

fn process_connection(mut stream: TcpStream) {
    thread::spawn(move || {
        for i in 0..5 {
            let _ = stream.write_all(&[i as _]);
            thread::sleep(Duration::from_secs(1));
        }
    });
}
