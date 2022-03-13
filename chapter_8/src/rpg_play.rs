use rand;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Dwarf {}

#[derive(Debug)]
struct Elf {}

#[derive(Debug)]
struct Human {}

#[derive(Debug)]
enum Thing {
    Sword,
    Spear,
    Trinket,
}

trait Enchanter: std::fmt::Debug {
    fn competency(&self) -> f64;

    fn enchant(&self, thing: &mut Thing) {
        let probability_of_success = self.competency();
        let spell_is_successful = rand::thread_rng().gen_bool(probability_of_success);

        println!("{self:?} mutters incoherently");

        if spell_is_successful {
            println!("The {thing:?} grows brightly");
        } else {
            println!("The {thing:?} fizzes, then turns into a worthless trinket");
            *thing = Thing::Trinket {}
        }
    }
}

impl Enchanter for Dwarf {
    fn competency(&self) -> f64 {
        0.5
    }
}

impl Enchanter for Elf {
    fn competency(&self) -> f64 {
        0.95
    }
}

impl Enchanter for Human {
    fn competency(&self) -> f64 {
        0.8
    }
}

pub fn rpgplaymain() {
    let mut it = Thing::Spear;

    let d = Dwarf {};
    let e = Elf {};
    let h = Human {};

    let party: Vec<&dyn Enchanter> = vec![&d, &e, &h];
    let spellcenter = party.choose(&mut rand::thread_rng()).unwrap();

    spellcenter.enchant(&mut it);
    spellcenter.enchant(&mut Thing::Sword);
}
