use crate::config::Config;

use sendgrid::errors::SendgridError;
use sendgrid::v3::{Email, Message, Personalization, Sender};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Mailer {
    pub cnfg: Arc<Config>,
    pub sender: Arc<Sender>,
}

pub fn init_mailer(cnfg: &Arc<Config>) -> Arc<Mailer> {
    let sender = Sender::new(cnfg.sendgrid_api_key.clone());
    let client = Mailer {
        cnfg: cnfg.clone(),
        sender: Arc::new(sender),
    };
    Arc::new(client)
}

impl Mailer {
    pub fn send(&self, to: String, template_id: String) -> Result<(), SendgridError> {
        let pln = Personalization::new().add_to(Email::new().set_email(&to));

        let msg = Message::new()
            .set_from(
                Email::new()
                    .set_email(&self.cnfg.sendgrid_email_from)
                    .set_name(&self.cnfg.sendgrid_name_from),
            )
            .set_template_id(&template_id)
            .add_personalization(pln);

        self.sender.send(&msg)?;
        Ok(())
    }
}
