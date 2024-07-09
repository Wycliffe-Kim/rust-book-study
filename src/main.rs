/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

fn get_closure_with_string(value: String) -> Box<dyn FnOnce() -> String> {
    Box::new(|| value)
}

fn get_closure_with_i32(value: i32) -> impl FnOnce() -> i32 {
    move || value
}

fn main() {
    let default_string = String::from("default");
    println!(
        "{}",
        Some(String::from("data")).unwrap_or_else(get_closure_with_string(default_string.clone()))
    );
    println!(
        "{}",
        None.unwrap_or_else(get_closure_with_string(default_string.clone()))
    );
    let default_i32 = 0;
    println!(
        "{}",
        Some(5).unwrap_or_else(get_closure_with_i32(default_i32))
    );
    println!("{}", None.unwrap_or_else(get_closure_with_i32(default_i32)));
}
