use std::{collections::HashMap, fs::{self, File}, io::Write, path::Path};
use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};

#[derive(Parser)]
struct UrlShortner {
    #[command(subcommand)]
    command: Commands
}

#[derive(Serialize, Deserialize, Debug)]
struct UrlData {
    original_url:String,
    visits: u64,
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

fn save_url_data(data: HashMap<String, UrlData>) {
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

fn get_all_short_to_long_urls() -> HashMap<String,UrlData> {
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

fn get_all_long_to_short_urls() -> HashMap<String, String> {
    let path = Path::new("urls.json");

    if !path.exists() {
        return HashMap::new();
    }

    let data = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(why) => panic!("Unable to read data from {} : {}", path.display(), why)
    };

    let urls_data: HashMap<String, UrlData> = serde_json::from_str(&data).unwrap();

    let mut long_to_short = HashMap::new();

    for (short, url_data) in &urls_data {
        long_to_short.insert(url_data.original_url.clone(),short.clone());
    }

    return long_to_short
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

    let mut short_to_long:HashMap<String, UrlData> = get_all_short_to_long_urls();
    let mut long_to_short:HashMap<String, String> = get_all_long_to_short_urls();

    match &cli.command {
        Commands::Shorten { url } => {
            println!("Command is to shorten the URL...");
            let contains_key = long_to_short.contains_key(url);
            let mut new_url = String::from("https://urlshortnercli/");

            if !contains_key {
                let short_id = generate_random_id();
                let url_data = UrlData {
                    original_url: url.to_string(),
                    visits: 0,
                };
                short_to_long.insert(short_id.clone(), url_data);
                long_to_short.insert(url.to_string(),short_id);
                save_url_data(short_to_long);
            } 
            let key = long_to_short.get(url);
            match key {
                Some(short_id) => new_url.push_str(short_id),
                None => println!("key doesn't exists")
            }          

            println!("New url for {}: {}", url, new_url);            
        },
        Commands::Resolve { url } => {
            println!("Command is used to get the original URL...");
            
            let short_id = url.split("/").last().unwrap_or("");

            println!("{}", short_id);

            let url_data = short_to_long.get(short_id);

            match url_data {
                Some(data) => println!("Original url {}",data.original_url),
                None => println!("Not a valid url")
            }
        }
    }
}
