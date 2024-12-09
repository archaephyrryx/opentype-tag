use clap::Parser;

#[derive(Parser)]
struct Command {
    #[arg(long, default_value_t = false)]
    reverse: bool,
    input: String,
}

#[derive(Debug)]
enum ReverseMagicError {
    TooShort(u8), // Anything less than 4 characters can't possibly be magic
    BadCode(String), // A code that doesn't reverse correctly
    TooLong(u8), // anything above 8 characters can't possibly be magic
}

impl std::fmt::Display for ReverseMagicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooShort(len) => write!(f, "input too short ({len}) for solution to exist"),
            Self::TooLong(len) => write!(f, "input too long ({len}) for solution to exist"),
            Self::BadCode(code) => write!(f, "irreversible token `{code}`"),
        }
    }
}

impl std::error::Error for ReverseMagicError { }


fn reverse_magic(image: &'_ str) -> Result<u32, ReverseMagicError> {
    let preimage = match image.len() {
        l @ 0..=3 => return Err(ReverseMagicError::TooShort(l as u8)),
        4 => {
            /* Easy case - when char-length is exactly 4 */
            let mut bytes = [0u8; 4];
            for (ix, c) in image.chars().enumerate() {
                bytes[ix] = c as u8;
            }
            u32::from_be_bytes(bytes)
        }
        5..=7 => {
            // Potentially ambiguous cases - slightly harder
            unimplemented!("reverse-magic not implemented for ambiguous sequences");
        }
        8 => {
            /* Easy case - when char-length is exactly 8 */
            let mut bytes = [0u8; 4];
            for ix in 0..4 {
                let ascii = &image[ix*2..(ix+1)*2];
                let code = ascii
                    .parse::<u8>()
                    .map_err(|_| {
                        ReverseMagicError::BadCode(ascii.to_owned())
                    })?;
                bytes[ix] = code;
            }
            u32::from_be_bytes(bytes)
        }
        l @ 9.. => return Err(ReverseMagicError::TooLong(l as u8)),
    };
    Ok(preimage)
}

fn format_magic(magic: u32) -> String {
    let bytes = magic.to_be_bytes();
    let show = |b: u8| {
        if b.is_ascii_alphanumeric() {
            String::from(b as char)
        } else {
            format!("{:02x}", b)
        }
    };
    format!(
        "{}{}{}{}",
        show(bytes[0]),
        show(bytes[1]),
        show(bytes[2]),
        show(bytes[3])
    )
}

fn main() {
    let command = Command::parse();
    if command.reverse {
        let solution = reverse_magic(command.input.as_str());
        match solution {
            Err(e) => eprintln!("No solution: {e}"),
            Ok(preimage) => println!("`{}` <= {}", command.input, preimage),
        }
    } else {
        let value = command.input;
        let val = value.parse::<u32>().expect("could not parse as u32");
        let oput = format_magic(val);
        println!("{value} => \"{oput}\"");
    }
}
