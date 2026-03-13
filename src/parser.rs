use regex::Regex;

/*Data*/
pub fn extract_date(line: &str) -> Option<String> {
    let re = Regex::new(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2},\d{3}").unwrap();
    re.find(line).map(|m| m.as_str().to_string())
}

/*Log Type: ERROR, DEBUG , INFO, WARN*/
pub fn extract_type_log(line: &str) -> Option<String> {
    let re = Regex::new(r"[A-Z][A-Z][A-Z][A-Z][A-Z]").unwrap();
    re.find(line).map(|m| m.as_str().to_string())
}


/*
Workflow type:
    - PatientQuery
    - ResultOut

*/
pub fn extract_type_workflow(line: &str) -> Option<String> {
    let re = Regex::new(r"[A-Z][a-z]+[A-Z][a-z]+\(([^)]+)\)").unwrap();
    re.find(line).map(|m| m.as_str().to_string())
}


/*
Error from the middleware same like RapidComm, POCCeleretor

*/
pub fn extract_error(line: &str) -> Option<String> {
    let re = Regex::new(r"- ([A-Z][a-zA-Z0-9_]+\([^)]*\)[^\-]*)$").unwrap();
    re.captures(line)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().trim().to_string())
}
/*
Connection status from middleware

*/
pub fn extract_connection_status(line: &str) -> Option<String> {
    let re = Regex::new(r"(Disconnected|Connection error:|Connected to).*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}




/*

Grep HL7 

*/

pub fn extract_hl7(line: &str) -> Option<String> {
    let re = Regex::new(r"[A-Z][A-z][A-Z]\|.*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}

pub fn extract_msa(line: &str) -> Option<String> {
    let re = Regex::new(r"MSA\|.*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}

/*Migliorare il mapping*/
pub fn extract_message(line: &str) -> Option<String> {
    let re = Regex::new(r"-\s(Query|Queue|).*").unwrap();
    re.find(line).map(|m| m.as_str().trim().to_string())
}