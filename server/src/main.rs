pub extern crate simple_server;
pub extern crate file;
#[macro_use]
pub extern crate lazy_static;
pub extern crate serde_json;
pub extern crate time;
#[macro_use]
pub extern crate serde_derive;
pub extern crate serde;

use simple_server::*;

use std::collections::HashMap;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;
use std::io;
use std::process;
use std::fs;
use std::path::Path;

mod plot;
use plot::*;

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

use std::fs::File;
use std::io::prelude::*;



fn main() {
    let mut file = File::open("felix.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    match serde_json::from_str::<Vec<BuyPost>>(&contents[..]) {
        Ok(posts) => {
            let purchases = posts.into_iter()
                .map(|post| post.into_purchase())
                .collect::<Vec<Purchase>>();

            println!("{:#?}", purchases);
        },
        Err(err) => {
            println!("parse error {:?}", err);
        }
    }

    /*
    let mut data = Database::new(String::from("data.json"));
    data.add_point(Purchase {
        age: Age::UnderThirteen,
        gender: Gender::Female,
        continent: Continent::Europe,
        time: Moment::from_dur(time::Duration::days(1))
    });
    data.add_point(Purchase {
        age: Age::EighteenToThirty,
        gender: Gender::Other,
        continent: Continent::Asia,
        time: Moment::from_dur(time::Duration::days(3))
    });
    println!("response = {}", data.form_response_json());
    */


    /*
    let mut plot = HashMap::new();
    plot.insert("all", vec![(1, 0.4), (4, 0.3), (54, 68.34)]);
    plot.insert("women", vec![(5, 0.3), (7, 465.1), (4, 84.0)]);

    let s = serde_json::to_string(&plot).unwrap();

    println!("{}", s);
    */

    /*
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
    */
}
