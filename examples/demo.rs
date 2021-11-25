use calcit_fingertrees::measure::Size;
use calcit_fingertrees::monoid::Sum;
use calcit_fingertrees::FingerTree;
use calcit_fingertrees::RcRefs;

fn main() {
    let a: FingerTree<RcRefs, _> = vec!["one", "two", "three", "four", "five"]
        .into_iter()
        .map(Size)
        .collect();

    println!("a: {:?}", a);
    println!("a: {:?}", a.split_right(|measure| *measure > Sum(1)));
    println!("a: {:?}", a.split_right(|measure| *measure > Sum(2)));
    println!("a: {:?}", a.split_right(|measure| *measure > Sum(3)));
    println!("a: {:?}", a.split_right(|measure| *measure > Sum(4)));
    println!("a: {:?}", a.split_right(|measure| *measure > Sum(5)));
    println!("a: {:?}", a.split_right(|measure| *measure > Sum(6)));

    println!("a: {:?}", a.split_left(|measure| *measure > Sum(1)));
    println!("a: {:?}", a.split_left(|measure| *measure > Sum(2)));
    println!("a: {:?}", a.split_left(|measure| *measure > Sum(3)));
    println!("a: {:?}", a.split_left(|measure| *measure > Sum(4)));
    println!("a: {:?}", a.split_left(|measure| *measure > Sum(5)));
    println!("a: {:?}", a.split_left(|measure| *measure > Sum(6)));
}
