mod dns_resolver;
mod mac_play;
mod reqwest_play;
mod rpg_play;
use dns_resolver::main_dns;
use mac_play::mac_main;
use reqwest_play::reqwestmain;
use rpg_play::rpgplaymain;

fn main() {
    //    reqwestmain();
    //rpgplaymain();
    //dns_resolver::main_dns();
    mac_main();
}
