use rand::Rng;

use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicI32;

fn get_pay_combination(coins: &Vec<i32>, cents: i32) -> Vec<i32> {
    let mut left = cents;

    let mut res = Vec::<i32>::new();

    for coin in coins.iter().rev() {
        if left - coin >= 0 {
            left -= coin;
            res.push(*coin);
        }

        if left == 0 {
            return res;
        }
    }

    panic!()
}

fn init_map(coins: Vec<i32>) -> HashMap<i32, i32> {
    let mut usage_map = HashMap::new();

    for coin in coins {
        if !usage_map.contains_key(&coin) {
            usage_map.insert(coin, 0);
        }
    }

    usage_map
}

fn main() {

    //let coins: Vec<i32> = vec![1, 2, 2, 5, 10, 20, 20, 50, 100, 200, 200, 5 * 100, 10 * 100, 20 * 100, 20 * 100, 50 * 100];
    let count = 100;
    let num_threads = 8;
    let sum = num_threads * count;
    let max_money = 10000;
    let div = max_money / num_threads;

    assert!(max_money % num_threads == 0);

    let ranges = Arc::new(Mutex::new(Vec::new()));
    for i in 0..num_threads {
        let start = 2 + i * div;
        let end = (i + 1) * div + 1;
        ranges.lock().unwrap().push((start, end));
        println!("({}, {})", start, end);
    }

    let mut threads = vec![];

    const COINS: [i32; 16] = [1, 2, 2, 5, 10, 20, 20, 50, 100, 200, 200, 5 * 100, 10 * 100, 20 * 100, 20 * 100, 50 * 100];
    let result_map = Arc::new(Mutex::new(init_map(COINS.to_vec())));
    
    for i in 0..num_threads {
        let cloned_result_map = Arc::clone(&result_map);
        let range = ranges.clone();

        threads.push(std::thread::spawn(move || {
            let mut random = rand::thread_rng();
            
            let range = range.lock().unwrap()[i as usize];
            let coins_vec = COINS.to_vec();
            let cloned = coins_vec.clone();
            let mut temp_usage_map = init_map(coins_vec);
            
            for _ in 0..count {
                for max_random_number in range.0..range.1 {
                    let random_number = random.gen_range(1, max_random_number);
                    let res = get_pay_combination(&cloned, random_number);

                    for coin in res {
                        *temp_usage_map.get_mut(&coin).unwrap() += 1;
                    }
                }
            }

            let mut map = cloned_result_map.lock().unwrap();
            for pair in temp_usage_map.into_iter() {
                *map.get_mut(&pair.0).unwrap() += pair.1;
            }
        }));
    }

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
