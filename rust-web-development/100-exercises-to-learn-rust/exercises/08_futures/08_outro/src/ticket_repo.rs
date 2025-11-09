use std::{future::Future, sync::Arc};

use ticket_fields::{TicketDescription, TicketTitle};
use tokio::sync::Mutex;

use crate::{
    ticket::{Ticket, TicketDraft, TicketId, TicketStore},
    TicketPatch,
};

pub struct TicketModel {
    ticket: Arc<Mutex<TicketStore>>,
}

impl TicketModel {
    pub fn new(ticket: Arc<Mutex<TicketStore>>) -> Self {
        Self { ticket }
    }
}
pub trait TicketRepo {
    fn add<'a>(&'a mut self, draft: TicketDraft) -> impl Future<Output = TicketId> + 'a; // RPITIT: keep trait as `fn`, return `impl Future<Output = ...> + 'a`; implementors return an `async move { ... }` block. `'a` ties the future to the borrow of `&'a mut self`. Callers use `.await`. See: Rust Reference "return-position impl Trait in traits" and `std::future::Future`.
    fn update<'a>(
        &'a mut self,
        id: TicketId,
        patch: TicketPatch,
    ) -> impl Future<Output = Option<Ticket>> + 'a;
    fn read<'a>(&'a mut self, id: TicketId) -> impl Future<Output = Ticket> + 'a;
}

impl TicketRepo for TicketModel {
    async fn add(&mut self, draft: TicketDraft) -> TicketId {
        let mut ticket = self.ticket.lock().await;
        let ticket_id = ticket.add_ticket(draft);
        ticket_id
    }
    async fn update(&mut self, id: TicketId, patch: TicketPatch) -> Option<Ticket> {
        let mut ticket_instance = self.ticket.lock().await;
        let ticket = ticket_instance.get_mut(id).unwrap();
        if let Some(items) = patch.common {
            ticket.title = TicketTitle::try_from(items.title).unwrap();
            ticket.description = TicketDescription::try_from(items.description).unwrap();
        } else {
            return None;
        }
        if let Some(status) = patch.status {
            ticket.status = status;
        } else {
            return None;
        }
        let ticket = ticket.clone();
        Some(ticket)
    }
    async fn read(&mut self, id: TicketId) -> Ticket {
        let ticket = self.ticket.lock().await;
        ticket.get(id).expect("No ticket found").clone()
    }
}
