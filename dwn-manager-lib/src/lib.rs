use colored::Colorize;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;

pub fn exec() {
    println!("Alive !");

    let result = download_file(reqwest::blocking::Client::new(), "https://mocki.io/v1/0e910579-8fd2-496c-8d1a-cf743041c020", "./kpu.txt");
    if result.is_ok() {
        println!("{}", "Working !!!".bright_green());
    } else {
        println!("{}", format!("Failed {:?}", result.err().unwrap()).bright_red())
    }
}

// Needs blocking client
fn download_file(
    blocking_client: Client,
    url: &str,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = blocking_client.get(url).send();
    let mut output_file = File::create(output_path)?;

    if let Err(e) = response {
        let error_code = e.status().unwrap().as_u16();
        eprintln!("{:?} Error downloading file", format!("[{}]", error_code).bright_red());
        return Err(Box::new(e));
    }

    copy(response.as_mut().unwrap(), &mut output_file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    // run with: cargo test --color=always --workspace -- --show-output
    #[test]
    fn call_with_reqwest() {
        let url = "https://mocki.io/v1/0e910579-8fd2-496c-8d1a-cf743041c020";
        let client = reqwest::blocking::Client::new();

        let result = download_file(client, url, "../test.txt");
        assert!(result.is_ok())
    }
}
