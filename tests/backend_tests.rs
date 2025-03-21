use expression::backends::get_backend;

#[test]
fn test_get_backend_valid() {
    let backend = get_backend("swww");
    assert!(backend.is_ok());

    let backend = get_backend("feh");
    assert!(backend.is_ok());
}

#[test]
fn test_get_backend_nonexistent() {
    let backend = get_backend("girlfriend");
    assert!(backend.is_err());
}
