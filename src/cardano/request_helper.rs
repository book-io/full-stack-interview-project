use tokio_native_tls::TlsConnector as TlsConnector_Async;
use tokio_native_tls::native_tls::TlsConnector;
use tokio_native_tls::TlsStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::TcpStream;
use std::io::Write;
use std::io::Read;
use tokio::time::{sleep, Duration, Instant};
use std::net::*;
use std::vec::Vec;
use anyhow::Context;

pub struct HttpGetRequest<'a>
{
  host: &'a str,
  path: &'a str,
  headers: Vec<(&'a str, &'a str)>
}

impl HttpGetRequest<'_>
{
  pub fn new<'a>(host: &'a str, path: &'a str, headers:  Vec<(&'a str, &'a str)>) -> HttpGetRequest<'a>
  {
    HttpGetRequest {
      host,
      path,
      headers
    }
  }
}

pub async fn http_s_get(req: HttpGetRequest<'_>) -> anyhow::Result<String>
{
  println!("making request: GET {} ", req.path);
  //1. handshake
  let stream      = TcpStream::connect((req.host, 443)).await.context("tcp handshake failed")?;
  let connector   = TlsConnector::new().unwrap();
  let mut stream  = TlsConnector_Async::from(connector).connect(req.host, stream).await.context("tls handshake failed")?;

  //2. headers
  let mut request = String::from("");
  request.push_str(format!("GET {} HTTP/1.1\r\n", req.path).as_str());
  request.push_str(format!("Host: {}\r\n", req.host).as_str());
  request.push_str(format!("Accept: {}\r\n", "application/json").as_str());
  request.push_str("Connection: close\r\n");
  for header in req.headers
  {
    request.push_str(format!("{}: {}\r\n", header.0, header.1).as_str());
  }
  request.push_str("\r\n");

  //3. make request
  stream.write_all(request.as_bytes()).await.context("Writing to tcp socket failed")?;

  //4. read response
  let mut response = String::from("");
  stream.read_to_string(&mut response).await.context("Reading from tcp socket failed")?;

  Ok(response)
}

pub fn parse_httpStatusCode(response: &String) -> i32
{
  let line = response.lines().next();

  match line
  {
    Some(line) =>
    {
      let from = "HTTP/1.1 ".len();
      let until = "HTTP/1.1 xyz".len();
      if line.len() >= until
      {
        let status = &line[from..until];
        match status.parse::<i32>()
        {
          Ok(n) => n,
          Err(_) =>
          {
            eprintln!("Error parsing status {}.", status);
            500
          },
        }
      }
      else
      {
        500
      }
    },
    None => 500
  }
}

pub fn parse_extractJsonStringFromBody(res: &String) -> anyhow::Result<String>
{
  let mut lines = res.lines();
  let mut s: &str = "";

  loop
  {
    s = match lines.next()
    {
      Some(value) => value,
      None => break
    }
  }

  let first: &str = &s[0..1];
  let last: &str = &s[s.len() - 1..];

  //TODO: this suffices for now, but do better
  if first == "{" && last == "}"
  {
    Ok(s.to_string())
  }
  else
  {
    Err(anyhow::anyhow!("Error getting json string from response"))
  }
}