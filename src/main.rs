#[path="modules/kv/mod.rs"]
mod kv;

fn main() {
    let kv_pair: Vec<(String, i32)> = kv::create_kv::<i32>();
    kv::add_kv::<i32>(kv_pair, ("One", 1));
    println!("kv_pair");
}
