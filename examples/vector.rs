pub fn init(x:u32,y:u32,z:u32)->Vec<u32>{
    let v:Vec<u32>=vec![x,y,z];
    return v;
}

fn main(){
    println!("{:?}",init(1,2,3));
}