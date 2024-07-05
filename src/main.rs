/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("Please input a number: ");
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    println!("input: {}", input);

    let input_number: i32 = input.trim().parse().unwrap_or(0);
    let message = if input_number == 0 {
        Message::Move { x: 1, y: 2 }
    } else if input_number == 1 {
        Message::Write(input)
    } else if input_number == 2 {
        Message::ChangeColor(1, 2, 3)
    } else {
        Message::Quit
    };

    match message {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move x: {}, y: {}", x, y),
        Message::Write(s) => println!("Write: {}", s),
        Message::ChangeColor(r, g, b) => println!("ChangeColor r: {}, g: {}, b: {}", r, g, b),
    }
}
