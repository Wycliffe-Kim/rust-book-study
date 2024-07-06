/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */
pub mod back_of_house;
pub mod front_of_house;

fn front_of_house() {
    println!("----- front_of_house -----");
    // Absolute path
    crate::restaurant::front_of_house::hosting::add_to_waitlist(1);

    // Relative path
    front_of_house::hosting::add_to_waitlist(2);
}

fn back_of_house() {
    println!("----- back_of_house -----");

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::breakfast::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    let order1 = back_of_house::appetizer::Appetizer::Soup;
    let order2 = back_of_house::appetizer::Appetizer::Salad;

    println!("{:?}", order1);
    println!("{:?}", order2);
}

pub fn eat_at_restaurant() {
    front_of_house();
    back_of_house();
}
