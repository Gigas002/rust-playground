mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // requires `use`
    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_orders() { }

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
//     Ok(())
// }

// fn function2() -> io::Result<()> {
//     // --snip--
//     Ok(())
// }

// or

use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}

// use std::cmp::Ordering;
// use std::io;

// same as

// use std::{cmp::Ordering, io};

// use std::io;
// use std::io::Write;

// same as

// use std::io::{self, Write};

use std::collections::*;
