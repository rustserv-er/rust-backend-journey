// Return an array with 100 elements, all equal to 0.

pub fn zeros() -> [u32; 100] {
    let arr:[u32;100]=[0;100];
   return arr;
}

pub fn first_3(s: &[u32]) -> &[u32] {
    return &s[..3];
}

pub fn last_3(s:&[u32])->&[u32]{
    return &s[3..6];
}


fn main(){
    let array =zeros();
    let s:[u32;6]=[1,2,3,4,5,6];
    let slices_first= first_3(&s);
    let slices_last=last_3(&s);
    println!("{:?}",array);
    println!("{:?}",slices_last);
    println!("{:?}",slices_first)
}

