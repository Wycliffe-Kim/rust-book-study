/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

fn main() {
    let a: u8 = 250;
    let b: u8 = 2;

    println!(
        "[checked_add] {a} + {b} = {}",
        a.checked_add(b).unwrap_or_default()
    );
    println!("[wrapping_add] {a} + {b} = {}", a.wrapping_add(b));
    println!("[saturating_add] {a} + {b} = {}", a.saturating_add(b));
    println!("[overflowing_add] {a} + {b} = {:?}", a.overflowing_add(b));

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!(
        "sum: {}, difference: {}, product: {}, quotient: {}, truncated: {}, remainder: {}",
        sum, difference, product, quotient, truncated, remainder
    );
}
