// Exercie 1 to compare two character equality 

pub fn eq(x:char,y:char)->bool{
    return x==y;
}
// Exercise 2

pub fn add(a:f32,b:f32,c:f32)->f32{
    return a+b+c;
}

// Exercise 3
pub fn cast(x:f32,y:f32,z:f32)->f32{
    return x+y+z;
}

fn main(){
    let c:char='2';
    let x:char='3';
    let z:u8=2;
    let m:i8=5;
    let k:f32=6.0;
    let u:f32= m as f32; 
    let s:f32= z as f32;
    // Exercise 1
    println!("{}",eq(c,x));
    //Exercise 2
    println!("{}",add(1.0,2.0,3.0));
    // Exercise 3
    println!("{}",cast(s,u,k));
}

