mod alt {
    pub mod altcode;
}

mod code;
mod opt;

use crate::alt::altcode::hosting;
//use crate::alt::altcode::add_to_waitlist;

fn main() {
    hosting::add_to_waitlist();
    code::how_to_use_mod();
    opt::good::good_code();
}
