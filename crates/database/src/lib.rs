

macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}


macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

static GLOBAL: usize = 42;

fn main() {
    say_hello!();
}