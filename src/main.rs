extern crate chrono;
extern crate quoted_printable;

use std::process::{Command, Stdio};
use std::io::prelude::*;
use std::io::{Error, ErrorKind, Result};
use chrono::*;
use quoted_printable::encode;

const FROM_ADDRESS  : &str = "\"NixOS Server\" <no-reply@nixcloud.com>";
const TO_ADDRESS    : &str = "mail@markus-kohlhase.de";

fn create(to: &[String], subject: &str, body: &str) -> String {

    let now = Local::now()
        .format("%d %b %Y %H:%M:%S %z")
        .to_string();

    let subject = format!("=?UTF-8?Q?{}?=", String::from_utf8_lossy(&encode(subject.as_bytes())));

    format!(
       "Date:{date}\r\n\
        From:{from}\r\n\
        To:{to}\r\n\
        Subject:{subject}\r\n\
        MIME-Version: 1.0\r\n\
        Content-Type: text/plain; charset=utf-8\r\n\r\n\
        {body}",
        date    = now.as_str(),
        from    = FROM_ADDRESS,
        to      = to.join(","),
        subject = subject,
        body    = body
     )
}

fn send(mail: &str) -> Result<()> {
    let mut child = Command::new("sendmail")
        .arg("-t")
        .stdin(Stdio::piped())
        .spawn()?;
    child.stdin
        .as_mut()
        .ok_or_else(||Error::new(ErrorKind::Other,"Could not get stdin"))?
        .write_all(&mail.as_bytes())?;
    child.wait_with_output()?;
    Ok(())
}


fn main() {
    let mail = create(&[TO_ADDRESS.into()], "test", "foo");
    println!("sending mail...");
    match send(&mail) {
        Ok(_) => {
            println!("successfully sent mail");
        }
        Err(err) => {
            println!("uupps... {}", err);
        }
    }
}
