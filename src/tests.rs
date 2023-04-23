use crate::tools::TestReader;

const TESTS_FOLDER: &str = "./tests";

#[test]
fn test_assigment() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/assignment/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_case() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/assignment/prefix_operator.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}
