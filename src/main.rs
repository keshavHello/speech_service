use std::process::Command;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs::File;

fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8999").unwrap();
    println!("Server running on http://localhost:8999");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_connection(stream),
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Read request once (do NOT use read_to_end)
    let mut buffer = [0; 8192];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(0) => return,
        Ok(n) => n,
        Err(_) => {
            not_found(stream);
            return;
        }
    };

    let request_bytes = &buffer[..bytes_read];
    let request = String::from_utf8_lossy(request_bytes);
    println!("Request:\n{}", request);

    let request_line = match request.lines().next() {
        Some(line) => line,
        None => {
            not_found(stream);
            return;
        }
    };

    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    println!("Method: {}, Path: {}", method, path);

    match (method, path) {
        ("GET", "/") => get_data(stream),
        ("POST", "/speak") => handle_post_file(stream, request_bytes),
        _ => not_found(stream),
    }
}

fn get_data(mut stream: TcpStream) {
    respond(stream, "200 OK", "Hello from Rust server!");
}

fn not_found(mut stream: TcpStream) {
    respond(stream, "404 NOT FOUND", "Not Found");
}

fn handle_post_file(mut stream: TcpStream, buffer: &[u8]) {
    println!("Handling POST /speak");

    // 1️⃣ Find HTTP body start
    let body_start = match buffer.windows(4).position(|w| w == b"\r\n\r\n") {
        Some(pos) => pos + 4,
        None => {
            respond(stream, "400 BAD REQUEST", "Invalid POST request");
            return;
        }
    };

    println!("Body starts at byte index: {}", body_start);

    let body = &buffer[body_start..];
    let body_str = String::from_utf8_lossy(body);

    /*
        Multipart format:

        --boundary
        headers
        <empty line>
        FILE CONTENT
        --boundary--
    */

    // 2️⃣ Split multipart headers and content
    let mut sections = body_str.split("\r\n\r\n");

    // Skip multipart headers
    sections.next();

    let file_and_boundary = match sections.next() {
        Some(v) => v,
        None => {
            respond(stream, "400 BAD REQUEST", "Invalid multipart data");
            return;
        }
    };

    println!("Extracted file section length: {}", file_and_boundary.len());

    // 3️⃣ Remove trailing boundary
    let file_content = match file_and_boundary.find("\r\n------") {
        Some(idx) => &file_and_boundary[..idx],
        None => file_and_boundary,
    };

    // 4️⃣ Write clean file content
    let mut file = File::create("input.txt").unwrap();
    file.write_all(file_content.as_bytes()).unwrap();
    println!("File saved as input.txt");

    // 5️⃣ Optional: execute Python script
    let output = Command::new("python")
        .arg("speech.py")
        .output()
        .expect("Failed to run Python script");

    if !output.stderr.is_empty() {
        eprintln!("Python error: {}", String::from_utf8_lossy(&output.stderr));
    }

    println!("Python output: {}", String::from_utf8_lossy(&output.stdout));

    respond(stream, "200 OK", "File received and saved");
}

fn respond(mut stream: TcpStream, status: &str, body: &str) {
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        body.len(),
        body
    );

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    start_server();
}
