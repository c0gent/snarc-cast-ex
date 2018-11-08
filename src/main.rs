extern crate snarc;

use std::sync::Arc;
use snarc::{Snarc, Weak as SnarcWeak};

trait Material {}

struct Concrete;

impl Material for Concrete {}

fn main() {
	let a_c: Arc<Concrete> = Arc::new(Concrete);
    let _a_m: Arc<Material> = a_c as _;

    let s_c: Snarc<Concrete> = Snarc::new(Concrete);
    let _s_m: Snarc<Material> = s_c.clone() as _;

    let sw_c: SnarcWeak<Concrete> = Snarc::downgrade(&s_c);
    let _sw_m: SnarcWeak<Material> = sw_c as _;
}
