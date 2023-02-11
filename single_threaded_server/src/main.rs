use std::net::TcpListener;

fn main() {
    // listen to the local address 127.0.0.1:7878 for incoming TCP streams.
    // when it gets an incoming stream, it will print "Connection established!"
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}

