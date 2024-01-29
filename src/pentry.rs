use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufRead;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordEntry {
    pub service: String,
    pub username: String,
    pub password: String,
}

impl PasswordEntry {
    pub fn new(service: String, username: String, password: String) -> Self {
        PasswordEntry {
            service,
            username,
            password,
        }
    }
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }
    #[allow(dead_code)]
    pub fn user_input() -> Self {
        println!("Enter Service:");
        let mut service = String::new();
        std::io::stdin()
            .read_line(&mut service)
            .expect("Failed to read line");

        println!("Enter Username:");
        let mut username = String::new();
        std::io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");

            println!("Enter Password:");
            let mut password = String::new();
            std::io::stdin()
                .read_line(&mut password)
                .expect("Failed to read line");

            PasswordEntry::new(
                service.trim().to_string(),
                username.trim().to_string(),
                password.trim().to_string(),
            )
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self)
            .expect("Failed to convert to json")
    }
    pub fn write_to_file(&self) {
        let json_output = format!("{}\n", self.to_json());

        match OpenOptions::new()
        .create(true)
        .append(true)
        .open("passwords.json")
        {
            Ok(mut file) => {
                if let Err(e) = file.write_all(json_output.as_bytes()) {
                    eprintln!("Error: {}", e);
                } else {
                    println!("Success");
                }
            }
            Err(e) => eprintln!("Error: {}", e),
            }
            
        }

    }


    pub fn read_passwords_from_file() -> Result<Vec<PasswordEntry>, std::io::Error> {
        let file = File::open("passwords.json")?;
        let reader = BufReader::new(file);
        let mut services = Vec::new();

        for line in reader.lines() {
            if let Ok(json_string) = line {
                if let Ok(service_info) = PasswordEntry::from_json(&json_string) {
                    services.push(service_info);
                }
            }

        }
        Ok(services)
    }

    pub fn prompt(text: &str) -> String {
        println!("{}", text);
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }