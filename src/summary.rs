/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

use crate::{news_article::NewsArticle, tweet::Tweet};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub enum SummaryType {
    NewsArticle,
    Tweet,
}

pub fn new(summary_type: SummaryType) -> Box<dyn Summary> {
    match summary_type {
        SummaryType::NewsArticle => Box::new(NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
        }),
        SummaryType::Tweet => Box::new(Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }),
    }
}

pub fn notify(item: &Box<dyn Summary>) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2(item: &impl Summary) {
    println!("[2] Breaking news! {}", item.summarize());
}
