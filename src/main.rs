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

fn main() {

    let coins = vec![1, 2, 2, 5, 10, 20, 20, 50, 100, 200, 200, 5 * 100, 10 * 100, 20 * 100, 20 * 100, 50 * 100];

    let mut num_not_pay = 0;

    for i in 1..100*100 {

        if !can_pay(&coins, i) {
            num_not_pay += 1;
            println!("can not pay {} cents", i);
        }

    }

    println!("can not pay {} combinations", num_not_pay);

    let res = get_pay_combination(&coins, 9999);
    let s: String = res.iter().map(|n| n.to_string() + ", ").collect();

    println!("{}", s);

}
