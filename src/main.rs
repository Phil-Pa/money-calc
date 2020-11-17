use rand::Rng;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::*;

fn get_pay_combination(coins: &[i32], cents: i32) -> Option<Vec<i32>> {
    let mut left = cents;
    let mut res = Vec::<i32>::new();

    for coin in coins.iter().rev() {
        if left - coin >= 0 {
            left -= coin;
            res.push(*coin);
        }

        if left == 0 {
            return Some(res);
        }
    }

    None
}

fn init_map(coins: &[i32]) -> HashMap<i32, i32> {
    let mut usage_map = HashMap::new();

    for coin in coins {
        if !usage_map.contains_key(coin) {
            usage_map.insert(*coin, 0);
        }
    }

    usage_map
}

fn get_range(parts: i32, i: i32) -> (i32, i32) {
    (2 + i * parts, (i + 1) * parts + 1)
}

const COINS: [i32; 16] = [1, 2, 2, 5, 10, 20, 20, 50, 100, 200, 200, 5 * 100, 10 * 100, 20 * 100, 20 * 100, 50 * 100];

fn create_thread(index: i32, count: i32, num_parts: i32, tx_ref: Sender<(i32, HashMap<i32, i32>)>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let mut random = rand::thread_rng();
        
        let range = get_range(num_parts, index);
        let mut temp_usage_map = init_map(&COINS);
        
        for _ in 0..count {
            for max_random_number in range.0..range.1 {
                let random_number = random.gen_range(1, max_random_number);
                let res = get_pay_combination(&COINS, random_number).unwrap();

                for coin in res {
                    *temp_usage_map.get_mut(&coin).unwrap() += 1;
                }
            }
        }

        //std::thread::sleep(std::time::Duration::from_millis(500 * index as u64));

        println!("sending {}", index);
        tx_ref.send((index, temp_usage_map)).unwrap();
    })
}

fn main() {

    

    let count = 100;
    let num_threads = 8;
    let max_money = 10000;
    let num_parts = max_money / num_threads;

    let mut threads = Vec::with_capacity(num_threads as usize);

    let map = init_map(&COINS.to_vec());
    let result_map = Arc::new(Mutex::new(map));
    let (tx, rx) = channel();
    
    for i in 0..num_threads {
        let tx_ref = Sender::clone(&tx);

        threads.push(create_thread(i, count, num_parts, tx_ref));
    }

    let result_map_ref = result_map.clone();

    std::thread::spawn(move || {
        for recv_data in rx {

            let (thread_id, map): (i32, HashMap<i32, i32>) = recv_data;

            println!("receiving {}", thread_id);
            for pair in map {
                let (coin, coin_count) = pair;
                *result_map_ref.lock().unwrap().get_mut(&coin).unwrap() += coin_count;
            }
        }
    });

    for thread in threads {
        thread.join().unwrap();        
    }

    let mut map = HashMap::new();

    for i in result_map.lock().unwrap().iter() {
        let (k, v) = i;
        map.insert(*k, *v);
    }

    let mut v: Vec<_> = map.into_iter().collect();
    v.sort();

    let mut s = 0;

    for pair in v {
        println!("{} -> {}", pair.0, pair.1 as f32);
        s += pair.1;
    }

    println!("{}", s);

}
