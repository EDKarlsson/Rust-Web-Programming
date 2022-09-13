use futures::executor::block_on;
use std::{thread, time};

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

fn main() {
    let now = time::Instant::now();
    let future_one = do_something(1);
    let outcome = block_on(future_one);
    println!("Here is the outcome 1: {}", outcome);
    println!("time elapsed {:?}", now.elapsed());
    let future_two = async {
        let outcome_two = do_something(2).await;
        let outcome_three = do_something(3).await;
        // return do_something(2).await
        return [outcome_two, outcome_three]
    };
    let future_two = block_on(future_two);
    println!("Here is the outcome: {:?}", future_two);
}