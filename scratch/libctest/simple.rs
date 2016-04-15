extern crate libc;
use libc::c_int;

#[link(name = "simple")]
extern {
    fn add(a: c_int, b: c_int) -> c_int;
}

fn main(){
    let a = unsafe{ add(3,5) };
    println!("{}",a);
}
