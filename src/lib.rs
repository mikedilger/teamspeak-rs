
#[cfg(test)]
mod tests;

use std::net::{TcpStream, Shutdown};
use std::io::Error as IoError;

fn ts3_escape(s: &str) -> String
{
    let mut output: String = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        match c {
            '\\' => output.push_str(r"\\"),
            '/' => output.push_str(r"\/"),
            ' ' => output.push_str(r"\s"),
            '|' => output.push_str(r"\p"),
            '\x07' => output.push_str(r"\a"),
            '\x08' => output.push_str(r"\b"),
            '\x0C' => output.push_str(r"\f"),
            '\x0A' => output.push_str(r"\n"),
            '\x0D' => output.push_str(r"\r"),
            '\x09' => output.push_str(r"\t"),
            '\x0B' => output.push_str(r"\v"),
            c => output.push(c),
        }
    }
    output
}

pub struct TeamSpeak {
    host: String,
    port: u16,
    user: String,
    password: String,
    stream: Option<TcpStream>,
}

impl TeamSpeak {
    pub fn new(host: &str, port: u16, user: &str, password: &str) -> TeamSpeak
    {
        TeamSpeak {
            host: host.to_owned(),
            port: port,
            user: user.to_owned(),
            password: password.to_owned(),
            stream: None
        }
    }

    pub fn connect(&mut self) -> Result<(), IoError>
    {
        if self.stream.is_some() {
            try!(self.stream.as_ref().unwrap().shutdown(Shutdown::Both));
        }

        self.stream = Some( try!(TcpStream::connect(
            &*format!("{}:{}", self.host, self.port))) );

        // FIXME: login

        Ok(())
    }
}
