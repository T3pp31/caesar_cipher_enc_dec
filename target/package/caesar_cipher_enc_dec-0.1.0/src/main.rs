use caesar_cipher_enc_dec::caesar_cipher::caesar_encrypt;
fn main() {
    // 暗号化と復号化 --- (*1)
    let text = "I LOVE YOU.";
    let enc_text = caesar_encrypt(&text, 3); // 暗号化
    let dec_text = caesar_encrypt(&enc_text, -3); // 復号化
    println!("文字列: {}", text);
    println!("暗号化: {}", enc_text);
    println!("復号化: {}", dec_text);
}
