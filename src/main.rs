use std::net::TcpListener;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4545").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");
    }
}
