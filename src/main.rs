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

fn main() {
    coin_example();
}
