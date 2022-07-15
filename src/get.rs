use std::io::prelude::*;
use std::net::TcpStream;

pub fn get(host: &str) -> std::io::Result<()> {
    let mut conn = TcpStream::connect(host)?;

    conn.write_all(b"GET / HTTP/1.0")?;
    conn.write_all(b"\r\n")?;

    write!(conn, "Host: {}", host);
    conn.write_all(b"\r\n\r\n")?;

    std::io::copy(
        &mut conn,
        &mut std::io::stdout()
    )?;

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::get::get;

    #[test]
    fn test_get() {
        let _ = get("127.0.0.1:80");
    }
}