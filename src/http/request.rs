use std::{
    io::{
        Result,
        BufRead, BufReader,
        Read
    },
    net::TcpStream,

};
use std::fmt::{Display, Formatter};
use std::io::Write;
use crate::http::{
    Response,
    Method,
    Status,
    Header
};
use regex as re;


pub struct Request {
    pub method: Method,
    pub url: String,
    pub version: String,
    pub headers: Vec<Header>,
    pub body: String,

    stream: TcpStream
}


impl Request {
    pub fn respond(&mut self, response: Response) -> Result<()> {
        // Format the headers into strings
        let mut headers: Vec<String> = Vec::new();
        for header in response.headers.iter() {
            headers.push(format!("{key}: {value}", key=header.key(),value=header.value()));
        }

        // Write the response text
        let response_text: String = format!(
            "HTTP/{version} {status_int} {status}\r\n{headers}\r\n\r\n{body}",
            version=self.version,
            status_int=response.status as u16,
            status=response.status,
            headers=headers.join("\r\n"),
            body=response.body
        );

        // Write the response
        self.stream.write(response_text.as_bytes())?;
        Ok(())
    }
}

impl From<TcpStream> for Request {
    fn from(value: TcpStream) -> Self {
        // Create placeholder values
        let method: Method;
        let mut url: String;
        let version: String;

        let mut headers: Vec<Header> = Vec::new();
        let mut content_length: usize = 0;

        // Read the stream into a BufReader
        let mut reader: BufReader<&TcpStream> = BufReader::new(&value);

        // Read the first line of the request
        let mut first_line: String = String::default();
        match reader.read_line(&mut first_line) {
            Ok(_) => (),
            Err(e) => panic!("Unable to read in first line of request: {e}"),
        };

        // Interpret the first line
        let request_line_regex: re::Regex = re::Regex::new(
            r"^(?<method>GET|HEAD|POST|PUT|DELETE|CONNECT|OPTIONS|TRACE|PATCH)\s(?<url>[a-zA-Z0-9/%\.~_:?#\[\]@!$&'()*+,;=-]+)\sHTTP/(?<version>\d\.\d)\r?\n?$"
        ).unwrap();

        match first_line.is_empty() {
            true => {
                method = Method::GET;
                url = String::from("/");
                version = String::from("1.1")
            },
            false => {
                let captures = match request_line_regex.captures(first_line.as_str()) {
                    Some(captures) => captures,
                    None => panic!("Unable to parse the first line of the request: \"{first_line}\"")
                };

                method = Method::from(&captures["method"]);
                url = captures["url"].to_string();
                version = captures["version"].to_string();
            }
        };
        
        // If the URL does not end with a slash, add it
        if !url.starts_with('/') {
            url += "/";
        }

        // Interpret the headers
        loop {
            // Read in the line
            let mut line: String = String::default();
            match reader.read_line(&mut line) {
                Ok(_) => (),
                Err(e) => panic!("Unable to read in header line: {e}")
            }

            // Test if the line is empty
            if line.as_str() == "\r\n" {
                break;
            }

            // Parse the header
            let header: Header = Header::from(line);
            if header.key().as_str() == "Content-Length" {
                content_length = header.value().parse::<usize>().unwrap_or(0);
            }
        }

        // Read the body
        let mut body: Vec<u8> = vec![0u8; content_length];

        match reader.read_exact(&mut body) {
            Ok(_) => (),
            Err(e) => panic!("Unable to read body of request: {e}")
        };

        let body: String = match String::from_utf8(body) {
            Ok(s) => s,
            Err(e) => panic!("Unable to convert UTF-8 array into a string: {e}")
        };

        // Return
        Self {
            method,
            url,
            version,
            headers,
            body,
            stream: value
        }

    }
}


impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.method, self.url)
    }
}