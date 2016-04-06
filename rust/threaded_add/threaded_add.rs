use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    let inputsize:  usize = 1000;
    let numthreads: usize = 7;
    let mut thread_list: Vec<JoinHandle<i64>> = Vec::new();
    
    let input: Vec<i64> = vec![1; inputsize];
    let shared_input = Arc::new(input);
    for t in 0..numthreads {
        let thread_copy = shared_input.clone();
        let span = inputsize/numthreads;
        let handle = thread::spawn(move || {
            println!("Spawned thread");
            let mut start = t*span;
            let mut end   = t*span+span;
            if (t==numthreads-1) {
                end = inputsize;
            }
            let partialsum = sum_arr(&thread_copy,start,end);
            partialsum
        });
        thread_list.push(handle);
    }

    //Wait for threads to finish and collect their results
    let mut sum: i64 = 0;
    for thread in thread_list {
        sum+=thread.join().unwrap();
    }
    println!("{}",sum);
}

//Sums data vector from index start up to but not including index end
fn sum_arr(data: &Vec<i64>, start: usize, end: usize) -> i64 {
    let mut sum: i64 = 0;
    for i in start..end {
        if (start >=0) && (end<=data.len()) {
            sum += data[i];
        }
    }
    sum
}
