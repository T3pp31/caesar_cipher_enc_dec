/// # caesar_encrypt
///
/// can easily use caesar_encrypt and decrypt.
/// set text and shift number
///
/// # usage
/// ```
/// use caesar_cipher_enc_dec::caesar_cipher::caesar_encrypt;
/// fn main(){
/// let text = "I LOVE YOU";
/// let enc_text = caesar_encrypt(&text, 3);
/// let dec_text = caesar_encrypt(&text, -3);
/// }
/// ```
/// # Example
/// you can use this encrypt code for decrypt.
///  ```
/// use crate::caesar_cipher_enc_dec::caesar_cipher::caesar_encrypt;
/// let text = "I LOVE YOU";
/// for i in 0..26{
///     caesar_encrypt(&text, i)}
/// ```

pub mod caesar_cipher {
    pub fn caesar_encrypt(text: &str, shift: i16) -> String {
        let a_code = 'A' as i16;

        let is_az = |c| 'A' <= c && c <= 'Z';
        let conv = |c| (((c - a_code + shift + 26) % 26 + a_code) as u8) as char;
        let enc = |c| if is_az(c) { conv(c as i16) } else { c };
        return text.chars().map(|c| enc(c)).collect();
    }
}

#[cfg(test)]
mod tests {
    use crate::caesar_cipher::caesar_encrypt;
    #[test]
    fn test_caesar() {
        let text = "I LOVE YOU.";
        let enc_text = caesar_encrypt(&text, 3);
        let dec_text = caesar_encrypt(&enc_text, -3);
        println!("enc_test:{}", enc_text);
        println!("dec_text:{}", dec_text);

        assert_eq!(text, dec_text);
        assert_eq!("L ORYH BRX.", enc_text)
    }
}
