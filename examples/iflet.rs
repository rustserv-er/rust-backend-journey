pub fn unwrap_or_default(x:Option<u32>,v:u32)->u32{
   if let Some(v)=x{
  
       return v;
   }
   return v;
} 


fn main (){
    println!("{}",unwrap_or_default(None,0));
    // unwrap_or_default(None,0);
        println!("{}",unwrap_or_default(Some(10),0));
}