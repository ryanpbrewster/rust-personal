/**
Each character on a computer is assigned a unique code and the preferred standard is ASCII (American Standard Code for Information Interchange). For example, uppercase A = 65, asterisk (*) = 42, and lowercase k = 107.

A modern encryption method is to take a text file, convert the bytes to ASCII, then XOR each byte with a given value, taken from a secret key. The advantage with the XOR function is that using the same encryption key on the cipher text, restores the plain text; for example, 65 XOR 42 = 107, then 107 XOR 42 = 65.

For unbreakable encryption, the key is the same length as the plain text message, and the key is made up of random bytes. The user would keep the encrypted message and the encryption key in different locations, and without both "halves", it is impossible to decrypt the message.

Unfortunately, this method is impractical for most users, so the modified method is to use a password as a key. If the password is shorter than the message, which is likely, the key is repeated cyclically throughout the message. The balance for this method is using a sufficiently long password key for security, but short enough to be memorable.

Your task has been made easy, as the encryption key consists of three lower case characters. Using cipher.txt, a file containing the encrypted ASCII codes, and the knowledge that the plain text must contain common English words, decrypt the message and find the sum of the ASCII values in the original text.
*/

pub fn solve(cipher_text: &[u8], key_length: usize) -> Vec<u8> {
    XorKeys::of_length(key_length)
        .max_by_key(|key| score(decrypt(cipher_text, key)))
        .expect("length must be > 0")
}

fn decrypt(cipher_text: &[u8], key: &[u8]) -> Vec<u8> {
    cipher_text.iter().zip(key.iter().cycle()).map(|(c, k)| c ^ k).collect()
}
fn score(plain_text: Vec<u8>) -> u64 {
    plain_text.iter().map(|&ch| char_score(ch)).sum()
}

fn char_score(ch: u8) -> u64 {
    if is_vowel(ch as char) {
        1
    } else {
        0
    }
}

fn is_vowel(ch: char) -> bool {
    ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u'
}


const LO: u8 = b'a';
const HI: u8 = b'z';
struct XorKeys {
    bytes: Vec<u8>,
    done: bool
}
impl XorKeys {
    fn of_length(len: usize) -> XorKeys {
        XorKeys { bytes: vec![LO; len], done: false }
    }
}

impl Iterator for XorKeys {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let cur = self.bytes.clone();
        self.done = true;
        for i in 0..self.bytes.len() {
            if self.bytes[i] == HI {
                self.bytes[i] = LO;
            } else {
                self.bytes[i] += 1;
                self.done = false;
                break;
            }
        }
        Some(cur)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn keys() {
        let mut count: u32 = 0;
        for _ in XorKeys::of_length(3) {
            count += 1;
        }

        let expected = ((HI - LO + 1) as u32).pow(3);
        assert_eq!(count, expected);
    }

    #[test]
    fn main() {
        let mut fin = File::open(Path::new("data/p059_main.in")).expect("couldn't open file");
        let mut s = String::new();
        fin.read_to_string(&mut s).expect("could not read file");
        let cipher_text = s.as_bytes();

        let key = solve(cipher_text, 3);
        let plain_text = decrypt(cipher_text, key.as_slice());
        println!("{:?}", plain_text.iter().map(|&v| v as char).collect::<String>());
        let ans = plain_text.iter().map(|&n| n as u32).sum::<u32>();
        assert_eq!(ans, 107359);
    }
}
