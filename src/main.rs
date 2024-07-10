use crate::keywords::OPERATORS;

#[path="modules/kv/mod.rs"]
mod kv;

#[path = "modules/keywords/mod.rs"]
mod keywords;

fn main() {
    let kv_pair: Vec<(String, i32)> = kv::create_kv::<i32>();
    kv::add_kv::<i32>(kv_pair, ("One", 1));
    println!("{}", OPERATORS.ADD);
}
