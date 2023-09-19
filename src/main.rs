use std::io;
use std::io::Write;
use error_chain::error_chain;
use reqwest::Client;

const GOOGLE_URL: &str = "https://google.com/";

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() {
    loop {
        let mut url = String::new();
        let mut proxy_url = String::new();
        let mut proxy_auth = String::new();

        request_input("Enter url (default Google) >> ".to_string(), &mut url);
        request_input("Use proxy (protocol://ip:port) >> ".to_string(), &mut proxy_url);
        request_input("Proxy auth (user:pwd) >> ".to_string(), &mut proxy_auth);

        let url = if url.trim().len() > 0 {
            url.trim().to_string()
        } else {
            GOOGLE_URL.to_string()
        };

        let proxy_url = proxy_url.trim().to_string();
        let proxy_auth = proxy_auth.trim().to_string();

        let client = get_reqwest_client(&proxy_url, &proxy_auth);

        let res = match client.get(url.trim()).send().await {
            Ok(res) => res,
            Err(e) => return println!("Failed to make request, it's error {}", e),
        };
        let status = res.status();

        if !status.is_success() {
            println!("Failed. Status code is {}", status.as_u16());
        } else {
            println!("Success");
        }
    }
}

fn get_reqwest_client(proxy_url: &String, proxy_auth: &String) -> Client {
    let mut client = Client::builder();

    if !proxy_url.is_empty() {
        let mut proxy = reqwest::Proxy::all(proxy_url).expect("Failed to unwrap proxy");

        if !proxy_auth.is_empty() {
            let auth = proxy_auth.split(":").collect::<Vec<&str>>();;
            proxy = proxy.basic_auth(auth[0], auth[1]);
        }

        client = client.proxy(proxy);
    }

    client.build().unwrap()
}

fn request_input(message: String, save_to: &mut String) {
    print!("{}", message);
    io::stdout().flush().expect("Flush failed");
    io::stdin().read_line(save_to).expect("Failed to read input");
}
