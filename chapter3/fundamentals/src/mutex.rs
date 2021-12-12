use std::rc::Rc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = Rc::new(vec![1; 10]);

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            let mut data_ref = data_ref.borrow_mut();
            data_ref[x] += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    dbg!(data);
}
