// mod模块是crate下组织代码的一种形式
// 通过分层的mod可以提高可读性和重用性
// 并且可以灵活控制模块的可见行
mod another_front_of_house;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        pub mod back_of_house {

            pub enum Appetizer {
                Soup,
                Salad,
            }

            pub struct Breakfast {
                pub toast: String,
                seasonal_fruit: String,
            }

            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast {
                    Breakfast {
                        toast: String::from(toast),
                        seasonal_fruit: String::from("peaches"),
                    }
                }
            }

            fn fix_incorrect_order() {
                cook_order();
                super::serve_order();
            }

            fn cook_order() {}
        }


        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

pub fn eat_breakfast_at_restaurant() {
    let mut meal = crate::front_of_house::serving::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 下面这行会导致编译报错
    // meal.seasonal_fruit = String::from("blueberries");

}

pub fn after_eat_at_restaurant() {
    let order1 = crate::front_of_house::serving::back_of_house::Appetizer::Soup;
    let order2 = crate::front_of_house::serving::back_of_house::Appetizer::Salad;
}

use crate::front_of_house::hosting;

pub fn use_eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

pub fn eat_at_another_front_of_house() {
    crate::another_front_of_house::hosting::add_to_waitlist();

}