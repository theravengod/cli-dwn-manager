use colored::Colorize;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;

pub fn exec() {
    println!("Alive !");
    let good_url = "https://mocki.io/v1/0e910579-8fd2-496c-8d1a-cf743041c020";

    let result = download_file(reqwest::blocking::Client::new(), good_url, "./kpu.txt");
    match result {
        Ok(c_size) => {
            println!("[✅ ] Downloaded {} bytes", c_size);
        }
        Err(e) => {
            eprintln!("[❌ ] Something went wrong: {:?}", e);
        }
    }
}

// Needs blocking client
fn download_file(
    blocking_client: Client,
    url: &str,
    output_path: &str,
) -> Result<u64, Box<dyn std::error::Error>> {
    let response = blocking_client.get(url).send();

    let (payload, err_payload) = match response {
        Ok(val) => (Some(val), None),
        Err(e) => (None, Some(e)),
    };

    if let Some(mut data) = payload {
        println!("[✅ ] Got data !");
        let status_code = data.status();
        if status_code.is_success() {
            println!("[✅ ] Got code: {}", status_code.as_u16());

            let output_file = File::create(output_path);
            match output_file {
                Ok(mut file) => {
                    copy(&mut data, &mut file)
                        .inspect(|size| println!("[✅ ] Copied successfully {} bytes", size))
                        .map_err(Box::from)
                }
                Err(e) => {
                    eprintln!("[❌ ] Could not create file at location {}: {}", output_path, e);
                    Err(Box::from(e))
                }
            }
        } else {
            Err(format!("Received code: {}", status_code.as_u16()).into())
        }
    } else {
        Err(Box::new(err_payload.unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    // run with: cargo test --color=always --workspace -- --show-output
    #[test]
    fn test_good_call() {
        let url = "https://mocki.io/v1/0e910579-8fd2-496c-8d1a-cf743041c020";
        let client = reqwest::blocking::Client::new();

        let result = download_file(client, url, "../test.txt");
        assert!(result.is_ok());
        assert!(fs::remove_file("../test.txt").is_ok())
    }

    #[test]
    fn test_good_call_but_no_file_access() {
        let url = "https://mocki.io/v1/0e910579-8fd2-496c-8d1a-cf743041c020";
        let client = reqwest::blocking::Client::new();

        let _ = download_file(client, url, "/test.txt");
        assert!(!fs::exists("../test.txt").unwrap());
    }

    #[test]
    fn test_bad_call() {
        let bad_url = "https://mocki.io/v1/0e910579-8fd2-496c-8d1a-cf743041c0201";
        let client = reqwest::blocking::Client::new();

        let result = download_file(client, bad_url, "../test.txt");
        assert!(result.is_err());
        if fs::exists("../test.txt").unwrap() {
            assert!(fs::remove_file("../test.txt").is_ok())
        }
    }
}
