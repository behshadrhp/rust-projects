use std::fs;
use std::io;

use reqwest;


#[tokio::main]
async fn main() -> io::Result<()> {

    println!("--- Admin Panel Finder ---");
    println!("--------------------------");

    let routers = fs::read_to_string("./src/directory_list.txt").expect("can't read file");

    println!("Number of routes being tested : {}", routers.len());

    for line in routers.lines() {
        
        let mut url = "https://google.com/".to_string();
        url.push_str(line);

        let response = reqwest::get(url).await.expect("Unable to send request.");

        if response.status().is_success() {
            println!("Trusted admin panel : {}", response.url());
        }
    }

    Ok(())
}
