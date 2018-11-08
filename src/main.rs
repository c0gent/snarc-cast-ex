extern crate snarc;

use std::sync::Arc;
use snarc::Snarc;

trait Material {}

struct Concrete;

impl Material for Concrete {}

fn main() {
	let a_c: Arc<Concrete> = Arc::new(Concrete);
    let a_m: Arc<Material> = a_c as _;

    let s_c: Snarc<Concrete> = Snarc::new(Concrete);
    let s_m: Snarc<Material> = s_c as _;
}
