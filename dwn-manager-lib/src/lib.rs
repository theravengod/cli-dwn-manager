pub fn exec() {
    println!("Alive !")
}

#[cfg(test)]
mod tests {
    // use cargo test --color=always --workspace -- --show-output
    #[test]
    fn call_with_reqwest() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        rt.block_on(async {
            let url = "https://mocki.io/v1/0e910579-8fd2-496c-8d1a-cf743041c020".to_string();
            let client = reqwest::Client::new();

            let body = client
                .get(url)
                .send()
                .await
                .expect("HTTP error")
                .text()
                .await
                .unwrap();

            println!("Got body: {:?}", body);

            assert!(!body.is_empty());
        });
    }
}
