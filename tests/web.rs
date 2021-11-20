//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use wasm_bindgen_test::*;
use routing_wasm::router::{http::Method, radix_tree::Router};

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn test_insert () {
    let mut router = Router::new();
    router.insert(Method::GET, "/ciao/bla", 0);
    router.insert(Method::GET, "/ciao", 1);
    router.insert(Method::GET, "/ciao/bl", 2);
    router.insert(Method::GET, "/c/fratm", 3);
    router.insert(Method::GET, "/bell/fratm", 4);
    router.insert(Method::GET, "/bellaaa/fratm", 5);

    assert_eq!(router.lookup(Method::GET, "/c/fratm").unwrap(), 3);
    assert_eq!(router.lookup(Method::GET, "/ciao/bl").unwrap(), 2);
    assert_eq!(router.lookup(Method::GET, "/ciao/bl").unwrap(), 2);
    assert_eq!(router.lookup(Method::GET, "/ciao").unwrap(), 1);
    assert_eq!(router.lookup(Method::GET, "/ciao").unwrap(), 1);
    assert_eq!(router.lookup(Method::GET, "/ciao/bla").unwrap(), 0);
    assert_eq!(router.lookup(Method::GET, "/ciao/bla").unwrap(), 0);

    assert_eq!(router.lookup(Method::GET, "/bellaa/fratm").is_err(), true);
    assert_eq!(router.lookup(Method::GET, "/bedsadsllaa/fratm").is_err(), true);
    assert_eq!(router.lookup(Method::GET, "//fratm").is_err(), true);
    assert_eq!(router.lookup(Method::GET, "//sdiofjdsifjsdi").is_err(), true);

    assert_eq!(router.lookup(Method::GET, "/ciao/blo").is_err(), true);
}

/*
let route = new RouterWrapper();
route.insert("Get", '/ciao/bla', () => {
    console.log('bla');
});

route.insert("Get", '/ciao', () => {
    console.log('ciao');
});

route.insert("Get", '/ciao/bl', () => {
    console.log("bl");
});

route.insert("Get", '/c/fratm', () => {
    console.log("fratm");
});
route.insert("Get", '/bell/fratm', () => {
    console.log("bell");
});
route.insert("Get", '/bellaaa/fratm', () => {
    console.log("bell");
});


route.lookup('Get', '/c/fratm');
route.lookup('Get', '/ciao/bl');
route.lookup('Get', '/ciao');
route.lookup('Get', '/ciao/bla');
// route.lookup('Get', '//fsASrar');
route.lookup('Get', '/bellaa/fratm'); */
