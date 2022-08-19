// implement print minimal ticket for TextualObjectMachine

use crate::to::to_struct::TextualObject;
use crate::to_ticket::to_ticket_struct::TextualObjectTicket;

impl TextualObject {
    pub fn print_minimal_ticket(&self) -> String {
        let ticket = TextualObjectTicket::from(self.clone());
        ticket.print_minimal()
    }

    pub fn update_minimal_ticket(&mut self) -> Self {
        let ticket = TextualObjectTicket::from(self.clone());
        let ticket_minimal = ticket.print_minimal();
        self.ticket_minimal = ticket_minimal;
        self.ticket_minimal.clone();
        self.to_owned()
    }
}
