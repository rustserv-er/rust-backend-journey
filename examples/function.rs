#![allow(unused)]


fn add(x:u32,y:u32)->u32{
    return x+y;
}

pub fn div(x:u32,y:u32)->u32{
    return x*y;
}
pub fn mul(x:u32,y:u32)->u32{
    return x/y;
}

fn main(){
    println!("{}",add(4,5));
    println!("{}",div(4,5));
    println!("{}",mul(4,5));
}