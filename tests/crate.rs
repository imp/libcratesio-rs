extern crate libcratesio;

use libcratesio::{CratesIO, Crate, Error};

#[test]
fn basic_data() {
    let krate = Crate::by_name("requests").unwrap();
    println!("{:?}", krate);
    assert_eq!(krate.name, "requests");
}

#[test]
fn raw_data() {
    let krate = CratesIO::query("requests").unwrap().as_json().unwrap();
    println!("{:#?}", krate);
    assert!(true);
}

#[test]
fn versions_iter() {
    let krate = Crate::by_name("requests").unwrap();
    for ver in krate.versions.iter() {
        println!("{:?}", ver);
    }
    assert!(true);
}

#[test]
fn error() {
    let err = Crate::by_name("xxx").unwrap_err();
    println!("{:?}", err);
    assert!(true);
}
