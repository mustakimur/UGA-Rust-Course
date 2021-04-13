mod alt {
    pub mod altcode;
}
use crate::alt::altcode::hosting;

fn main() {
    hosting::add_to_waitlist();
}
