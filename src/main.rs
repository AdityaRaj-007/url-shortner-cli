use std::{collections::HashMap, fs::{self, File}, io::Write, path::Path};
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct UrlShortner {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Shorten {
        #[arg(short , long)]
        url :String
    }, 
    Resolve {
        #[arg(short, long)]
        url: String
    }
}

fn save_url_data(data: HashMap<String, String>) {
    let path = Path::new("urls.json");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Unable to open {} in write mode: {}", display, why),
        Ok(file) => file
    };

    let serialized_data = serde_json::to_string_pretty(&data).unwrap();

    match file.write_all(serialized_data.as_bytes()) {
        Err(why) => panic!("Unable to write data in {} : {}", display, why),
        Ok(_) => println!("Urls saved successfully")
    }
}

fn get_all_urls() -> HashMap<String,String> {
    let path = Path::new("urls.json");
    if !path.exists() {
        return HashMap::new();
    }

    let data = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(why) => panic!("Unable to read data from {} : {}", path.display(), why)
    };
    
    return serde_json::from_str(&data).unwrap_or_else(|_| HashMap::new())
}

fn generate_random_id() -> String {
    let chars = String::from("abcdefghijklmnopqrstuvwxyz012223456789_");

    let mut short_id = String::from("");

    for _ in 0..6 {
        let j = fastrand::usize(..chars.len());
        let ch = chars.chars().nth(j).unwrap();
        short_id.push(ch);
    } 

    return short_id
}
fn main() {
    let cli = UrlShortner::parse();

    let mut urls:HashMap<String, String> = get_all_urls();

    match &cli.command {
        Commands::Shorten { url } => {
            println!("Command is to shorten the URL...");
            let contains_key = urls.contains_key(url);
            let mut new_url = String::from("https://urlshortnercli/");

            if !contains_key {
                let short_id = generate_random_id();
                urls.insert(url.to_string(), short_id);
            } 
            let key = urls.get(url);
            match key {
                Some(short_id) => new_url.push_str(short_id),
                None => println!("key doesn't exists")
            }          

            println!("New url for {}: {}", url, new_url);

            save_url_data(urls);
        },
        Commands::Resolve { url } => {
            println!("Command is used to get the original URL...");
            
            let mut short_id = String::from("");

            for char in url.chars().rev() {
                if char != '/' {
                    short_id.push(char);
                } else {
                    break;
                }
            }

            println!("{}", short_id);

            let original_url = urls.iter().find_map(|(key, val)| if *val == short_id.chars().rev().collect::<String>() {Some(key)} else {None});

            match original_url {
                Some(url) => println!("Original url {}",url),
                None => println!("Not a valid url")
            }
        }
    }
}
