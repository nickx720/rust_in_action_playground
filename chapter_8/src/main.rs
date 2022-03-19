mod dns_resolver;
mod reqwest_play;
mod rpg_play;
use dns_resolver::main_dns;
use reqwest_play::reqwestmain;
use rpg_play::rpgplaymain;

fn main() {
    //    reqwestmain();
    //rpgplaymain();
    dns_resolver::main_dns();
}
