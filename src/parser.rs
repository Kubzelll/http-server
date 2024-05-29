
use std::collections::HashMap;

pub fn parse_request(request: &[u8]) -> Result<(String, String, HashMap<String, String>), Box<dyn std::error::Error>> {
    let request = String::from_utf8_lossy(request);

    let mut lines = request.lines();
    let request_line = lines.next().ok_or("Invalid HTTP request: Missing request line")?;
    let mut parts = request_line.split_whitespace();

    let method = parts.next().ok_or("Invalid HTTP request: Missing method")?.to_string();
    let path = parts.next().ok_or("Invalid HTTP request: Missing path")?.to_string();

    let mut headers = HashMap::new();
    for line in lines {
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }
    }

    Ok((method, path, headers))
}
