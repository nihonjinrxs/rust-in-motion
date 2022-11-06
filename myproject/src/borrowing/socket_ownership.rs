use std::{
    thread,
    time::Duration,
    net::TcpListener,
};

fn open_socket_for_five_seconds() {
    let _listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    thread::sleep(Duration::from_secs(5));
} // _listener automatically cleaned up at end of function scope

pub fn run() {
    open_socket_for_five_seconds();
    println!("Back in main");
    thread::sleep(Duration::from_secs(5));
}
