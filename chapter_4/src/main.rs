#![allow(unused_variables)]

#[derive(Debug,Clone, Copy)]
struct CubeSat {
    id: u64,
}

#[derive(Debug,Clone, Copy)]
enum StatusMessage {
    Ok,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

struct GroundStation;

impl GroundStation {
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }

    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mail = Mailbox { messages: vec![] };
    let base = GroundStation {};
    let sat_ids = fetch_sat_ids();
    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = Message {
            to: sat_id,
            content: String::from("hello"),
        };
        base.send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.connect(sat_id);
        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
    let sat_a= CubeSat{id: 0};
    let a_status = check_status(sat_a.clone());
    println!("a: {:?},",a_status.clone());

    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn return_id(){
        assert_eq!(fetch_sat_ids(),vec![1,2,3]);
    }
}
