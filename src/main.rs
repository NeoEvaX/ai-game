use directories::ProjectDirs;
use std::{io::stdin, collections::HashMap};

struct Node {
    id: i32,
    address: String,
    message: String,
}

fn get_project_directory() -> String {
    // Lin: /home/username/.config/initiativetracker"
    // Win: C:\Users\Username\AppData\Roaming\Nex-Verse\Initiative Tracker\config
    // Mac: /Users/username/Library/Application Support/com.Nex-Verse.Initiative-Tracker
    let proj_dirs = ProjectDirs::from("com", "Nex-Verse", "Initiative Tracker");
    proj_dirs.unwrap().config_dir().display().to_string()
}

fn main() {
    let mut game_nodes = HashMap::new();

    for node in nodes {
        game_nodes.insert(node.unwrap().address, node.unwrap());
    }


    println!("Gane Name");
    let mut new_game = String::new();
    stdin()
        .read_line(&mut new_game)
        .expect("Failed to read line");
}

#[cfg(test)]
mod tests {

}