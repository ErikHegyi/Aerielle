use std::{
    io::Result,
    net::TcpStream
};
use crate::http::Response;

pub struct Request {}


impl Request {
    pub fn respond(&self, response: Response) -> Result<Response> {
        todo!()
    }
}

impl From<TcpStream> for Request {
    fn from(value: TcpStream) -> Self {
        todo!()
    }
}