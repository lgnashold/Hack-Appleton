pub extern crate simple_server;
pub extern crate file;
#[macro_use]
pub extern crate lazy_static;
pub extern crate serde_json;
pub extern crate time;
#[macro_use]
pub extern crate serde_derive;
pub extern crate serde;
pub extern crate walkdir;

use simple_server::*;

use walkdir::WalkDir;

use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, Mutex, MutexGuard};
use std::thread;
use std::io;
use std::process;
use std::fs;
use std::path::Path;

use std::fs::File;
use std::io::prelude::*;

mod plot;
use plot::*;

lazy_static! {
    static ref PAGES: RwLock<HashMap<String, Vec<u8>>> = RwLock::new(HashMap::new());
}

lazy_static! {
    static ref DATABASE: Mutex<Database> = Mutex::new(Database::new(String::from("data.json")));
}

fn load_pages() -> Result<(), String> {
    let mut pages: RwLockWriteGuard<HashMap<String, Vec<u8>>> = PAGES.write().unwrap();

    match file::get("html/index.html") {
        Ok(page) => pages.insert("/".to_owned(), page),
        Err(err) => return Err(format!("fail to read index.html: {:?}", err))
    };

    for entry in WalkDir::new("html") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let mut file = File::open(entry.path()).unwrap();
            let length = file.metadata().unwrap().len() as usize;
            let data: Vec<u8> = file
                .bytes()
                .take(length)
                .map(|r: Result<u8, _>| r.unwrap())
                .collect();

            let path = entry.path().clone().to_str().unwrap().clone();
            let url = String::from(&path[4..]);

            println!("found url {}", url);

            pages.insert(url, data);
        }
    }

    /*
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
    */


    Ok(())
}

fn get_response() -> String {
    let mut db: MutexGuard<Database> = DATABASE.lock().unwrap();
    db.form_response_json()
}

fn post_purchase(post: String) {
    match serde_json::from_str::<BuyPost>(&post[..]) {
        Ok(parsed) => {
            println!("parsed: {}", parsed);
            let purchase = parsed.into_purchase();
            let mut db: MutexGuard<Database> = DATABASE.lock().unwrap();
            db.add_point(purchase);
        },
        Err(err) => {
            println!("post purchase parse error {:?}", err);
        }
    };
}

fn test_felix_parse_respond() {
    let mut file = File::open("felix.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    match serde_json::from_str::<Vec<BuyPost>>(&contents[..]) {
        Ok(posts) => {
            let purchases = posts.into_iter()
                .map(|post| post.into_purchase())
                .collect::<Vec<Purchase>>();

            let mut data = Database::new(String::from("data.json"));
            data.clear();
            for purchase in purchases {
                data.add_point(purchase);
            }

            let response = data.form_response();
            data.close();
            println!("response = {:#?}", response);
            let json = serde_json::to_string(&response).unwrap();
            println!("{}", json);
        },
        Err(err) => {
            println!("parse error {:?}", err);
        }
    }
}

fn main() {
    //let host = "127.0.0.1";
    let host = "localhost";
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
                    "/plots.json" => {
                        Ok(response.body(get_response().into_bytes())?)
                    }
                    "/buy" => {
                        let body = request.body();
                        let body_str = String::frmo_utf8_lossy(body);
                        post_purchase(body_str);
                        Ok(response.body("post received!".to_owned().into_bytes())?)
                    }
                    /*
                    s if s.len() >= 3 && &s[0..3] == "buy" => {
                        let json = String::from(&s[3..]);
                        println!("post purchase {}", json);
                        post_purchase(json);
                        Ok(response.body("post received!".to_owned().into_bytes())?)
                    }
                    */
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
            }
            &Method::POST => {
                let data = String::from_utf8_lossy(request.body()).into_owned();
                let body = format!("you posted \"{}\"", data);
                println!("{}", body);
                post_purchase(data);
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
