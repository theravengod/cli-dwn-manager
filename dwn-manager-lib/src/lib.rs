use std::fs::File;
use std::io::copy;
use reqwest::blocking::Client;

pub fn exec() {
    println!("Alive !")
}

// Needs blocking client
fn download_file(blocking_client: Client, url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting download from: {}", url);

    let mut response = blocking_client.get(url).send()?;
    let mut output_file = File::create(output_path)?;

    copy(&mut response, &mut output_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // run with cargo test --color=always --workspace -- --show-output
    #[test]
    fn call_with_reqwest() {
        let url = "https://mocki.io/v1/0e910579-8fd2-496c-8d1a-cf743041c020";
        let client = reqwest::blocking::Client::new();

        let result = download_file(client, url, "../test.txt");
        assert!(result.is_ok())
    }
}
