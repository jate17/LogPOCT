use crate::parser;


/*
use std::collections::HashMap;

const ackcode: HashMap<&str, &str> = HashMap::from([
    ("AA", "Application Accept"),
    ("AE", "Application Error"),
    ("AR", "Application Reject")
]);
*/

fn split_hl7(line: &str) -> Vec<&str> {
    line.split('|').collect()
}

pub fn msa(blocks: &Vec<String>) -> Vec<String> {
    let mut msa: Vec<String> = Vec::new();
    for line in blocks {
        let t = parser::extract_msa(line).unwrap_or_default();
        if !t.is_empty(){
            msa.push(t)
        }
    }
    msa
}