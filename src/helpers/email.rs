use crate::config;

use sendgrid::SGClient;
use sendgrid::{Destination, Mail};
use std::error::Error;

// pub type Mailer = Pool<PostgresConnectionManager>;

pub fn init_mailer(cnfg: &config::Config) -> Result<(), Box<Error>> {
    let client = SGClient::new(cnfg.sendgrid_api_key.clone());

    let mail_info = Mail::new()
        .add_to(Destination {
            address: "example@gmail.com",
            name: "example",
        })
        .add_from("support@example.com")
        .add_subject("Test mail")
        .add_text("Test message")
        .add_html("<h1>Test message</h1>")
        .add_from_name("Support");

    match client.send(mail_info) {
        Err(err) => println!("Error: {}", err),
        Ok(body) => println!("Response: {}", body),
    };

    Ok(())
}
