// Using the langtop module
mod langtop;
mod langdetector;

// Importing the Langtop struct
use langtop::Langtop;

fn main() {
    println!("***** Langtop Engine *****");

    let mut langtop = Langtop::new("./hello/abc");

    langtop.execute();

    println!("\n***** Summary *****");
    langtop.print_summary();

    println!("***** END *****");
}
