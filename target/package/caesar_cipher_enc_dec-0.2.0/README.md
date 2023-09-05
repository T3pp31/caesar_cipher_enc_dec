# caesar_encrypt

can easily use caesar_encrypt and decrypt.
set text and shift number

# usage

```
use caesar_cipher_enc_dec::caesar_cipher::caesar_encrypt;
fn main(){
    let text = "I LOVE YOU";
    let enc_text = caesar_encrypt(&text, 3);
    let dec_text = caesar_encrypt(&text, -3);
}
 ```

# Example

 you can use this encrypt code for decrypt.

  ```
 use crate::caesar_cipher_enc_dec::caesar_cipher::caesar_encrypt;
 let text = "I LOVE YOU";
 for i in 0..26{
     caesar_encrypt(&text, i)}
```
