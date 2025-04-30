
use front_of_house::front_of_house::hosting;
use front_of_house::front_of_house::serving;
use front_of_house::eat_at_restaurant;

fn main() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();

    serving::take_order();
    serving::serve_order();
    serving::take_payment();

    eat_at_restaurant();

}
