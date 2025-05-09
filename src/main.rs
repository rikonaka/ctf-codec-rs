use clap::Parser;

/// A collection of codecs for the codec problems I encountered in CTF competitions.
#[derive(Parser, Debug)]
#[command(version, author = "RikoNaka", about, long_about = None)]
struct Args {
    /// Encode the input into octal
    #[arg(long = "eo", default_value = "")]
    encode_octal: String,

    /// Decode the octal input into normal
    #[arg(long = "do", default_value = "")]
    decode_octal: String,

    /// Encode the input into hex
    #[arg(long = "eh", default_value = "")]
    encode_hex: String,

    /// Decode the hex input into normal
    #[arg(long = "dh", default_value = "")]
    decode_hex: String,

    /// Set the delimiter
    #[arg(short, long, default_value = "\\")]
    delimiter: String,

    /// Try best to codec
    #[arg(short, long, action, default_value = "false")]
    try_best: bool,
}

fn encode_octal(input: &str, delimiter: &str) -> String {
    let input_bytes = input.as_bytes();
    let mut ret_vec = Vec::new();
    for i in input_bytes {
        ret_vec.push(format!("{}{:o}", delimiter, i));
    }
    ret_vec.join("")
}

fn decode_octal(input: &str, delimiter: &str, try_best: bool) -> String {
    let input_split: Vec<&str> = input
        .split(delimiter)
        .filter(|x| x.trim().len() > 0)
        .collect();

    let mut ret_vec = Vec::new();
    for i in input_split {
        let value = u8::from_str_radix(i, 8).expect(&format!("convert octal [{}] failed", input));
        ret_vec.push(value);
    }

    if try_best {
        let ret = String::from_utf8_lossy(&ret_vec).to_string();
        ret
    } else {
        let ret = String::from_utf8(ret_vec)
            .expect("convert [u8] to string failed, try --try-best parameter");
        ret
    }
}

fn encode_hex(input: &str, delimiter: &str) -> String {
    let input_bytes = input.as_bytes();
    let mut ret_vec = Vec::new();
    for i in input_bytes {
        ret_vec.push(format!("{}{:x}", delimiter, i));
    }
    ret_vec.join("")
}

fn main() {
    let args = Args::parse();

    if args.encode_octal.len() > 0 {
        let ret = encode_octal(&args.encode_octal, &args.delimiter);
        println!("{}", ret);
    } else if args.decode_octal.len() > 0 {
        let ret = decode_octal(&args.decode_octal, &args.delimiter, args.try_best);
        println!("{}", ret);
    } else if args.encode_hex.len() > 0 {
        let ret = encode_hex(&args.encode_hex, &args.delimiter);
        println!("{}", ret);
    }
}
