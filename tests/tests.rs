#[cfg(test)]

mod tests{
    use caesar_cipher_enc_dec::caesar_cipher::{decrypt, encrypt};
    #[test]
    fn encrypt_text() {
        let text: &str = "I LOVE YOU.";
        let enc_text: String = encrypt(&text, 3);
        let dec_text:String = encrypt(&enc_text, -3);
        println!("enc_test:{}", enc_text);
        println!("dec_text:{}", dec_text);

        assert_eq!(text, dec_text);
        assert_eq!("L ORYH BRX.", enc_text)
    }
    #[test]
    fn decrypt_text(){
        let text: &str = "L ORYH BRX.";
        let dec_text = decrypt(&text, 3);

        assert_eq!("I LOVE YOU.", dec_text)
    }

}
