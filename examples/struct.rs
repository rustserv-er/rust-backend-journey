#[derive(Debug)]

pub struct Account{
    pub address:String,
    pub balance:u32,
}

pub fn new(address:String)->Account{
    let account=Account{
        address:address,
        balance:0
    };
    return account;
}

fn main(){
    let account=Account{
        address:String::from("vfifm43445344ferjn"),
        balance:20
    };
    println!("{:?}",new("njgnrgn45".to_string()));

    println!("{:?}",account);
}