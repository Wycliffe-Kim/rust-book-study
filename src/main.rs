/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

fn coin_example() {
    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }

    println!("----- coin_example -----");
    println!(
        "value_in_cents(Coin::Penny) = {}",
        value_in_cents(Coin::Penny)
    );
    println!(
        "value_in_cents(Coin::Nickel) = {}",
        value_in_cents(Coin::Nickel)
    );
    println!(
        "value_in_cents(Coin::Dime) = {}",
        value_in_cents(Coin::Dime)
    );
    println!(
        "value_in_cents(Coin::Quarter(UsState::Alaska)) = {}",
        value_in_cents(Coin::Quarter(UsState::Alaska))
    );
    println!(
        "value_in_cents(Coin::Quarter(UsState::Alabama)) = {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );
}

fn plus_one_example() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    println!("----- plus_one_example -----");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five = {:?}", five);
    println!("six = {:?}", six);
    println!("none = {:?}", none);
    println!("five = {:?}", five.unwrap_or(0));
    println!("six = {:?}", six.unwrap_or(0));
    println!("none = {:?}", none.unwrap_or(0));
}

fn dice_roll_example() {
    use rand::Rng;

    println!("----- dice_roll_example -----");

    println!("----- _ 사용함 -----");
    for _ in 0..5 {
        let dice_roll = rand::thread_rng().gen_range(1..=10);
        print!("{} ", dice_roll);
        match dice_roll {
            1 => println!("You rolled a 1!"),
            2 => println!("You rolled a 2!"),
            3 => println!("You rolled a 3!"),
            4 => println!("You rolled a 4!"),
            5 => println!("You rolled a 5!"),
            6 => println!("You rolled a 6!"),
            _ => println!("You rolled a number that is not 1-6!"),
        }
    }

    println!("----- _ 사용안함 -----");
    for _ in 0..5 {
        let dice_roll = rand::thread_rng().gen_range(1..=10);
        print!("{} ", dice_roll);
        match dice_roll {
            1 => println!("You rolled a 1!"),
            2 => println!("You rolled a 2!"),
            3 => println!("You rolled a 3!"),
            4 => println!("You rolled a 4!"),
            5 => println!("You rolled a 5!"),
            6 => println!("You rolled a 6!"),
            other => println!("You rolled a number that is not 1-6! ({})", other),
        }
    }
}

fn main() {
    coin_example();
    plus_one_example();
    dice_roll_example();
}
