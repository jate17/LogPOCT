


use std::fs;


pub fn readlog(filename: &String) -> Result<Vec<String>, std::io::Error> {
    println!("[*] Read Log: {}", filename);
    let file = fs::read_to_string(filename)?;
    let content: Vec<String> = file.lines().map(|l| l.to_string()).collect();
    Ok(content)
}