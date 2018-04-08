
use time::Duration;
use std::collections::HashMap;
use serde_json;

use std::path::Path;
use std::mem;
use std::fs::File;
use std::io::prelude::*;

//type Plot = HashMap<String, Vec<(u64, f64)>>;

// exact mirror of time::Duration
#[derive(Clone, Serialize, Deserialize)]
pub struct Moment {
    secs: i64,
    nanos: i32,
}
impl Moment {
    pub fn from_dur(t: Duration) -> Moment {
        unsafe {
            mem::transmute(t)
        }
    }

    pub fn into_dur(self) -> Duration {
        unsafe {
            mem::transmute(self)
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Age {
    UnderThirteen,
    ThirteenToEighteen,
    EighteenToThirty,
    ThirtyToFifty,
    FiftyAndOlder
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Gender {
    Female,
    Male,
    Other
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Continent {
    NorthAmerica,
    SouthAmerica,
    Europe,
    Africa,
    Asia,
    Austrilia,
    Antarctica
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Purchase {
    pub age: Age,
    pub gender: Gender,
    pub continent: Continent,
    pub time: Moment
}

pub struct Database {
    points: Vec<Purchase>,
    path: Path
}

pub struct XY {
    x: f64,
    y: f64
}

pub type Response = HashMap<String, HashMap<String, Vec<XY>>>;

impl Database {
    pub fn new(path: Path) -> Database {
        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents);

                match serde_json::from_str::<Vec<Purchase>>(contents) {
                    Ok(points) => {
                        println!("read database");
                        Database {
                            points,
                            path
                        }
                    },
                    Err(err) => {
                        println!("failed to parse database json: {:?}", err);
                        Database {
                            points: Vec::new(),
                            path
                        }
                    }
                }
            },
            Err(_) => {
                println!("database not found");
                Database {
                    points: Vec::new(),
                    path
                }
            }
        }
    }

    pub fn save(&self) {
        match File::create(self.path) {
            Ok(mut file) => {
                let str = serde_json::to_string(&self.points).unwrap();
                file.write_all(str.as_bytes());
            }
        }
    }

    pub fn close(self) {
        mem::drop(self)
    }

    pub fn add_point(&mut self, point: Purchase) {
        self.points.push(point);
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self.points).unwrap()
    }

    pub fn from_string(str: String) -> Database {

    }
}

impl Drop for Database {
    fn drop(&mut self) {
        self.save()
    }
}