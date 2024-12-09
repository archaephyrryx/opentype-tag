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
    TooLong(usize), // anything above 8 characters can't possibly be magic
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

const HEX: u32 = 16;

fn is_code_word(code: &str, word: &str) -> bool {
    code.chars().all(|c| char::is_ascii_hexdigit(&c)) &&
        word.chars().all(|c| char::is_ascii_alphabetic(&c))
}

fn code_to_u8(code: &str) -> Result<u8, ReverseMagicError> {
    u8::from_str_radix(code, HEX)
        .map_err(|_| ReverseMagicError::BadCode(code.to_owned()))
}

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

        l @ 5..=7 => {
            // Potentially ambiguous cases - slightly harder

            /* Fall back on simple cases */

            // Case 1: Len=5, being 3 letters and 1 hex number at start or end
            if l == 5 {
                let (lcode, after) = (&image[0..2], &image[2..]);
                let (before, rcode) = (&image[0..3], &image[3..]);
                // left case
                if is_code_word(lcode, after) {
                    let mut bytes = [0u8; 4];
                    bytes[0] = code_to_u8(lcode)?;
                    for (ix, c) in after.chars().enumerate() {
                        bytes[ix + 1] = c as u8;
                    }
                    u32::from_be_bytes(bytes)
                } else if is_code_word(rcode, before) {
                    let mut bytes = [0u8; 4];
                    for (ix, c) in before.chars().enumerate() {
                        bytes[ix] = c as u8;
                    }
                    bytes[3] = code_to_u8(rcode)?;
                    u32::from_be_bytes(bytes)
                } else {
                    unimplemented!("arbitrary 5-byte reverse-magic logic is incomplete");
                }
            }
            else {
                unimplemented!("reverse-magic not implemented for ambiguous sequences");
            }
        }
        8 => {
            /* Easy case - when char-length is exactly 8 */
            let mut bytes = [0u8; 4];
            for ix in 0..4 {
                let ascii = &image[ix*2..(ix+1)*2];
                let code = u8::from_str_radix(ascii, HEX)
                    .map_err(|_| {
                        ReverseMagicError::BadCode(ascii.to_owned())
                    })?;
                bytes[ix] = code;
            }
            u32::from_be_bytes(bytes)
        }
        l @ 9.. => return Err(ReverseMagicError::TooLong(l)),
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
