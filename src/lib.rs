pub enum Fruit {
    Apple,
    Orange,
}

pub fn tested1(val: isize) -> isize {
    if val % 2 == 1 {
        val
    } else if val == 0 {
        42
    } else {
        -val
    }
}

pub fn tested2(val: Fruit) {
    match val {
        Fruit::Apple => println!("Apple"),
        Fruit::Orange => println!("Orange"),
    }
}

pub fn untested(val: f32) -> f32 {
    val
}

#[test]
fn unit_test1() {
    assert_eq!(23,tested1(23));
}

#[test]
fn unit_test2() {
    tested2(Fruit::Apple);
}
