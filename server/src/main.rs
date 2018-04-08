extern crate simple_server;
extern crate file;
#[macro_use]
extern crate lazy_static;

use simple_server::*;

use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
use std::io;
use std::process;
use std::fs;

lazy_static! {
    static ref PAGES: RwLock<HashMap<String, Vec<u8>>> = RwLock::new(HashMap::new());
}

fn load_pages() -> Result<(), String> {
    let mut pages: RwLockWriteGuard<HashMap<String, Vec<u8>>> = PAGES.write().unwrap();

    match file::get("html/index.html") {
        Ok(page) => pages.insert("/".to_owned(), page),
        Err(_) => return Err("fail to read index.html".to_owned())
    };

    for path in fs::read_dir("html").unwrap()
        .map(|r| r.unwrap().path()) {
        println!("reading path {:?}", path);

        let str = path.clone().into_os_string().into_string().unwrap();
        let data = file::get(&str).expect("page data read error");
        let url = format!("/{}", path.file_name().unwrap().to_os_string().into_string().unwrap());
        pages.insert(url, data);
    }


    Ok(())
}

fn main() {
    let host = "127.0.0.1";
    let port = "80";

    load_pages().expect("failed to load pages");

    thread::spawn(move || {
        loop {
            let mut line = String::new();
            io::stdin().read_line(&mut line)
                .expect("stdin read failure");
            match line.trim() {
                "exit" => {
                    println!("exiting");
                    process::exit(0);
                },
                "refresh" => {
                    println!("refreshing pages... {:?}", load_pages());

                },
                _ => {
                    println!("unknown command {}", line);
                }
            }
        }
    });

    let server = Server::new(|request, mut response| {
        println!("request received: {} {}", request.method(), request.uri());

        match request.method() {
            &Method::GET => {
                match request.uri().path() {
                    //"/" => Ok(response.body(INDEX.to_owned().into_bytes())?),
                    path => {
                        let pages: RwLockReadGuard<HashMap<String, Vec<u8>>> = PAGES.read().unwrap();
                        match pages.get(path) {
                            Some(page) => Ok(response.body(page.clone())?),
                            None => {
                                response.status(StatusCode::NOT_FOUND);
                                Ok(response.body("404 not found".to_owned().into_bytes())?)
                            }
                        }
                    }
                }
                //let body = format!("you requested path \"{}\"", request.uri().path());
                //Ok(response.body(body.into_bytes())?)
            }
            &Method::POST => {
                let data = String::from_utf8_lossy(request.body()).into_owned();
                let body = format!("you posted \"{}\"", data);
                Ok(response.body(body.into_bytes())?)
            }
            _ => {
                response.status(StatusCode::NOT_FOUND);
                Ok(response.body("404 not found".to_owned().into_bytes())?)
            }
        }
    });

    server.listen(host, port);
}
