#[allow(dead_code)]
#[easy_lg::simple]
struct Complex {
    data: &T,
    id: i32,
}

fn main() {
    let _complex = Complex {
        data: &"Hello",
        id: 42,
    };
}
