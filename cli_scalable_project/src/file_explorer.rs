pub fn read_text(path : String) -> Result<String, Box<dyn std::error::Error>> {
    let contents: String = std::fs::read_to_string(path)?;
    return Ok(contents);
}

pub fn search_in_text(content: &str, querry: &Vec<String>) -> Result<Vec<String>, String> {
    let mut result: Vec<String> = Vec::new();

    for line in content.lines() {
        for keyword in querry {
            if line.contains(keyword) {
                if !result.contains(&&line.to_string()) {
                    result.push(line.to_string());
                }
            }
        }
    }

    if result.len() < 1 {
        return Err("did not found anything or does not have a querry keyword"
            .trim()
            .to_string());
    }

    return Ok(result);
}