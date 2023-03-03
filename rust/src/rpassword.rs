fn main() {
    let actual = rpassword::prompt_password("your password: ").unwrap();
    assert_eq!(actual, "abc");
}
