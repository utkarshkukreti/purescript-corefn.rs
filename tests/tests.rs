extern crate purescript_corefn;

#[test]
fn t() {
    assert!(purescript_corefn::from_str("{}").is_err());
    let str = include_str!("./Main/corefn.json");
    println!("{:?}", purescript_corefn::from_str(str).unwrap());
}
