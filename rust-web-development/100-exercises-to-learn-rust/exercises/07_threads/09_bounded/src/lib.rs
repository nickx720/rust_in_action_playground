// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, RecvError, SyncSender, TryRecvError};
use std::thread;
use std::time::Duration;

pub mod data;
pub mod store;

#[derive(Clone, Debug)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
    capacity: usize,
}
// TODO insert try_recv says empty
// then send for insert failed: "Disconnected"?
impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, RecvError> {
        let (sender, receiver) = sync_channel(self.capacity);
        let command = Command::Insert {
            draft,
            response_channel: sender,
        };
        self.sender.try_send(command).unwrap();
        receiver.recv()
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, RecvError> {
        let (sender, receiver) = sync_channel(self.capacity);
        let command = Command::Get {
            id,
            response_channel: sender,
        };
        self.sender.try_send(command).expect("Something went wrong");
        receiver.recv()
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender, capacity }
}

#[derive(Clone)]
pub enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: SyncSender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: SyncSender<Option<Ticket>>,
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
                response_channel
                    .try_send(id)
                    .expect("Send for Insert failed");
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id).unwrap().to_owned();
                response_channel
                    .try_send(Some(ticket))
                    .expect("Send for Get failed")
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break;
            }
        }
    }
}
