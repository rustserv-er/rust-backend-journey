//Exercie 1: Return the first element in the tuple t.

pub fn first(t:(bool,u32,char))->bool{
    return t.0;
}


// Exercise 2: Return the last element 


pub fn last(t: (bool, u32, char)) -> char {
    return t.2;
}

// Exercise 3: swap the first and second elements of the tuple t


pub fn swap(t:(u32, u32))->(u32,u32){
    let (mut a,mut c)=t;
    let b=a;
     a=c;
     c=b;
    return (a,c);
}


fn main(){
    // Exercise 1
    let t:(bool,u32,char)=(true,20,'f');
    let g:(u32,u32)=(2,3);
    println!("{}",first(t));
      println!("{}",last(t));
      let (b,c)=swap(g);

      println!("{},{}",b,c);
    //Exercise 2
}