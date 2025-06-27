use url::percent_encoding::percent_decode;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];

    //let input = "foo%20bar";
    let decoded = percent_decode(input.as_bytes()).decode_utf8();
    println!("{}", decoded.unwrap());
}
