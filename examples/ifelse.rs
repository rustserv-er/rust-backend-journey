// Exercie 1 

pub fn min(x:i32,y:i32)->i32{
    let hello:i32=if x<y{   
        x
    }
    else{
      y
    };
    return hello;
}

pub fn max(x:i32,y:i32)->i32{
    let hello:i32=if x>y{ 
    x
    }
    else{
      y
    };
    return hello;
}



pub fn sign(x:i32)->i32{
    let hellow:i32=if x>0{
         1
    }
    else{
        -1
    };
    return hellow;
}


fn main(){

    println!("{:?}",min(20,30));
     println!("{:?}",max(20,30));
     println!("{:?}",sign(90));
}