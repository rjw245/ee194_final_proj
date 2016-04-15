#![feature(link_args)]
#![feature(libc)]
extern crate libc;
use libc::c_int;

#[link_args = "-L./ -lsimple"]
#[link(name = "simple", kind="static")]
extern {
    fn add(a: c_int, b: c_int) -> c_int;
}

fn main(){
    let a = unsafe{ add(3,5) };
    println!("{}",a);
}
