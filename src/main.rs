fn main() {
    //for loop range
    for i in 1..10 {
        println!("{}", i);
    }

    //for loop vec
    let animals = vec!["dog", "cat", "fish", "rabbit", "snake", "frog", "spider"];
    for i in animals {
        println!("{}", i);
    }

    let fat = fatorial(5);
    println!("{}", fat);
}

fn fatorial(num: i32) -> i32 {
    let mut mult = 1;

    for i in 1..(num + 1) {
        mult *= i;
    }
    mult
}
