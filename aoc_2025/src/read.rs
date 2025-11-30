pub fn read(path: &str) -> Result<String, anyhow::Error> {
    let input = std::fs::read_to_string(path)?;
    Ok(input)
}
