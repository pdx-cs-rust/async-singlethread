use std::rc::Rc;
use std::cell::RefCell;

async fn dp(value: Rc<RefCell<u8>>) {
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let mut value = value.borrow_mut();
    println!("{}", *value);
    *value += 1;
}


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let value = Rc::new(RefCell::new(0u8));

    let local = tokio::task::LocalSet::new();

    local.spawn_local(dp(Rc::clone(&value)));
    local.spawn_local(dp(Rc::clone(&value)));
    local.await;

    println!("{}", *value.borrow());
}
