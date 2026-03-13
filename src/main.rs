mod file;
mod parser;
mod connection_utils;
mod hl7_utils;


fn main() {
   let blocks: Vec<String> = file::readlog(&"./rapidcomm_lis_middleware.log".to_string()).unwrap();
    //let m = parser::extract_type_workflow(&blocks[0]).unwrap_or_default();
    for line in blocks{
        let d = parser::extract_message(&line);
      
      println!("{:?}",d);

    }
      



} 
  
