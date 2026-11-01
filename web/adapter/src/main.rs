use std::{env, fs::File, io::Read};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = env::args()
        .nth(1)
        .ok_or("Usage: packet-inspect REQUEST.json | --examples")?;
    if arg == "--examples" {
        println!("{}", packet_web::examples());
        return Ok(());
    }
    let mut text = String::new();
    File::open(arg)?
        .take((packet_web::MAX_REQUEST + 1) as u64)
        .read_to_string(&mut text)?;
    println!("{}", packet_web::inspect(&text));
    Ok(())
}
