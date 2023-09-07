
fn main() {
    let value_one = 24;

    let value_two = 67;
    let value = explicit_lifetime(&value_one, &value_two);
}

fn explicit_lifetime<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32 {
    if p1 > p2 {
        p1
    } else {
        p2
    }
}
