mod args;
mod spellcast;

fn main() {
    let args = args::parse();
    println!("Args: {args:?}");
}
