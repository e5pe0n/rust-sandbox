#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug)]
struct User<'a> {
    no: usize,
    name: &'a str,
}

impl Default for User<'_> {
    fn default() -> Self {
        Self {
            no: 1,
            name: "user1",
        }
    }
}

macro_rules! factory {
    () => {
        Default::default()
    };
    ($t:ident, $field:ident = $value:expr) => {
        $t {
            $field: $value,
            ..factory!()
        }
    };
    ($t:ident, $field:ident = $value:expr, $($_field:ident = $_value: expr),*) => {
        $t {
            $field: $value,
            ..factory!($t, $($_field = $_value), *)
        }
    }
}

fn main() {
    println!("{:?}", factory!(Point, x = 100));
    println!("{:?}", factory!(Point, y = 100));
    println!("{:?}", factory!(Point, y = 100, x = -100));
    // Point { x: 100, y: 0 }
    // Point { x: 0, y: 100 }
    // Point { x: -100, y: 100 }

    println!("{:?}", factory!(User, no = 100));
    println!("{:?}", factory!(User, name = "user100"));
    println!("{:?}", factory!(User, name = "user100", no = 100));
    // User { no: 100, name: "user1" }
    // User { no: 1, name: "user100" }
    // User { no: 100, name: "user100" }
}
