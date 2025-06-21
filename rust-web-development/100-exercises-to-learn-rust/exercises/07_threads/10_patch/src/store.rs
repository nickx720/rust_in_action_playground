use crate::data::{Status, Ticket, TicketDraft, TicketPatch};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore {
    tickets: BTreeMap<TicketId, Ticket>,
    counter: u64,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: BTreeMap::new(),
            counter: 0,
        }
    }

    pub fn add_ticket(&mut self, ticket: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket {
            id,
            title: ticket.title,
            description: ticket.description,
            status: Status::ToDo,
        };
        self.tickets.insert(id, ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }

    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }
    pub fn update_mut(&mut self, id: TicketId, patch: TicketPatch) -> Result<(), String> {
        let ticket = self.tickets.get_mut(&id);
        if let Some(ticket) = ticket {
            let title;
            let description;
            let status;
            if let Some(old_title) = patch.title {
                title = old_title;
            } else {
                title = ticket.title.clone();
            }
            if let Some(old_desc) = patch.description {
                description = old_desc;
            } else {
                description = ticket.description.clone();
            }
            if let Some(old_status) = patch.status {
                status = old_status;
            } else {
                status = ticket.status;
            }
            ticket.title = title;
            ticket.description = description;
            ticket.status = status;
            Ok(())
        } else {
            return Err("Not found".to_string());
        }
    }
}
