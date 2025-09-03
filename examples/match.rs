pub fn num_to_string(num:u32)->String{
    match num{
        1 => "one".to_string(),
        2=> "TWO".to_string(),
        3=> "three".to_string(),
    
        _=> "other".to_string(),
    }
}

pub fn unwrap_or_default(x:Option<u32>,v:u32)->u32{
    match x{
        Some(v)=> v,
        None => v,
    }
}

fn main(){
    println!("{}",num_to_string(9));
    println!("{}",unwrap_or_default(None,0));
    println!("{}",unwrap_or_default(Some(10),0));
}