extern crate libcratesio;

use libcratesio::{CratesIO, Crate, Error, ErrorKind};

#[test]
fn basic_data() {
    let krate = Crate::by_name("libcratesio").unwrap();
    println!("{:?}", krate);
    assert_eq!(krate.name, "libcratesio");
}

#[test]
fn raw_data() {
    let krate = CratesIO::query("libcratesio")
        .unwrap()
        .as_json()
        .unwrap();
    println!("{:#?}", krate);
    assert!(true);
}

#[test]
fn versions_iter() {
    let krate = Crate::by_name("libcratesio").unwrap();
    for ver in krate.versions.iter() {
        println!("{:?}", ver);
    }
    assert!(true);
}

#[test]
fn error() {
    let no_such_crate = Crate::by_name("no_such_crate");
    assert!(no_such_crate.is_err());
    let err = no_such_crate.unwrap_err();
    if let Error(ErrorKind::CratesIOError(response), _) = err {
        assert_eq!(response.detail(), "Not Found");
    } else {
        assert!(false);
    }
}
