
fn main() {
    run_single_responsibility_before();

    run_single_responsibility_after();
}

fn run_single_responsibility_before() {
    println!("running `run_single_responsibility_before`:\n");

    use solid::single_responsibility_before::*;

    let mut order = Order::new();

    order.add_item("Keyboard", 1, 50.0);
    order.add_item("SSD", 1, 150.0);
    order.add_item("USB cable", 2, 5.0);

    println!("{}", order.total_price());
    order.pay("debit", "0372846");

    println!("------------");
}

fn run_single_responsibility_after() {
    println!("running `run_single_responsibility_after`:\n");

    use solid::single_responsibility_after::*;

    let mut order = Order::new();

    order.add_item("Keyboard", 1, 50.0);
    order.add_item("SSD", 1, 150.0);
    order.add_item("USB cable", 2, 5.0);

    println!("{}", order.total_price());
    
    let processor = PaymentProcessor {};

    processor.pay_debit(&mut order, "0372846");

    println!("------------");
}