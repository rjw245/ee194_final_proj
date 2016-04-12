//Concurrency modules
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

const TOTAL_SIZE:       usize = 16;
const NTHREADS:         usize = 4;

fn main(){
    let mut thread_list: Vec<JoinHandle<()>> = Vec::new();

    let mut a : Vec<Vec<f32>> = Vec::new();
    let mut b : Vec<Vec<f32>> = Vec::new();
    let mut c : Vec<Vec<f32>> = Vec::new();

    init_matrix(&mut a, TOTAL_SIZE, 1.0);
    init_matrix(&mut b, TOTAL_SIZE, 1.0);
    init_matrix(&mut c, TOTAL_SIZE, 0.0);

    let shared_a = Arc::new(a);
    let shared_b = Arc::new(b);
    let shared_c = Arc::new(Mutex::new(c));

    // //split into separate threads

    for t in 0..NTHREADS {
        let a = shared_a.clone();
        let b = shared_b.clone();
        let c = shared_c.clone();
        let handle = thread::spawn(move ||{
            println!("Spawned thread");
            let mut c_mut = c.lock().unwrap();
            // multiply(&thread_a, &thread_b, &c, t);
            let from = (t * TOTAL_SIZE) / NTHREADS;
            let to   = ((t+1) * TOTAL_SIZE) / NTHREADS;
            for i in from..to {
                for j in 0..TOTAL_SIZE {
                    for k in 0..TOTAL_SIZE {
                        c_mut[i][j] += a[i][k] * b[k][j];
                    }
                }
            }
            });
        thread_list.push(handle);
    }

    // // rejoin
    thread::sleep(Duration::from_millis(5));
    // let mut result:
    print_matrix(&shared_a);
    println!("*");
    print_matrix(&shared_b);
    println!("=");
    print_matrix(&shared_c.lock().unwrap());
}

// fn multiply(a: &Vec<Vec<f32>>, b: &Vec<Vec<f32>>, c: &Vec<Vec<f32>>, slice: usize){
//     let s = slice;
//     let from = (s * TOTAL_SIZE) / NTHREADS;
//     let to   = ((s+1) * TOTAL_SIZE) / NTHREADS;
//     for i in from..to {
//         for j in 0..TOTAL_SIZE {
//             // c[i][j] = 0.0;
//             for k in 0..TOTAL_SIZE {
//                 c[i][j] = a[i][k] * b[k][j];
//             }
//         }
//     }
// }

fn init_matrix(m: &mut Vec<Vec<f32>>, size: usize, increment: f32) {
    let mut val : f32 = 0.0;
    for r in 0..size{
        m.push(Vec::new());
        for c in 0..size{
            m[r].push(val);
            val=val+increment;
        }
    }
}

fn print_matrix(m: & Vec<Vec<f32>>) {
    for r in m {
        for item in r {
            print!("{} ",item);
        }
        println!("");
    }
}
