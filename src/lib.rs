#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    clippy::borrow_interior_mutable_const
)]

mod kaz_jc_sideb;
mod kaz_10f_fsmash;

#[skyline::main(name = "kazuya_mishima_wins")]
pub fn main() {
    kaz_jc_sideb::install();
    kaz_10f_fsmash::install();
}