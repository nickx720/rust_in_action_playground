use std::collections::HashMap;

use serde_json::json;

pub fn main() {
    let mut capitals = HashMap::new();

    capitals.insert("Cook Islands", "Avarua");
    capitals.insert("Fiji", "Suva");
    capitals.insert("Kiribati", "South Tarawa");
    capitals.insert("Niue", "Alofi");
    capitals.insert("Tonga", "Nuku alofa");
    capitals.insert("Tuvalu", "Punfafuti");

    let tongan_captal = capitals["Tonga"];

    println!("Capital of Tonga is {tongan_captal}");

    let serde_capital = json!({
        "Cook Islands": "Avarua",
        "Fiji": "Suva",
        "Kiribati": "South Tarawa",
        "Niue": "Alofi",
        "Tonga": "Nuku alofa",
        "Tuvalu": "Punfafuti"
    });
    let capital = serde_capital["Tonga"].clone();
    println!("Capital of Tonga is {capital}");
}
