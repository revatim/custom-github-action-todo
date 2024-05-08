// Unit tests
// This is an conditional compilation attribute that tells the compiler to only compile the code when running tests
#[cfg(test)]
// Import the functions to be tested
// If this were in the same file as main then we would use super::function_name
use crate::process_line;
use crate::process_file;
use crate::process_directory;

#[test]
fn test_process_line() {
    assert_eq!(process_line("TODO: Implement this function"), 1);
    assert_eq!(process_line("This is a test line"), 0);
}

#[test]
fn test_process_file() {
    let path = "resources/test-assets/todo-example.rs";
    assert_eq!(process_file(path).unwrap(), 1);
    let path = "resources/test-assets/test-multiple-todos.rs";
    assert_eq!(process_file(path).unwrap(), 2);
}

#[test]
fn test_process_directory() {
    let path = "resources/test-assets";
    assert_eq!(process_directory(path).unwrap(), 3);
}           
