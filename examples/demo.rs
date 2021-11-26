use fingertrees::measure::Size;
use fingertrees::rc::FingerTree as RcFingerTree;
use fingertrees::{rc, ArcRefs, FingerTree, Measured, RcRefs, Refs};

const TEST_SIZE: usize = 1000000;

fn main() {
    let ft: RcFingerTree<_> = (0..TEST_SIZE).map(Size).collect();
    println!("Finished")
}
