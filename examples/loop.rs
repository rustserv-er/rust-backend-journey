pub fn fill(i:u32,n:usize)->Vec<u32>{
    let mut v:Vec<u32>=Vec::new();
    for i in 0..n{
  
        v.push(i as u32);
    }
    return v;
}

pub fn sum(nums:Vec<i32>)->i32{
     let mut i:i32=0;
     for num in nums{
        i+=num;
     }
     return i;
}


fn main(){

    println!("{:?}",sum(vec![1,2,3,4,5]));
    println!("{:?}",fill(0,10))
}

