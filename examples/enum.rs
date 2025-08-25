#[derive(Debug,PartialEq)]

pub enum color{
    Red,
    Green,
    Blue,
    Rgb(u8,u8,u8)}

fn main(){
    let r1:color=color::Red;
    let r2:color=color::Blue;
    let r3:color=color::Green;
    let r4:color=color::Rgb(255,0,0);
    println!("{:?}",r1);
     println!("{:?}",r2);
      println!("{:?}",r3);
       println!("{:?}",r4);
}