use std::thread;
use std::time::Duration;

use bounded::data::{Status, TicketDraft};
use bounded::launch;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn works() {
    let client = launch(5);
    let draft = TicketDraft {
        title: ticket_title(),
        description: ticket_description(),
    };
    let ticket_id = client.insert(draft.clone());
    loop {
        match ticket_id {
            Ok(msg) => {
                dbg!("got the value {:?}", msg);
                break;
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                dbg!("Empty");
                thread::sleep(Duration::from_millis(200));
                break;
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                dbg!("Disconnected");
                break;
            }
        }
    }

    //    let client2 = client.clone();
    //    let ticket = client2.get(ticket_id).unwrap().unwrap();
    //    assert_eq!(ticket_id, ticket.id);
    //    assert_eq!(ticket.status, Status::ToDo);
    //    assert_eq!(ticket.title, draft.title);
    //    assert_eq!(ticket.description, draft.description);
}
