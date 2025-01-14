use webbase::AccessTokenRsa;

pub const RSA_PRIVATE: &str = include_str!("../../test/rsa_private.key");
pub const RSA_PUBLIC: &str = include_str!("../../test/rsa_public.key");

#[tokio::main]
pub async fn main() {
    let token_info = AccessTokenRsa::encode_token(
        1,
        &"test_acount".into(),
        &"test_name".into(),
        &"test.app".into(),
        8,
        &RSA_PRIVATE.into(),
    )
    .unwrap();
    println!("{}", token_info);

    let decode_info = AccessTokenRsa::decode_token(&token_info, &RSA_PUBLIC.into()).unwrap();
    println!("{:?}", decode_info);
}
