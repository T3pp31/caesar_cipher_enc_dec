/// # caesar_encrypt
///
/// can easily use caesar_encrypt and decrypt.
/// set text and shift number
///
/// # usage
/// ```
/// use caesar_cipher_enc_dec::caesar_cipher::encrypt;
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
///     caesar_encrypt(&text, i);
/// }
/// ```

pub fn encrypt(text: &str, shift: i16) -> String {
    let a_code = 'A' as i16;

    let is_az = |c| 'A' <= c && c <= 'Z';
    let conv = |c| (((c - a_code + shift + 26) % 26 + a_code) as u8) as char;
    let enc = |c| if is_az(c) { conv(c as i16) } else { c };
    return text.chars().map(|c| enc(c)).collect();
}
