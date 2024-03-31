# caesar_encrypt

can easily use caesar_encrypt and decrypt.
set text and shift number

# usage

```
use caesar_cipher_enc_dec::caesar_cipher::caesar_encrypt;
fn main(){
    let text = "I LOVE YOU";
    let enc_text = encrypt(&text, 3);
    let dec_text = encrypt(&enc_text, -3);
    let dec_text2 = decrypt(&enc_text, 3);
}
 ```

# Example

 you can use this encrypt code for decrypt.

  ```
 use crate::caesar_cipher_enc_dec::caesar_cipher::caesar_encrypt;
 let text = "L ORYH BRX.";
 for i in 0..26{
    let mut result = encrypt(&text, i)}
    println!("{}", result);
```

# link

<https://crates.io/crates/caesar_cipher_enc_dec>

# これからやること．

エラーハンドリングをunwrapやexpectなどのpanicするメソッドからResultやOptionへ移行
単体テスト，結合テストの作成
