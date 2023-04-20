// cargo run --bin unique --release

use win_unique::COMLibrary;
use win_unique::{mdm_dev_detail_ext01, product};

fn main() {
    let _com_con = COMLibrary::new().unwrap(); // initialise security context

    println!("{:#?}\n{:#?}", product(), mdm_dev_detail_ext01());
}
