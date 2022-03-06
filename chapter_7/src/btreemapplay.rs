use std::collections::BTreeMap;

pub fn main() {
    let mut voc = BTreeMap::new();

    voc.insert(3_697_915, "Amsterdam");
    voc.insert(1_300_405, "Middleburg");
    voc.insert(540_000, "Enkuizen");
    voc.insert(469_400, "Delft");
    voc.insert(266_868, "Noorn");
    voc.insert(173_000, "Rotterdam");

    for (guilders, kamer) in &voc {
        println!("{kamer} invested {guilders}");
    }

    print!("Smaller chambers: ");
    for (_guilders, kamer) in voc.range(0..500_000) {
        print!("{kamer} ");
    }
    println!("");
}
