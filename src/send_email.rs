use std::fs;

use lettre::message::{header, Attachment, Body, Message, MultiPart, SinglePart};
use lettre::transport::smtp::response::Response;
use lettre::{
    transport::smtp::{
        authentication::{Credentials, Mechanism},
        PoolConfig,
    },
    SmtpTransport, Transport,
};


pub fn send_mail(to: &str, data: Vec<u8>) -> Result<Response, lettre::transport::smtp::Error> {
    let m = Message::builder()
    .from("Kvantomat <nchk-kvantomat@yandex.ru>".parse().unwrap())
    .to(format!("User <{}>", to).parse().unwrap())
    .subject("Student List")
    .multipart(
        MultiPart::mixed()
            .singlepart(Attachment::new(String::from("list.xlsx")).body(
                data,
                "application/vnd.ms-excel".parse().unwrap(),
            )),
    ).unwrap();

    // Create TLS transport on port 587 with STARTTLS
    let sender = SmtpTransport::starttls_relay("smtp.ya.ru").unwrap()
    // Add credentials for authentication
    .credentials(Credentials::new(
        "nchk-kvantomat@yandex.ru".to_owned(),
        "ikwzkllrqrvdhjwz".to_owned(),
    ))
    // Configure expected authentication mechanism
    .authentication(vec![Mechanism::Plain])
    // Connection pool settings
    .pool_config(PoolConfig::new().max_size(20))
    .build();

    sender.send(&m)
}
