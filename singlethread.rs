use std::cell::RefCell;
use std::rc::Rc;

async fn dp(id: u64, value: Rc<RefCell<u8>>) {
    tokio::time::sleep(std::time::Duration::from_millis(1000 - id)).await;
    let mut value = value.borrow_mut();
    println!("{} {}", id, *value);
    *value += 1;
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let value = Rc::new(RefCell::new(0u8));

    let local = tokio::task::LocalSet::new();
    for id in 0..2 {
        local.spawn_local(dp(id, Rc::clone(&value)));
    }
    local.await;

    println!("{}", Rc::try_unwrap(value).unwrap().into_inner());
}
