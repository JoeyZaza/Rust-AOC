use lib::days::day3;
use lib::read_file;
fn main() {
    let path = "../input/day3.txt";

    let file = read_file(path);

    let res = day3::main(file); //DEBUGGING DONE: Capital Z's were getting returned as Some(0) instead of Some(52)
                                // cuz I'm an idiot and was looping through range 1..table.len() and forgetting that
                                //the table's length was 52 and Rust's range is upper bounds exclusive (or whatever it's called)
                                //Edit: took me way too long to debug this....am I dumb?
                                //Edit: yeah I'm dumb
                                //Edit: also I can't actually debug because GDB refuses to work for me..probably cuz I'm dumb
    println!("{res}");
}
