use std::io;
use chrono::prelude::*;
fn main() {
    let lines = io::stdin().lines();
    for line in lines {
        println!("[{}] {}", Utc::now().to_rfc2822(), line.unwrap());
    }
}


