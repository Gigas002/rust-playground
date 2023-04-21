use crate::us_state;

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(us_state::UsState)
}
