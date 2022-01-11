fn main() {
    let actual = rpassword::read_password_from_tty(Some("your password: ")).unwrap();
    assert_eq!(actual, "abc");
}
