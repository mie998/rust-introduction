use std::fmt;

struct My {
    id: i32,
}

impl Drop for My {
    fn drop(&mut self) {
        println!("Dropping my {}", self.id);
    }
}

impl fmt::Pointer for My {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "My")
    }
}

impl fmt::Display for My {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "My")
    }
}

fn main() {
    let mut my1 = My { id: 1 };

    let my2 = &mut my1;
    my2.id = 2;
    println!("{}", my2);

    let my3 = &mut my1;
    my3.id = 3;
    println!("{}", my3);
}
