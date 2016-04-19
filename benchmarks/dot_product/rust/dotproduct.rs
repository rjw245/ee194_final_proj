use std::sync::Arc;
use std::thread;
use std::env;

const N: usize          = 1048576;

fn main() {
    let args: Vec<String> = env::args().collect();
    let THREADS: usize = args[1].parse::<usize>().unwrap();

    let mut x: Vec<f32> = vec![0.0; N];
    let mut y: Vec<f32> = vec![0.0; N];
    init_vecs(&mut x, &mut y);

    if THREADS==1 {
        let dotprod = dot_prod(&x, &y, 0, N);
        println!("{}",dotprod);
    } else { 
        let x_arc = Arc::new(x);
        let y_arc = Arc::new(y);
        let mut handles = Vec::new();
        for t in 0..THREADS {
            let x_shared = x_arc.clone();
            let y_shared = y_arc.clone();
            let handle = thread::spawn(move || {
                println!("Spawned thread");
                let start   = t    *N / THREADS;
                let mut end = (t+1)*N / THREADS;
                if(t==THREADS-1) { end = N; }
                let partial_dotprod: f32 = dot_prod(&x_shared,&y_shared,start,end);
                partial_dotprod
            });
            handles.push(handle);
        }

        let mut dotprod: f32 = 0.0;

        for handle in handles {
            dotprod += handle.join().unwrap();
        }
        println!("{}",dotprod);
    }
}

fn init_vecs(x: &mut Vec<f32>, y: &mut Vec<f32>) {
    //Initialize so that dot product = N
    for i in 0..x.len() {
        x[i] = 4.0;
        y[i] = 0.25;
    }
}

fn dot_prod(x: & Vec<f32>, y: & Vec<f32>, start: usize, end: usize) -> f32 {
    let mut dotprod: f32 = 0.0;
    for i in start..end {
        if(i>=0 && i<x.len()) {
            dotprod = dotprod + (x[i] * y[i]);
        }
    }
    dotprod
}
