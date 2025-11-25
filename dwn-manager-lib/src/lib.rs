use colored::Colorize;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;

pub fn exec() {
    println!("Alive !");

    let result = download_file(reqwest::blocking::Client::new(), "https://mocki.io/v1/0e910579-8fd2-496c-8d1a-cf743041c0201", "./kpu.txt");
    match result {
        Ok(c_size) => {
            println!("[✅ ] Downloaded {} bytes", c_size);
        },
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
) -> Result<(u64), Box<dyn std::error::Error>> {
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
            let mut output_file = File::create(output_path)?;

            let copy_result = copy(&mut data, &mut output_file);
            if copy_result.is_ok() {
                let copied_size = copy_result.unwrap();
                println!("[✅ ] Copied successfully {} bytes", copied_size);
                Ok(copied_size)
            } else {
                Err(Box::new(copy_result.err().unwrap()))
            }
        } else {
            Err(format!("Received code: {}", status_code.as_u16()).into())
        }
    } else {
        Err(Box::new(err_payload.unwrap()))
    }



    /*if let Err(e) = response {
        let error_code = e.status().unwrap().as_u16();
        eprintln!("{:?} Error downloading file", format!("[{}]", error_code).bright_red());
        return Err(Box::new(e));
    }

    copy(response.as_mut().unwrap(), &mut output_file)?;

    Ok(())*/
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
