mod args;
mod dictionary;
mod spellcast;
mod utils;

fn main() {
    let args = args::parse();
    println!("Args: {args:?}");
    println!(
        "Dictionary has {:?} entries",
        dictionary::load_dictionary_file(&args.dictionary)
            .unwrap()
            .len()
    )
}
