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
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let v1: Vec<i32> = Vec::from(arr);
    let v_iter: Vec<i32> = v1.iter().filter(|x| *x % 2 == 0).map(|x| x * 2).collect();
    let v_into_iter: Vec<i32> = v1
        .into_iter()
        .filter(|x| *x % 2 == 0)
        .map(|x| x * 2)
        .collect();

    let mut v2 = Vec::from(arr);
    v2.iter_mut()
        // .filter(|x| **x % 2 == 0)
        .for_each(|x| {
            *x = *x * 2;
        });
    // .map(|x| *x * 2)
    // .collect();

    println!("{:?}", v_iter);
    println!("{:?}", v_into_iter);
    println!("{:?}", v2);
}
