use rand::Rng;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

const DATA_SIZE: usize = 200 * 1024 * 1024; // 200MB

fn generate_random_data() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let data: Vec<u8> = (0..DATA_SIZE).map(|_| rng.gen::<u8>()).collect();
    data
}

fn handle_client(mut stream: TcpStream, data: &[u8]) {
    loop {
        // Send the random data to the client repeatedly
        if let Err(e) = stream.write_all(data) {
            eprintln!("Error sending data: {}", e);
            break;
        }
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind address");
    let random_data = generate_random_data();

    println!("Server listening on {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Clone the random data for each client
                let data = random_data.clone();

                // Spawn a new thread to handle each incoming connection
                thread::spawn(move || {
                    handle_client(stream, &data);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
