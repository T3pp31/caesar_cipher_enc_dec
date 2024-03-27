#[cfg(test)]

mod tests{
    use caesar_cipher_enc_dec::caesar_cipher::encrypt;
    #[test]
    fn test_caesar() {
        let text: &str = "I LOVE YOU.";
        let enc_text: String = encrypt(&text, 3);
        let dec_text:String = encrypt(&enc_text, -3);
        println!("enc_test:{}", enc_text);
        println!("dec_text:{}", dec_text);

        assert_eq!(text, dec_text);
        assert_eq!("L ORYH BRX.", enc_text)
    }
}
