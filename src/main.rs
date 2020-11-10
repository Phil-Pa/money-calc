use rand::Rng;

use std::collections::HashMap;

fn can_pay(coins: &Vec<i32>, cents: i32) -> bool {

    let mut res = cents;

    for coin in coins.iter().rev() {

        if res - coin >= 0 {
            res -= coin;
        }

        if res == 0 {
            return true;
        }

    }

    false
}

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

fn init_map(coins: &Vec<i32>) -> HashMap<&i32, i32> {
    let mut usage_map = HashMap::new();

    for coin in coins {
        if !usage_map.contains_key(&coin) {
            usage_map.insert(coin, 0);
        }
    }

    usage_map
}

fn main() {

    let coins = vec![1, 2, 2, 5, 10, 20, 20, 50, 100, 200, 200, 5 * 100, 10 * 100, 20 * 100, 20 * 100, 50 * 100];

    let mut rng = rand::thread_rng();

    let mut sum = 0;
    let count = 1000;

    let mut usage_map = init_map(&coins);

    for max_random_number in 2..100 * 100 {
        for _ in 0..count {
            let random_number = rng.gen_range(1, max_random_number);
    
            let res = get_pay_combination(&coins, random_number);
            //let s: String = res.iter().map(|n| n.to_string() + ", ").collect();
            sum += res.len();
    
            for coin in res {
                *usage_map.get_mut(&coin).unwrap() += 1;
            }
        }
    }

    println!("average num coins for paying: {}", sum as f32 / count as f32);

    // for entry in usage_map.iter() {
    //     let (key, value) = entry;
    //     println!("used {}: {} times", key, *value as f32 / sum as f32 * 100.0);
    // }

    let mut v: Vec<_> = usage_map.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));

    for pair in v {
        let (key, value) = pair;
        println!("used {}: {} times", key, value as f32 / sum as f32 * 100.0)
    }

    //println!("paying {} with:{}", random_number, s);

}
