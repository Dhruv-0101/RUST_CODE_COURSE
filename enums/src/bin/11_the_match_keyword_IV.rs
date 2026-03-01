#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            OnlineOrderStatus::Delivered => {
                println!("Your item has arrived")
            }
            other_status => {
                println!("Your item is {other_status:?}")
            }
        }
    }
}

fn main() {
    OnlineOrderStatus::Shipped.check();
    let order_status = OnlineOrderStatus::Shipped;
    order_status.check();
    order_status.check(); // ✅ Still works because `check(&self)` borrows immutably, no ownership move
}
