
extern crate regex;
#[macro_use] extern crate lazy_static;

#[cfg(test)]
mod tests;

use std::net::{TcpStream, Shutdown};
use std::io;
use std::io::{Read,Write};
use regex::Regex;

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

pub struct TsResult {
    id: u32,
    message: String,
}
impl TsResult {
    pub fn read<R: Read>(r: &mut R) -> Result<TsResult, io::Error>
    {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"error\s+id=(\S*)\s+msg=([^\r\n]*)[\r\n]").unwrap();
        }

        // Read the response
        let mut output: Vec<u8> = Vec::new();
        let _count = try!(r.read_to_end(&mut output));
        let s = String::from_utf8_lossy(&output);

        // Parse the response (for example: " error id=0 msg=ok")
        match RE.captures_iter(&s).next() {
            None => Err(io::Error::new(io::ErrorKind::Other, "Response not expected")),
            Some(cap) => {
                if let Ok(i) = cap.at(1).unwrap().parse::<u32>() {
                    Ok( TsResult {
                        id: i,
                        message: cap.at(2).unwrap().to_owned(),
                    })
                } else {
                    Ok( TsResult {
                        id: 4294967295,
                        message: cap.at(2).unwrap().to_owned(),
                    })
                }
            }
        }
    }
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

    pub fn connect(&mut self) -> Result<(), io::Error>
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

impl Drop for TeamSpeak {
    fn drop(&mut self) {
        if self.stream.is_some() {
            // FIXME: Send quit command first.

            let _ = self.stream.as_ref().unwrap().shutdown(Shutdown::Both);
        }
    }
}
