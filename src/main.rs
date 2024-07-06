/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

fn test(value: i32) -> Result<i32, i32> {
    match Some(value) {
        Some(1) => Ok(1),
        Some(2) => Ok(2),
        Some(3) => Ok(3),
        other => Err(other.unwrap_or(0)),
    }
}

fn enter_input(input: &mut String) {
    match std::io::stdin().read_line(input) {
        Ok(_) => (),
        Err(e) => {
            println!("Error: {:?}", e);
            enter_input(input);
        }
    }
}

fn main() {
    let mut input = String::new();
    println!("Enter a number: ");
    enter_input(&mut input);

    let value: i32 = input.trim().parse().unwrap_or(0);

    match test(value) {
        Ok(result) => println!("Result: {}", result),
        Err(other) => println!("Error: {} is big!", other),
    }
}
