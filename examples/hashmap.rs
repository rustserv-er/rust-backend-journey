use std::collections::HashMap;

pub fn init(address:String,balance:u32)->HashMap<String,u32>{
    let mut address_to_be_baalnce_insert:HashMap<String,u32>=HashMap::new();
   address_to_be_baalnce_insert.insert(address,balance);
   return address_to_be_baalnce_insert;
}

// 
fn main(){

   println!("{:?}",init("0x123".to_string(),1000));
}