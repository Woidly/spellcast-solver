mod args;
mod dictionary;
mod spellcast;

fn main() {
    let args = args::parse();
    println!("Args: {args:?}");
}
