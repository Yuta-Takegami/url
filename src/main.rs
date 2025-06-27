use url::percent_encoding::percent_decode;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    //let input = "foo%20bar";
    let decoded = percent_decode(input.as_bytes()).decode_utf8();
    println!("{}", decode(input));
}

fn decode(input: &str) -> String{
    percent_decode(input.as_bytes()).decode_utf8().unwrap().to_string()
}

# [cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn decode_space_ok(){
        let expected = "foo bar";
        let input = "foo%20bar";
        let actual = decode(input);
        assert_eq!(expected, actual);
    }
}
