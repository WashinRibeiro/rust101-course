#[derive(Debug)]

enum Order {
    OrderDone,
    PaymentDone,
    OrderSent,
    OrderDelivery
}

fn get_number(x: u8) -> Option<u8> {
    if (x < 10) {
        Some(x)
    } else {
        None
    }
}

fn main() {
    dbg!(Order::OrderDelivery);
    dbg!(get_number(9));
}