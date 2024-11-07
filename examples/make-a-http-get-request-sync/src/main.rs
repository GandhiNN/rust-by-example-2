use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn http_get_request_sync() -> Result<()> {
    let mut res = reqwest::blocking::get("https://api.restful-api.dev/objects")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}

fn main() {
    let _ = http_get_request_sync();
}
