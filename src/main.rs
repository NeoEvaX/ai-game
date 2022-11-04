use ron::de::from_reader;
use std::{io::stdin, collections::HashMap, fs::File};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Node {
    id: i32,
    address: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct GameStructure {
    nodes: HashMap<String, Node>,
}

fn main() {

    let input_path = format!("{}/assets/input.ron", env!("CARGO_MANIFEST_DIR"));
    let f = File::open(&input_path).expect("Failed opening file");
    let config: GameStructure = match from_reader(f) {
        Ok(x) => x,
        Err(e) => {
            println!("Failed to load config: {}", e);

            std::process::exit(1);
        }
    };

    println!("Config: {:?}", &config.nodes["1.1.1.2"].message);

    // println!("Gane Name");
    // let mut new_game = String::new();
    // stdin()
    //     .read_line(&mut new_game)
    //     .expect("Failed to read line");
}

#[cfg(test)]
mod tests {

}