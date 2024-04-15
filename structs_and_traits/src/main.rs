trait Bite {
    fn bite(self: &mut Self) {}
}

#[derive(Debug)] // include this line right before your struct definition
struct Grapes {
    grapes_left: i32,
}

impl Bite for Grapes {
    fn bite(self: &mut Self) {
        self.grapes_left -= 1
    }
}

fn main() {
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { grapes_left: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes.grapes_left);

    fn bunny_nibbles<T: Bite>(el: &mut T) {
        for _ in 0..3 {
            el.bite();
        }
    }

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);
}

#[derive(Debug)]
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        self.percent_left *= 0.8;
    }
}
