use futures::executor::block_on;
use std::{thread, time};
use futures::future::join_all;

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
        return [outcome_two, outcome_three];
    };
    let future_two = block_on(future_two);

    println!("Here is the outcome: {:?}", future_two);

    let third_outcome = async {
        let mut futures_vec = Vec::new();
        let future_four = do_something(4);
        let future_five = do_something(5);

        futures_vec.push(future_four);
        futures_vec.push(future_five);

        let handles = futures_vec.into_iter().map(async_std::task::spawn).collect::<Vec<_>>();
        let results = join_all(handles).await;

        return results;
    };

    let now = time::Instant::now();
    let result = block_on(third_outcome);

    println!("time elapsed for join vec {:?}", now.elapsed());
    println!("Here is the result: {:?}", result);
}