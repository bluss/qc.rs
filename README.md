% Crate qc.rs

qc.rs -- QuickCheck for Rust

Use `quick_check` to check that a specified property holds
for values of `trait Arbitrary`.

Example::

    extern mod qc;

    fn main() {
        qc::quick_check("sort", qc::config.verbose(true).trials(500),
            |mut v: ~[u8]| { sort(&mut v); is_sorted(v) });
    }

Issues:

* Figure out a way to generate shrinks lazily in trait Arbitrary, in a composable way.

---

Copyright x, 2013

Copyright License for qc.rs is identical with the Rust project:

'''
Licensed under the Apache License, Version 2.0
<LICENSE-APACHE or
http://www.apache.org/licenses/LICENSE-2.0> or the MIT
license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
at your option. All files in the project carrying such
notice may not be copied, modified, or distributed except
according to those terms.
'''

## Const `config`

~~~ {.rust}
QConfig
~~~

Default config value

## Struct `NonEmptyVec`

~~~ {.rust}
pub struct NonEmptyVec<T>(~[T]);
~~~

Like ~[T] but never empty

## Struct `QConfig`

~~~ {.rust}
pub struct QConfig {
    trials: uint,
    size: uint,
    verbose: bool,
    grow: bool,
}
~~~

## Struct `Random`

~~~ {.rust}
pub struct Random<T>(T);
~~~

A wrapper type to reuse an existing Rand instance for the Arbitrary impl

## Struct `SmallN`

~~~ {.rust}
pub struct SmallN(uint);
~~~

A small number >= 0.

## Struct `Unicode`

~~~ {.rust}
pub struct Unicode(~str);
~~~

## Trait `Arbitrary`

The Arbitrary trait can generate a randomly chosen value (with restrictions).
You can pass a size factor to allow specifying test size (sizes of vectors and
numbers).

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(uint) -> Self
~~~

arbitrary should return an arbitrary value of its type.
The value should be randomly chosen and its size should be scaled by the size
parameter.

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[Self]
~~~

shrink is used when trying to reduce a testcase to a minimal testcase.
shrink should return a vec of all combinations of "simpler" values.
Put the smallest shrink last.
     
The default method is good for incompressible values.

## Implementation for `QConfig`

### Method `size`

~~~ {.rust}
fn size(self, x: uint) -> QConfig
~~~

Set size factor (default 8)

### Method `trials`

~~~ {.rust}
fn trials(self, x: uint) -> QConfig
~~~

Set n trials (default 25)

### Method `grow`

~~~ {.rust}
fn grow(self, x: bool) -> QConfig
~~~

Set if size factor should gradually increase (default true)

### Method `verbose`

~~~ {.rust}
fn verbose(self, x: bool) -> QConfig
~~~

Set verbose (default false)


## Function `arbitrary`

~~~ {.rust}
fn arbitrary<T: Arbitrary>(sz: uint) -> T
~~~

Create an arbitrary value of type T

## Function `quick_check`

~~~ {.rust}
fn quick_check<A: Clone +
               Arbitrary>(name: &str, cfg: QConfig, prop: &fn(A) -> bool)
~~~

Repeatedly test `property` with values of type `A` chosen using `Arbitrary`.

If `property` holds true for all tested values, the quick_check test passes.

If a counterexample is found, quick_check will use `quick_shrink` to try to
find a minimal counterexample to `property`.

quick_check calls `fail!()` with an error message indicating `name` and the
repr of the counterexample.
 
Examples:
 
`quick_check!(|x: Type| property(x));`

`quick_check("name", config, |x: Type| property(x));`

`quick_check("str", config.trials(100), |s: ~str| s.is_ascii());`
 
NOTE: `A` must implement `Clone`.

## Function `quick_check_occurs`

~~~ {.rust}
fn quick_check_occurs<A: Arbitrary>(cfg: QConfig, name: &str,
                                    prop: &fn(A) -> bool)
~~~

## Function `quick_shrink`

~~~ {.rust}
fn quick_shrink<A: Clone +
                Arbitrary>(cfg: QConfig, value: A, prop: &fn(A) -> bool) -> A
~~~

