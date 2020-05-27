fn get_version() -> u16 {
    1000 // could use: return 1000; , only use return for an early return
}
// comment
fn usage() {
    let the_version: u16;
    the_version = get_version();

    println!("tinymd, a markdown compiler written by richm.");
    println!("Version {}", the_version);
}

fn main() {
    usage();
}
