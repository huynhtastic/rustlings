// quiz1.rs
// This is a quiz for the following sections:
// - Variables
// - Functions
// - If

// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// Write a function that calculates the price of an order of apples given
// the quantity bought. No hints this time!


// Put your function here!
const COST: u8 = 2;
const DISCOUNTED_COST: u8 = 1;

const MIN_FOR_DISCOUNT: u8 = 40;

fn calculate_price_of_apples(num_apples: u8) -> u8 {
    if num_apples > MIN_FOR_DISCOUNT {
        num_apples * DISCOUNTED_COST
    } else {
        num_apples * COST
    }
}

// Don't modify this function!
#[test]
fn verify_test() {
    let price1 = calculate_price_of_apples(35);
    let price2 = calculate_price_of_apples(40);
    let price3 = calculate_price_of_apples(41);
    let price4 = calculate_price_of_apples(65);

    assert_eq!(70, price1);
    assert_eq!(80, price2);
    assert_eq!(41, price3);
    assert_eq!(65, price4);
}
