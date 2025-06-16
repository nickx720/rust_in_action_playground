// TODO: Convert the implementation to use bounded channels.
use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError};

pub mod data;
pub mod store;

#[derive(Clone)]
pub struct TicketStoreClient {
    sender: SyncSender<Command>,
    capacity: usize,
}
// TODO insert try_recv says empty
// then send for insert failed: "Disconnected"?
impl TicketStoreClient {
    pub fn insert(&self, draft: TicketDraft) -> Result<TicketId, TryRecvError> {
        let (sender, receiver) = sync_channel(self.capacity);
        let command = Command::Insert {
            draft,
            response_channel: sender,
        };
        self.sender.try_send(command).expect("Something went wrong");
        receiver.try_recv()
    }

    pub fn get(&self, id: TicketId) -> Result<Option<Ticket>, TryRecvError> {
        let (sender, receiver) = sync_channel(self.capacity);
        let command = Command::Get {
            id,
            response_channel: sender,
        };
        self.sender.try_send(command).expect("Something went wrong");
        receiver.try_recv()
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender, capacity }
}

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
        match receiver.try_recv() {
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
