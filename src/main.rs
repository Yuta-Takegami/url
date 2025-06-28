use percent_encoding::percent_decode;
use std::env;
use anyhow::{ensure, Context, Error, Result};

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    //let input = "foo%20bar";
    //let decoded = percent_decode(input.as_bytes()).decode_utf8();
    Ok(println!("{}", decode(input)?))
}

fn decode(input: &str) -> Result<String, Error>{
    let decoded = percent_decode(input.as_bytes()).decode_utf8()?;
    Ok(decoded.to_string())
}

# [cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn decode_space_ok(){
        let expected = "foo bar";
        let input = "foo%20bar";
        let actual = decode(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn decode_ascii_ok(){
        let expected = r##" !"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ `"##;
        let input = r##"%20%21%22%23%24%25%26%27%28%29%2A%2B%2C%2D%2E%2F0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~ `"##;
        let actual = decode(input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn decode_invalid_utf8_ng(){
        let input = "%93%FA%96%7B%8C%EA%0D%0A";
        decode(input).unwrap();
    }
}
