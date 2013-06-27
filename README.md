qc.rs -- QuickCheck for Rust

Use `quick_check` to check that a specified property holds
for values of `trait Arbitrary + Shrink`.

Example::

    extern mod qc;

    fn main() {
        qc::quick_check("sort", qc::config.verbose(true).trials(500),
            |mut v: ~[u8]| { sort(&mut v); is_sorted(v) });
    }

Issues:

* Clean up Lazy and Shrink, implement Arbitrary and Shrink further

---

Copyright License for qc.rs is identical with the Rust project:

    Licensed under the Apache License, Version 2.0
    <LICENSE-APACHE or
    http://www.apache.org/licenses/LICENSE-2.0> or the MIT
    license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
    at your option. All files in the project carrying such
    notice may not be copied, modified, or distributed except
    according to those terms.
