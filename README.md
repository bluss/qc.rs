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

<div class='index'>

* [Const `config`](#const-config) - Default config value
* [Struct `NonEmptyVec`](#struct-nonemptyvec) - Like ~[T] but never empty
* [Struct `QConfig`](#struct-qconfig)
* [Struct `Random`](#struct-random) - A wrapper type to reuse an existing Rand instance for the Arbitrary impl
* [Struct `SmallN`](#struct-smalln) - A small number >= 0.
* [Struct `Unicode`](#struct-unicode)
* [Trait `Arbitrary`](#trait-arbitrary) - The Arbitrary trait can generate a randomly chosen value (with restrictions)
* [Implementation ` for QConfig`](#implementation-for-qconfig)
* [Implementation ` of Arb for int`](#implementation-of-arb-for-int)
* [Implementation ` of Arb for ~[T] where <T: Clone + Arb>`](#implementation-of-arb-for-t-where-t-clone-arb)
* [Implementation ` of ::std::to_bytes::IterBytes for Random<T> where <T: ::std::to_bytes::IterBytes>`](#implementation-of-stdto_bytesiterbytes-for-randomt-where-t-stdto_bytesiterbytes) - Automatically derived.
* [Implementation ` of ::std::cmp::Eq for Random<T> where <T: ::std::cmp::Eq>`](#implementation-of-stdcmpeq-for-randomt-where-t-stdcmpeq) - Automatically derived.
* [Implementation ` of ::std::clone::Clone for Random<T> where <T: ::std::clone::Clone>`](#implementation-of-stdcloneclone-for-randomt-where-t-stdcloneclone) - Automatically derived.
* [Implementation ` of ::std::to_bytes::IterBytes for Unicode`](#implementation-of-stdto_bytesiterbytes-for-unicode) - Automatically derived.
* [Implementation ` of ::std::cmp::Eq for Unicode`](#implementation-of-stdcmpeq-for-unicode) - Automatically derived.
* [Implementation ` of ::std::clone::Clone for Unicode`](#implementation-of-stdcloneclone-for-unicode) - Automatically derived.
* [Implementation ` of ::std::cmp::Eq for NonEmptyVec<T> where <T: ::std::cmp::Eq>`](#implementation-of-stdcmpeq-for-nonemptyvect-where-t-stdcmpeq) - Automatically derived.
* [Implementation ` of ::std::clone::Clone for NonEmptyVec<T> where <T: ::std::clone::Clone>`](#implementation-of-stdcloneclone-for-nonemptyvect-where-t-stdcloneclone) - Automatically derived.
* [Implementation ` of ::std::cmp::Eq for SmallN`](#implementation-of-stdcmpeq-for-smalln) - Automatically derived.
* [Implementation ` of ::std::clone::Clone for SmallN`](#implementation-of-stdcloneclone-for-smalln) - Automatically derived.
* [Implementation ` of Arbitrary for i8`](#implementation-of-arbitrary-for-i8)
* [Implementation ` of Arbitrary for int`](#implementation-of-arbitrary-for-int)
* [Implementation ` of Arbitrary for uint`](#implementation-of-arbitrary-for-uint)
* [Implementation ` of Arbitrary for float`](#implementation-of-arbitrary-for-float)
* [Implementation ` of Arbitrary for bool`](#implementation-of-arbitrary-for-bool)
* [Implementation ` of Arbitrary for char`](#implementation-of-arbitrary-for-char)
* [Implementation ` of Arbitrary for ()`](#implementation-of-arbitrary-for)
* [Implementation ` of Arbitrary for (A, B) where <A: Clone + Arbitrary, B: Clone + Arbitrary>`](#implementation-of-arbitrary-for-a-b-where-a-clone-arbitrary-b-clone-arbitrary)
* [Implementation ` of Arbitrary for (A, B, C) where <A: Clone + Arbitrary, B: Clone + Arbitrary, C: Clone + Arbitrary>`](#implementation-of-arbitrary-for-a-b-c-where-a-clone-arbitrary-b-clone-arbitrary-c-clone-arbitrary)
* [Implementation ` of Arbitrary for (A, B, C, D) where <A: Clone + Arbitrary, B: Clone + Arbitrary, C: Clone + Arbitrary, D: Clone +
 Arbitrary>`](#implementation-of-arbitrary-for-a-b-c-d-where-a-clone-arbitrary-b-clone-arbitrary-c-clone-arbitrary-d-clone-
-arbitrary)
* [Implementation ` of Arbitrary for (A, B, C, D, E) where <A: Clone + Arbitrary, B: Clone + Arbitrary, C: Clone + Arbitrary, D: Clone +
 Arbitrary, E: Clone + Arbitrary>`](#implementation-of-arbitrary-for-a-b-c-d-e-where-a-clone-arbitrary-b-clone-arbitrary-c-clone-arbitrary-d-clone-
-arbitrary-e-clone-arbitrary)
* [Implementation ` of Arbitrary for (A, B, C, D, E, F) where <A: Clone + Arbitrary, B: Clone + Arbitrary, C: Clone + Arbitrary, D: Clone +
 Arbitrary, E: Clone + Arbitrary, F: Clone + Arbitrary>`](#implementation-of-arbitrary-for-a-b-c-d-e-f-where-a-clone-arbitrary-b-clone-arbitrary-c-clone-arbitrary-d-clone-
-arbitrary-e-clone-arbitrary-f-clone-arbitrary)
* [Implementation ` of Arbitrary for Random<T> where <T: Rand>`](#implementation-of-arbitrary-for-randomt-where-t-rand)
* [Implementation ` of Arbitrary for ~T where <T: Arbitrary>`](#implementation-of-arbitrary-for-t-where-t-arbitrary)
* [Implementation ` of Arbitrary for u8`](#implementation-of-arbitrary-for-u8)
* [Implementation ` of Arbitrary for SmallN`](#implementation-of-arbitrary-for-smalln)
* [Implementation ` of Arbitrary for ~str`](#implementation-of-arbitrary-for-str)
* [Implementation ` of Arbitrary for Unicode`](#implementation-of-arbitrary-for-unicode)
* [Implementation ` of Arbitrary for ~[T] where <T: Clone + Arbitrary>`](#implementation-of-arbitrary-for-t-where-t-clone-arbitrary)
* [Implementation ` of Arbitrary for Option<T> where <T: Arbitrary>`](#implementation-of-arbitrary-for-optiont-where-t-arbitrary)
* [Implementation ` of Arbitrary for Result<T, U> where <T: Arbitrary, U: Arbitrary>`](#implementation-of-arbitrary-for-resultt-u-where-t-arbitrary-u-arbitrary)
* [Implementation ` of Arbitrary for NonEmptyVec<T> where <T: Clone + Arbitrary>`](#implementation-of-arbitrary-for-nonemptyvect-where-t-clone-arbitrary)
* [Implementation ` of Arbitrary for HashMap<K, V> where <K: Arbitrary + Eq + Hash, V: Arbitrary>`](#implementation-of-arbitrary-for-hashmapk-v-where-k-arbitrary-eq-hash-v-arbitrary)
* [Implementation ` of ::std::clone::Clone for UserType<T> where <T: ::std::clone::Clone>`](#implementation-of-stdcloneclone-for-usertypet-where-t-stdcloneclone) - Automatically derived.
* [Implementation ` of Arbitrary for UserType<T> where <T: Clone + Arbitrary>`](#implementation-of-arbitrary-for-usertypet-where-t-clone-arbitrary)
* [Implementation ` of ::std::clone::Clone for UserTree<T> where <T: ::std::clone::Clone>`](#implementation-of-stdcloneclone-for-usertreet-where-t-stdcloneclone) - Automatically derived.
* [Implementation ` of Arbitrary for UserTree<T> where <T: Clone + Arbitrary>`](#implementation-of-arbitrary-for-usertreet-where-t-clone-arbitrary)
* [Implementation ` of ::std::rand::Rand for Test_Foo`](#implementation-of-stdrandrand-for-test_foo) - Automatically derived.
* [Implementation ` of ::std::clone::Clone for Test_Foo`](#implementation-of-stdcloneclone-for-test_foo) - Automatically derived.
* [Function `arbitrary`](#function-arbitrary) - Create an arbitrary value of type T
* [Function `quick_check`](#function-quick_check) - Repeatedly test `property` with values of type `A` chosen using `Arbitrary`.
* [Function `quick_check_occurs`](#function-quick_check_occurs)
* [Function `quick_shrink`](#function-quick_shrink)

</div>

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

## Implementation of `Arb` for `int`

### Method `sh`

~~~ {.rust}
fn sh(&self, f: &fn(int) -> bool) -> bool
~~~

## Implementation of `Arb` for `~[T]` where `<T: Clone + Arb>`

### Method `sh`

~~~ {.rust}
fn sh(&self, f: &fn(~[T]) -> bool) -> bool
~~~

## Implementation of `::std::to_bytes::IterBytes` for `Random<T>` where `<T: ::std::to_bytes::IterBytes>`

Automatically derived.

### Method `iter_bytes`

~~~ {.rust}
fn iter_bytes(&self, __arg_0: ::bool, __arg_1: ::std::to_bytes::Cb) -> ::bool
~~~

## Implementation of `::std::cmp::Eq` for `Random<T>` where `<T: ::std::cmp::Eq>`

Automatically derived.

### Method `eq`

~~~ {.rust}
fn eq(&self, __arg_0: &Random<T>) -> ::bool
~~~

### Method `ne`

~~~ {.rust}
fn ne(&self, __arg_0: &Random<T>) -> ::bool
~~~

## Implementation of `::std::clone::Clone` for `Random<T>` where `<T: ::std::clone::Clone>`

Automatically derived.

### Method `clone`

~~~ {.rust}
fn clone(&self) -> Random<T>
~~~

## Implementation of `::std::to_bytes::IterBytes` for `Unicode`

Automatically derived.

### Method `iter_bytes`

~~~ {.rust}
fn iter_bytes(&self, __arg_0: ::bool, __arg_1: ::std::to_bytes::Cb) -> ::bool
~~~

## Implementation of `::std::cmp::Eq` for `Unicode`

Automatically derived.

### Method `eq`

~~~ {.rust}
fn eq(&self, __arg_0: &Unicode) -> ::bool
~~~

### Method `ne`

~~~ {.rust}
fn ne(&self, __arg_0: &Unicode) -> ::bool
~~~

## Implementation of `::std::clone::Clone` for `Unicode`

Automatically derived.

### Method `clone`

~~~ {.rust}
fn clone(&self) -> Unicode
~~~

## Implementation of `::std::cmp::Eq` for `NonEmptyVec<T>` where `<T: ::std::cmp::Eq>`

Automatically derived.

### Method `eq`

~~~ {.rust}
fn eq(&self, __arg_0: &NonEmptyVec<T>) -> ::bool
~~~

### Method `ne`

~~~ {.rust}
fn ne(&self, __arg_0: &NonEmptyVec<T>) -> ::bool
~~~

## Implementation of `::std::clone::Clone` for `NonEmptyVec<T>` where `<T: ::std::clone::Clone>`

Automatically derived.

### Method `clone`

~~~ {.rust}
fn clone(&self) -> NonEmptyVec<T>
~~~

## Implementation of `::std::cmp::Eq` for `SmallN`

Automatically derived.

### Method `eq`

~~~ {.rust}
fn eq(&self, __arg_0: &SmallN) -> ::bool
~~~

### Method `ne`

~~~ {.rust}
fn ne(&self, __arg_0: &SmallN) -> ::bool
~~~

## Implementation of `::std::clone::Clone` for `SmallN`

Automatically derived.

### Method `clone`

~~~ {.rust}
fn clone(&self) -> SmallN
~~~

## Implementation of `Arbitrary` for `i8`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(_: uint) -> i8
~~~

## Implementation of `Arbitrary` for `int`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(_: uint) -> int
~~~

## Implementation of `Arbitrary` for `uint`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(_: uint) -> uint
~~~

## Implementation of `Arbitrary` for `float`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(_: uint) -> float
~~~

## Implementation of `Arbitrary` for `bool`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(_: uint) -> bool
~~~

## Implementation of `Arbitrary` for `char`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(_: uint) -> char
~~~

## Implementation of `Arbitrary` for `()`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(_: uint)
~~~

## Implementation of `Arbitrary` for `(A, B)` where `<A: Clone + Arbitrary, B: Clone + Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> (A, B)
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[(A, B)]
~~~

## Implementation of `Arbitrary` for `(A, B, C)` where `<A: Clone + Arbitrary, B: Clone + Arbitrary, C: Clone + Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> (A, B, C)
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[(A, B, C)]
~~~

## Implementation of `Arbitrary` for `(A, B, C, D)` where `<A: Clone + Arbitrary, B: Clone + Arbitrary, C: Clone + Arbitrary, D: Clone +
 Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> (A, B, C, D)
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[(A, B, C, D)]
~~~

## Implementation of `Arbitrary` for `(A, B, C, D, E)` where `<A: Clone + Arbitrary, B: Clone + Arbitrary, C: Clone + Arbitrary, D: Clone +
 Arbitrary, E: Clone + Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> (A, B, C, D, E)
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[(A, B, C, D, E)]
~~~

## Implementation of `Arbitrary` for `(A, B, C, D, E, F)` where `<A: Clone + Arbitrary, B: Clone + Arbitrary, C: Clone + Arbitrary, D: Clone +
 Arbitrary, E: Clone + Arbitrary, F: Clone + Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> (A, B, C, D, E, F)
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[(A, B, C, D, E, F)]
~~~

## Implementation of `Arbitrary` for `Random<T>` where `<T: Rand>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(_: uint) -> Random<T>
~~~

## Implementation of `Arbitrary` for `~T` where `<T: Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> ~T
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[~T]
~~~

## Implementation of `Arbitrary` for `u8`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(_: uint) -> u8
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[u8]
~~~

## Implementation of `Arbitrary` for `SmallN`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> SmallN
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[SmallN]
~~~

## Implementation of `Arbitrary` for `~str`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> ~str
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[~str]
~~~

## Implementation of `Arbitrary` for `Unicode`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> Unicode
~~~

## Implementation of `Arbitrary` for `~[T]` where `<T: Clone + Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> ~[T]
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[~[T]]
~~~

## Implementation of `Arbitrary` for `Option<T>` where `<T: Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> Option<T>
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[Option<T>]
~~~

## Implementation of `Arbitrary` for `Result<T, U>` where `<T: Arbitrary, U: Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> Result<T, U>
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[Result<T, U>]
~~~

## Implementation of `Arbitrary` for `NonEmptyVec<T>` where `<T: Clone + Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> NonEmptyVec<T>
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[NonEmptyVec<T>]
~~~

## Implementation of `Arbitrary` for `HashMap<K, V>` where `<K: Arbitrary + Eq + Hash, V: Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> HashMap<K, V>
~~~

## Implementation of `::std::clone::Clone` for `UserType<T>` where `<T: ::std::clone::Clone>`

Automatically derived.

### Method `clone`

~~~ {.rust}
fn clone(&self) -> UserType<T>
~~~

## Implementation of `Arbitrary` for `UserType<T>` where `<T: Clone + Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> UserType<T>
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[UserType<T>]
~~~

## Implementation of `::std::clone::Clone` for `UserTree<T>` where `<T: ::std::clone::Clone>`

Automatically derived.

### Method `clone`

~~~ {.rust}
fn clone(&self) -> UserTree<T>
~~~

## Implementation of `Arbitrary` for `UserTree<T>` where `<T: Clone + Arbitrary>`

### Method `arbitrary`

~~~ {.rust}
fn arbitrary(sz: uint) -> UserTree<T>
~~~

### Method `shrink`

~~~ {.rust}
fn shrink(&self) -> ~[UserTree<T>]
~~~

## Implementation of `::std::rand::Rand` for `Test_Foo`

Automatically derived.

### Method `rand`

~~~ {.rust}
fn rand<R: ::std::rand::Rng>(__arg_0: &mut R) -> Test_Foo
~~~

## Implementation of `::std::clone::Clone` for `Test_Foo`

Automatically derived.

### Method `clone`

~~~ {.rust}
fn clone(&self) -> Test_Foo
~~~

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

