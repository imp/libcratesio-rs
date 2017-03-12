extern crate libcratesio;

use libcratesio::Crate;

#[test]
fn basic_data() {
    let krate = Crate::by_name("requests").unwrap();
    println!("{:?}", krate);
    assert_eq!(krate.name, "requests");
}

#[test]
fn raw_data() {
    let krate = Crate::json_data("requests").unwrap();
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
