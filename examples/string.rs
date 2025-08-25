// Exercise 1

pub fn hello()->String{
     let s:String=String::from("Hello Rust");
     return s;
    }


    // Exercise 2:
// pub fn greet(name:&str)->String{
//     let s:String=format!("Hello {}",name);
//     return s;
// }

pub fn greet(name:&str)->String{
    let s:String=String::from("Hello ");
    return s+name;
}


// Exercise 3
pub fn append(mut s:String)->String{
    s+="!";
    s
}


fn main(){
    println!("{}",hello());
    println!("{}",greet(" VEsh"));
    println!("{}",append("Go away".to_string()));
}