use expression::backends::get_backend;

#[test]
fn test_get_backend_valid() {
    let backend = get_backend("swww");
    assert!(backend.is_ok(), "Expected 'swww' backend to be found");

    let backend = get_backend("feh");
    assert!(backend.is_ok(), "Expected 'feh' backend to be found");
}

#[test]
fn test_get_backend_nonexistent() {
    let backend = get_backend("girlfriend");
    assert!(
        backend.is_err(),
        "Did not expect nonexistent girlfriend's backend to be found"
    );
}
