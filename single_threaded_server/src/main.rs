use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // Result may be throw an error if the data isn't valid UTF-8, or if there 
    // was a problem reading from the stream
    // This is an ungraceful method of handling the possible errors.
    let http_request: Vec<_> = buf_reader
        .lines() // lines method will split the stream of data whenver it sees a newline byte
        .map(|result| result.unwrap()) // map and unwrap each result to get each string
        .take_while(|line| !line.is_empty()) // identify end of the HTTP request by finding the
                                             // first empty string
        .collect();

    println!("Request: {:#?}", http_request);
}
