// Copyright 2017 The UNIC Project Developers.
//
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate unic_ucd_block;
use unic_ucd_block::BlockIter;

#[test]
fn test_all() {
    assert_eq!(BlockIter::new().count(), 277);

    let basic_latin = BlockIter::new().nth(0).unwrap();
    assert_eq!(basic_latin.range.low, '\u{0}');
    assert_eq!(basic_latin.range.high, '\u{7f}');
    assert_eq!(basic_latin.name, "Basic Latin");
}
