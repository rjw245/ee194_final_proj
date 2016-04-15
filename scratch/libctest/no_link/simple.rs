const NLOOPS: usize = 100000000;

fn main(){
    for i in 0..NLOOPS {
        let s = add(3,5);
    }
}

fn add(a: i32, b: i32) -> i32 {
    let s = a+b;
    s
}
