use crate::tools::TestReader;

const TESTS_FOLDER: &str = "./tests";

#[test]
fn test_assigment() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/assignment/*.lox"));
    for key in tr.iter() {
        println!("key: {}", key);
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

#[test]
fn test_class() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/class/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_closure() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/closure/*.lox"));
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

#[test]
fn test_field() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/field/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

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
fn test_function() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/function/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_if() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/if/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_inheritance() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/inheritance/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_limit() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/limit/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_logical_operator() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/logical_operator/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_method() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/method/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_nil() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/nil/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_number() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/number/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_operator() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/operator/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_other() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/other/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_print() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/print/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_regression() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/regression/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_return() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/return/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_scanning() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/scanning/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_super() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/super/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_this() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/this/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_variable() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/variable/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}

#[test]
fn test_while() {
    let tr = TestReader::new(&(TESTS_FOLDER.to_string() + "/while/*.lox"));
    for key in tr.iter() {
        println!("{}", key);
        let (expected, result) = tr.run_test(&(TESTS_FOLDER.to_string() + "/" + key));
        assert_eq!(expected, result)
    }
}
