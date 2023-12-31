use std::{env, fs, net::TcpListener};

use socketry::*;

/// Accept
fn handle_conn(v: Vec<u8>) -> Res<Vec<u8>> {
    let path = v.iter().map(|u| *u as char).collect::<String>();
    println!("FTP request for {path} received.");
    match fs::read(path) {
        Ok(x) => Ok(x),
        Err(e) => Ok(e.to_string().as_bytes().to_vec()),
    }
}

fn main() -> Null {
    env::set_var("RUST_LOG", "debug"); // yay for debugging

    let tp = Threadpool::new(8); // From socketry
    let serv_sock = makesock()?;
    println!("Server running.");
    for conn in serv_sock.incoming().flatten() {
        tp.task(TaskPair::new(conn, handle_conn))?;
    }

    Ok(())
}

/// Bind
fn makesock() -> Res<TcpListener> {
    Ok(TcpListener::bind("127.0.0.1:7777")?)
}
