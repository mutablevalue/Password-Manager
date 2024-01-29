pub mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords_from_file;
use crate::pentry::PasswordEntry;
use std::io::BufReader;



fn clr() {
    print!("\x1B[2J\x1B[1;1H");
}

fn main() {
    clr();
    let ascii = r#" 
    _____                    _____                   _______                  _______                   _____          
    /\    \                  /\    \                 /::\    \                /::\    \                 /\    \         
   /::\    \                /::\    \               /::::\    \              /::::\    \               /::\____\        
  /::::\    \              /::::\    \             /::::::\    \            /::::::\    \             /:::/    /        
 /::::::\    \            /::::::\    \           /::::::::\    \          /::::::::\    \           /:::/    /         
/:::/\:::\    \          /:::/\:::\    \         /:::/~~\:::\    \        /:::/~~\:::\    \         /:::/    /          
/:::/  \:::\    \        /:::/__\:::\    \       /:::/    \:::\    \      /:::/    \:::\    \       /:::/____/           
/:::/    \:::\    \      /::::\   \:::\    \     /:::/    / \:::\    \    /:::/    / \:::\    \      |::|    |            
/:::/    / \:::\    \    /::::::\   \:::\    \   /:::/____/   \:::\____\  /:::/____/   \:::\____\     |::|    |     _____  
/:::/    /   \:::\ ___\  /:::/\:::\   \:::\____\ |:::|    |     |:::|    ||:::|    |     |:::|    |    |::|    |    /\    \ 
/:::/____/  ___\:::|    |/:::/  \:::\   \:::|    ||:::|____|     |:::|    ||:::|____|     |:::|    |    |::|    |   /::\____\
\:::\    \ /\  /:::|____|\::/   |::::\  /:::|____| \:::\    \   /:::/    /  \:::\    \   /:::/    /     |::|    |  /:::/    /
\:::\    /::\ \::/    /  \/____|:::::\/:::/    /   \:::\    \ /:::/    /    \:::\    \ /:::/    /      |::|    | /:::/    / 
\:::\   \:::\ \/____/         |:::::::::/    /     \:::\    /:::/    /      \:::\    /:::/    /       |::|____|/:::/    /  
\:::\   \:::\____\           |::|\::::/    /       \:::\__/:::/    /        \:::\__/:::/    /        |:::::::::::/    /   
\:::\  /:::/    /           |::| \::/____/         \::::::::/    /          \::::::::/    /         \::::::::::/____/    
\:::\/:::/    /            |::|  ~|                \::::::/    /            \::::::/    /           ~~~~~~~~~~          
 \::::::/    /             |::|   |                 \::::/    /              \::::/    /                                
  \::::/    /              \::|   |                  \::/____/                \::/____/                                 
   \::/____/                \:|   |                   ~~                       ~~                                       
                             \|___|                                                                                     
                                                                                                                         
    "#;
    println!("{}", ascii);
    loop {
        println!("Menu");
        println!("Add Password");
        println!("View Password");
        println!("Search Password");
        println!("Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();


        match choice.trim(){
            "1" => {
                clr();

                let entry = pentry::PasswordEntry::new(
                prompt("Service :"),
                prompt("Username :"),
                prompt("Password :")

                );
                println!("Success");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    Vec::new()
                });
                for service in &services {
                    println!(
                        "Service: {}, Username: {}, Password: {}",
                        service.service, service.username, service.password
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error: {}", err);
                    Vec::new()
                });
                let search = prompt("Search: ");
                for service in &services {
                    if service.service.as_str().contains(&search) {
                        println!(
                            "Service: {}, Username: {}, Password: {}",
                            service.service, service.username, service.password
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Exiting");
                break;
            }
            _ => println!("Invalid choice"),
            }
            println!("\n\n");
    }
    let passwords = read_passwords_from_file();
}
