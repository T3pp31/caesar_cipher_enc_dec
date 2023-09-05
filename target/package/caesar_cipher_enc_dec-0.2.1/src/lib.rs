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
        let z_code = 'Z' as i16;

        let mut result = String::new();

        for ch in text.chars() {
            //小文字を大文字に変換
            let ch = if ch.is_lowercase() {
                ch.to_ascii_uppercase()
            } else {
                ch
            };
            let code = ch as i16;

            if a_code <= code && code <= z_code {
                let enc = ((code - a_code + shift + 26) % 26 + a_code) as u8 as char;
                result.push(enc);
            } else {
                result.push(ch);
            }
        }
        return result;
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
