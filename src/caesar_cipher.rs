/// # caesar_encrypt
///
/// can easily use caesar_encrypt and decrypt.
/// set text and shift number
///
/// # usage
/// ```
/// use caesar_cipher_enc_dec::caesar_cipher::{encrypt, decrypt};
/// fn main(){
/// let text = "I LOVE YOU";
/// let enc_text = encrypt(&text, 3);
/// let dec_text = decrypt(&enc_text, 3);
/// let dec_text_2 = encrypt(&enc_text, -3);
/// }
/// ```
/// # Example
/// you can use this encrypt code for decrypt.
///  ```
/// use caesar_cipher_enc_dec::caesar_cipher::encrypt;
/// let text = "I LOVE YOU";
/// for i in 0..26{
///     encrypt(&text, i);
/// }
/// ```

pub fn encrypt(text: &str, shift: i16) -> String {
    let a_code = 'A' as i16;

    let is_az = |c| 'A' <= c && c <= 'Z';
    let conv = |c| (((c - a_code + shift + 26) % 26 + a_code) as u8) as char;
    let enc = |c| if is_az(c) { conv(c as i16) } else { c };
    return text.chars().map(|c| enc(c)).collect();
}
pub fn decrypt(text: &str, shift: i16) -> String {
    return encrypt(text, -shift);
}
