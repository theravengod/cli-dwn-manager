use reqwest::blocking::{Client, Response};
use reqwest::header::CONTENT_LENGTH;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};

pub fn exec() {
    println!("Alive !");
    let good_url = "https://mocki.io/v1/0e910579-8fd2-496c-8d1a-cf743041c020";

    let result = download_file(reqwest::blocking::Client::new(), good_url, "./kpu.txt");
    match result {
        Ok(_) => {
            println!("[✅ ] Successful");
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
) -> Result<(), Option<Box<dyn std::error::Error>>> {
    let response = blocking_client.get(url).send();

    let (payload, err_payload) = match response {
        Ok(val) => (Some(val), None),
        Err(e) => (None, Some(e)),
    };

    if let Some(mut data) = payload {
        let total_size: u64 = data
            .headers()
            .get(CONTENT_LENGTH)
            .and_then(|header| header.to_str().ok())
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        println!("[✅ ] Got data !");
        let status_code = data.status();
        if status_code.is_success() {
            println!("[✅ ] Got code: {}", status_code.as_u16());

            let output_file = File::create(output_path);
            match output_file {
                Ok(mut file) => {
                    let progress_reporter = |downloaded: u64| {
                        if total_size > 0 {
                            let percentage = (downloaded as f64 / total_size as f64) * 100.0;
                            println!("Downloaded {}% of file", percentage);
                        } else {
                            println!("Downloaded {} bytes", downloaded);
                        }
                    };
                    copy_with_progress(&mut data, &mut file, progress_reporter)
                }
                Err(e) => {
                    eprintln!(
                        "[❌ ] Could not create file at location {}: {}",
                        output_path, e
                    );
                    Err(Some(Box::from(e)))
                }
            }
        } else {
            Err(Some(
                format!("Received code: {}", status_code.as_u16()).into(),
            ))
        }
    } else {
        Err(Some(Box::new(err_payload.unwrap())))
    }
}

fn copy_with_progress<F>(
    input: &mut Response,
    output: &mut File,
    on_progress: F,
) -> Result<(), Option<Box<dyn Error>>>
where
    F: Fn(u64) -> (),
{
    let mut buffer = [0; 8192];
    let mut downloaded: u64 = 0;
    let mut some_error: Option<Box<dyn Error>> = None;
    loop {
        let bytes_read = input
            .read(&mut buffer)
            .inspect(|b| println!("Transferred: {}", b))
            .map_err(|e| eprintln!("Error occurred while copying: {}", e))
            .unwrap();

        if bytes_read == 0 {
            break;
        }

        let write_result = output.write_all(&buffer[..bytes_read]);

        if write_result.is_err() {
            some_error = Some(Box::from(write_result.err().unwrap()));
            break;
        } else {
            downloaded += bytes_read as u64;
            on_progress(downloaded);
            std::io::stdout().flush().expect("Could not flush !"); // Might need to handle this error
        }
    }

    if some_error.is_some() {
        Err(some_error)
    } else {
        Ok(())
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
