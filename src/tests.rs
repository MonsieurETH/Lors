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

//#[test]
//fn test_benchmark() {
//    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/benchmark/*.lox"));
//    for key in tr.iter() {
//        println!("{}", key);
//        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
//        assert_eq!(expected, result)
//    }
//}

#[test]
fn test_block() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/block/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_bool() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/bool/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_call() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/call/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

//#[test]
//fn test_class() {
//    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/class/*.lox"));
//    for key in tr.iter() {
//        println!("{}", key);
//        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
//        assert_eq!(expected, result)
//    }
//}

#[test]
fn test_comments() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/comments/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_constructor() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/constructor/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

//#[test]
//fn test_field() {
//    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/field/*.lox"));
//    for key in tr.iter() {
//        println!("{}", key);
//        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
//        assert_eq!(expected, result)
//    }
//}

#[test]
fn test_for() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/for/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_case() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/for/closure_in_body.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}
