use front_of_house::hosting;

mod front_of_house;

mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

pub fn just_eat() {
    hosting::add_to_waitlist();
}
