use reqwest::Response;

pub async fn check_status(server: &str) -> std::io::Result<Response> {
    let response = reqwest::get(format!("http://{server}/status")).await;

    match response {
        Ok(response) => Ok(response),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "server not ready",
        )),
    }
}

pub async fn new_test(server: &str) -> std::io::Result<Response> {
    let response = reqwest::get(format!("http://{server}/new")).await;

    match response {
        Ok(response) => Ok(response),
        Err(e) => Err(std::io::Error::new(
            std::io::ErrorKind::TimedOut,
            "server not ready",
        )),
    }
}

pub async fn malicious_domains(server: &str) -> Vec<String> {
    let response = reqwest::get(format!("http://{server}/malicious-domains")).await;

    match response {
        Ok(response) => {
            if let Ok(body) = response.text().await {
                let domains: Vec<String> = serde_json::from_str(&body).unwrap();
                domains
            } else {
                Vec::new()
            }
        },
        Err(_) => {
            Vec::new()
        },
    }
}
