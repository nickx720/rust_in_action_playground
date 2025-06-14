// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, Sender, SyncSender};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, todo!()> {
        todo!()
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, todo!()> {
        todo!()
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(10);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: todo!(),
    },
    Get {
        id: TicketId,
        response_channel: todo!(),
    },
}

pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                todo!()
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                todo!()
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
