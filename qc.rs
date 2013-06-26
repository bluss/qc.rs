/* vim: sts=4 sw=4 et
 */

/*!

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

*/

use std::rand::{Rand, Rng, RngUtil};
use std::hashmap::HashMap;

pub struct QConfig {
    trials: uint,
    size: uint,
    verbose: bool,
    grow: bool,
}

/** Default config value */
pub static config: QConfig = QConfig{ trials: 25, size: 8, verbose: false, grow: true };

impl QConfig {
    /// Set size factor (default 8)
    pub fn size(self, x: uint) -> QConfig {
        QConfig{size: x, ..self}
    }
    /// Set n trials (default 25)
    pub fn trials(self, x: uint) -> QConfig {
        QConfig{trials: x, ..self}
    }
    /// Set if size factor should gradually increase (default true)
    pub fn grow(self, x: bool) -> QConfig {
        QConfig{grow: x, ..self}
    }
    /// Set verbose (default false)
    pub fn verbose(self, x: bool) -> QConfig {
        QConfig{verbose: x, ..self}
    }
}

/**
 
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
 */
pub fn quick_check<A: Clone + Arbitrary>(name: &str, cfg: QConfig, prop: &fn(A) -> bool) {
    for std::uint::range(0, cfg.trials) |i| {
        let value = arbitrary::<A>(cfg.size + if cfg.grow { i / 8 } else { 0 });
        if cfg.verbose {
            //println(fmt!("qc %s:  %u. trying value '%?'", name, 1+i, &value));
        }
        let v_copy = value.clone();
        if !prop(value) {
            if cfg.verbose {
                println(fmt!("qc %s: first falsification with value '%?'", name, &v_copy));
            }
            let shrink = quick_shrink(cfg, v_copy, prop);
            fail!(fmt!("qc %s: falsified (%u trials) with value '%?'", name, 1+i, shrink));
        }
    }
    if cfg.verbose {
        println(fmt!("qc %s: passed'", name));
    }
}

pub fn quick_shrink<A: Clone + Arbitrary>(cfg: QConfig, value: A, prop: &fn(A) -> bool) -> A {
    //assert!(!prop(value.clone()));
    let mut shrinks = value.shrink();
    loop {
        if shrinks.len() == 0 {
            break;
        }
        let elt = shrinks.pop();
        let elt_cpy = elt.clone();
        if !prop(elt) {
            if cfg.verbose { println(fmt!("Shrunk to: %?", &elt_cpy)); }
            return quick_shrink(cfg, elt_cpy, prop);
        }
    }
    if cfg.verbose {
        println(fmt!("Shrink finished: %?", &value));
    }
    value
}

pub fn quick_check_occurs<A: Arbitrary>(cfg: QConfig, name: &str, prop: &fn(A) -> bool) {
    let mut n = 0u;
    for std::uint::range(0, cfg.trials) |i| {
        n += 1;
        let value = arbitrary(cfg.size + if cfg.grow { i / 8 } else { 0 });
        if prop(value) {
            if cfg.verbose {
                println(fmt!("qc %s: occured (%u trials)", name, n));
            }
            break;
        }
    }
    if n >= cfg.trials {
        fail!(fmt!("qc %s: could not to reproduce", name));
    }
}

pub macro_rules! quick_check(
    ($qc_property:expr) => (
        quick_check!(config, $qc_property)
    );
    ($qc_config:expr, $qc_property:expr) => ({
        quick_check(
            fmt!("%s\n%s:%u", stringify!($qc_property), file!(), line!()),
            $qc_config,
            $qc_property);
    })
)

pub macro_rules! quick_check_occurs(
    ($qc_property:expr) => (
        quick_check_occurs!(config.trials(config.trials * 4), $qc_property)
    );
    ($qc_config:expr, $qc_property:expr) => ({
        quick_check_occurs($qc_config,
            fmt!("%s:%u", file!(), line!()), $qc_property);
    })
)

pub macro_rules! vmap(
    ($r:expr for $x:pat in $J:expr) => (
        std::vec::map_consume(($J), |$x| $r)
    );
)

pub macro_rules! concat(
    ($v:expr $(, $s:expr)* $(: $elt:expr)*) => ({
        let mut _m_retvec = $v;
        $(
            _m_retvec.push_all_move($s);
        )*
        $(
            _m_retvec.push($elt);
        )*
        _m_retvec
    })
)

trait Arb {
    fn sh(&self, &fn(Self) -> bool) -> bool;
}

impl Arb for int {
    fn sh(&self, f: &fn(int) -> bool) -> bool {
        if *self > 0 {
            f(*self-1)
        } else {
            true
        }
    }
}

impl<T: Clone + Arb> Arb for ~[T] {
    fn sh(&self, f: &fn(~[T]) -> bool) -> bool {
        for self.iter().enumerate().advance |(i, elt)| {
            let mut v1 = self.clone();
            for elt.sh() |s| {
                v1[i] = s.clone();
                if !f(v1.clone()) { return false }
            }

            v1.remove(i);
            if !f(v1) { return false }
        }
        true
    }
}

/* Arbitrary */

/**
 The Arbitrary trait can generate a randomly chosen value (with restrictions).
 You can pass a size factor to allow specifying test size (sizes of vectors and
 numbers).
 */
#[allow(default_methods)]
pub trait Arbitrary {
    /**
     arbitrary should return an arbitrary value of its type.
     The value should be randomly chosen and its size should be scaled by the size
     parameter.
     */
    fn arbitrary(uint) -> Self;

    /**
     shrink is used when trying to reduce a testcase to a minimal testcase.
     shrink should return a vec of all combinations of "simpler" values.
     Put the smallest shrink last.
     
     The default method is good for incompressible values.
     */
    fn shrink(&self) -> ~[Self] {
        ~[]
    }
}

/// Create an arbitrary value of type T
#[inline]
pub fn arbitrary<T: Arbitrary>(sz: uint) -> T {
    Arbitrary::arbitrary(sz)
}

/// A wrapper type to reuse an existing Rand instance for the Arbitrary impl
#[deriving(IterBytes, Eq, Clone)]
pub struct Random<T>(T);

#[deriving(IterBytes, Eq, Clone)]
pub struct Unicode(~str);

/// Like ~[T] but never empty
#[deriving(Eq, Clone)]
pub struct NonEmptyVec<T>(~[T]);

/// A small number >= 0.
#[deriving(Eq, Clone)]
pub struct SmallN(uint);

fn small_n(size: uint) -> uint {
    let f: std::rand::distributions::Exp1 = std::rand::random();
    let n = (*f) * (size as f64) as uint;
    n.min(&(16 * size))
}

fn gen_unicode_str<R: Rng>(rng: &mut R, len: uint) -> ~str {
    let text = ~"\
a b c 0 $ ⇌ [ˈʏpsilɔn] \\ \" ‚dsch‘ „füh“      ‡ € ⁿ ２ � 🈘
ἀπὸ состоится ทรงนับถือขันทีเป็นที่พึ่ง Hello world Καλημέρα κόσμε コンニチハ";
    let mut res = ~"";
    let mut words: ~[&str] = text.word_iter().collect();
    words.push_all([" ", " ", "\n"]);
    while res.len() < len {
        res += rng.choose(words);
    }
    res
}

fn gen_vec<T: Arbitrary>(sz: uint, n: uint) -> ~[T] {
    let mut v = ~[];
    for n.times {
        v.push(arbitrary(sz));
    }
    v
}


macro_rules! arb_rand( ($T:ty) => (
        impl Arbitrary for $T {
            fn arbitrary(_: uint) -> $T {
                std::rand::random()
            }
        }
    )
)

macro_rules! arb_tuple( ($($T:ident),+ -> $($S:expr),+) => (
        impl<$($T: Clone + Arbitrary),+> Arbitrary for ($($T),+) {
            fn arbitrary(sz: uint) -> ($($T),+) {
                ($(Arbitrary::arbitrary::<$T>(sz)),+)
            }
            fn shrink(&self) -> ~[($($T),+)] {
                match self {
                    &($(ref $T),+) => {
                        concat!(
                            $( vmap!($S for s in $T.shrink()) ),+
                        )
                    }
                }
            }
        }
    )
)

arb_rand!(i8)
//arb_rand!(u8)
arb_rand!(int)
arb_rand!(uint)
arb_rand!(float)
arb_rand!(bool)
arb_rand!(char)
arb_rand!(())

arb_tuple!(A, B ->
    (s, B.clone()),
    (A.clone(), s))
arb_tuple!(A, B, C ->
    (s, B.clone(), C.clone()),
    (A.clone(), s, C.clone()),
    (A.clone(), B.clone(), s))
arb_tuple!(A, B, C, D ->
    (s, B.clone(), C.clone(), D.clone()),
    (A.clone(), s, C.clone(), D.clone()),
    (A.clone(), B.clone(), s, D.clone()),
    (A.clone(), B.clone(), C.clone(), s))
arb_tuple!(A, B, C, D, E ->
    (s, B.clone(), C.clone(), D.clone(), E.clone()),
    (A.clone(), s, C.clone(), D.clone(), E.clone()),
    (A.clone(), B.clone(), s, D.clone(), E.clone()),
    (A.clone(), B.clone(), C.clone(), s, E.clone()),
    (A.clone(), B.clone(), C.clone(), D.clone(), s))
arb_tuple!(A, B, C, D, E, F ->
    (s, B.clone(), C.clone(), D.clone(), E.clone(), F.clone()),
    (A.clone(), s, C.clone(), D.clone(), E.clone(), F.clone()),
    (A.clone(), B.clone(), s, D.clone(), E.clone(), F.clone()),
    (A.clone(), B.clone(), C.clone(), s, E.clone(), F.clone()),
    (A.clone(), B.clone(), C.clone(), D.clone(), s, F.clone()),
    (A.clone(), B.clone(), C.clone(), D.clone(), E.clone(), s))

impl<T: Rand> Arbitrary for Random<T> {
    fn arbitrary(_: uint) -> Random<T> {
        Random(std::rand::random())
    }
}

impl<T: Arbitrary> Arbitrary for ~T {
    #[inline]
    fn arbitrary(sz: uint) -> ~T {
        ~arbitrary(sz)
    }
    #[inline]
    fn shrink(&self) -> ~[~T] {
        vmap!(~x for x in (**self).shrink())
    }
}

impl Arbitrary for u8 {
    fn arbitrary(_: uint) -> u8 {
        std::rand::random()
    }
    fn shrink(&self) -> ~[u8] {
        match *self {
            0 => ~[],
            1 => ~[0],
            2 => ~[1, 0],
            n @ 3 .. 10 => ~[n-1, n-2, n-3],
            n => ~[n-1, n-2, n-5, n - n/8, n - n/4, n/2]
        }
    }
}

impl Arbitrary for SmallN {
    fn arbitrary(sz: uint) -> SmallN {
        SmallN(small_n(sz))
    }
    fn shrink(&self) -> ~[SmallN] {
        vmap!(SmallN(n) for n in match **self {
            0 => ~[],
            1 => ~[0],
            2 => ~[1, 0],
            n @ 3 .. 10 => ~[n-1, n-2, n-3],
            n => ~[n-1, n-2, n-5, n - n/8, n - n/4, n/2]
        })
    }
}

impl Arbitrary for ~str {
    fn arbitrary(sz: uint) -> ~str {
        let rng = &mut **std::rand::task_rng();
        let n = small_n(sz);
        rng.gen_str(n)
    }
    fn shrink(&self) -> ~[~str] {
        vmap!(std::str::from_chars(s) for s in self.iter().collect::<~[char]>().shrink())
    }
}

impl Arbitrary for Unicode {
    fn arbitrary(sz: uint) -> Unicode {
        let rng = &mut **std::rand::task_rng();
        let n = small_n(sz);
        Unicode(gen_unicode_str(rng, n))
    }
}

impl<T: Clone + Arbitrary> Arbitrary for ~[T] {
    fn arbitrary(sz: uint) -> ~[T] {
        let n = small_n(sz);
        gen_vec(sz, n)
    }
    fn shrink(&self) -> ~[~[T]] {
        let mut res = ~[];
        for self.iter().enumerate().advance |(i, elt)| {
            res.push_all_move(
                vmap!({
                    let mut v2 = self.clone();
                    v2[i] = selt;
                    v2
                } for selt in elt.shrink())
            );

            let mut v1 = self.clone();
            v1.remove(i);
            res.push(v1);
        }
        /* Split vec in half */
        if self.len() > 2 {
            res.push(
                self.iter().transform(|x| x.clone()).skip(self.len()/2).collect()
            );
            res.push({
                let mut v1 = self.clone();
                v1.truncate(self.len()/2);
                v1
            })
        }
        res
    }
}

impl<T: Arbitrary> Arbitrary for Option<T> {
    fn arbitrary(sz: uint) -> Option<T> {
        if std::rand::random() {
            Some(arbitrary(sz))
        } else {
            None
        }
    }

    fn shrink(&self) -> ~[Option<T>] {
        match *self {
            Some(ref x) => {
                concat!(
                    vmap!(Some(s) for s in x.shrink())
                    : None
                )
            },
            None => ~[]
        }
    }
}

impl<T: Arbitrary, U: Arbitrary> Arbitrary for Result<T, U> {
    fn arbitrary(sz: uint) -> Result<T, U> {
        if std::rand::random() {
            Ok(arbitrary(sz))
        } else {
            Err(arbitrary(sz))
        }
    }
    fn shrink(&self) -> ~[Result<T, U>] {
        match *self {
            Ok(ref x) => {
                vmap!(Ok(s) for s in x.shrink())
            },
            Err(ref x) => {
                vmap!(Err(s) for s in x.shrink())
            }
        }
    }
}

impl<T: Clone + Arbitrary> Arbitrary for NonEmptyVec<T> {
    fn arbitrary(sz: uint) -> NonEmptyVec<T> {
        let n = 1 + small_n(sz);
        NonEmptyVec(gen_vec(sz, n))
    }

    fn shrink(&self) -> ~[NonEmptyVec<T>] {
        let shrinks = (**self).shrink();
        shrinks.iter()
            .filter_map(|&v| if v.len() > 0 { Some(NonEmptyVec(v)) } else { None })
            .collect()
    }
}

impl<K: Arbitrary + Eq + Hash, V: Arbitrary> Arbitrary for HashMap<K, V> {
    fn arbitrary(sz: uint) -> HashMap<K, V> {
        let n: uint = small_n(sz);
        let mut v = HashMap::new();
        for n.times {
            v.insert(arbitrary(sz), arbitrary(sz));
        }
        v
    }
}

/// Example of how to implement Arbitrary
#[deriving(Clone)]
enum UserType<T> {
    Nothing,
    Blob(int, ~str),
    Blub(~[T]),
}

impl<T: Clone + Arbitrary> Arbitrary for UserType<T> {
    fn arbitrary(sz: uint) -> UserType<T> {
        let x: u8 = std::rand::random();
        match x % 3 {
            0 => Nothing,
            1 => Blob(arbitrary(sz), arbitrary(sz)),
            _ => Blub(arbitrary(sz)),
        }
    }

    /* Simply re-use the shrink code for tuples,
     * and make sure we use the shrinking case ``Nothing`` */
    fn shrink(&self) -> ~[UserType<T>] {
        match *self {
            Nothing => ~[],
            Blob(x, ref y) => concat!(
                    vmap!(Blob(a, b) for (a, b) in (x, y.clone()).shrink())
                    : Nothing
                ),
            Blub(ref v) => concat!(
                    vmap!(Blub(x) for x in v.shrink())
                    : Nothing
                )
        }
    }
}

/// Example of how to implement Arbitrary
#[deriving(Clone)]
enum UserTree<T> {
    Nil,
    Node(T, ~UserTree<T>, ~UserTree<T>)
}

impl<T: Clone + Arbitrary> Arbitrary for UserTree<T> {
    fn arbitrary(sz: uint) -> UserTree<T> {
        let rint: u8 = std::rand::random();
        if sz == 0 || rint % 4 == 0 {
            Nil
        } else {
            Node(arbitrary(sz), ~arbitrary(sz/2), ~arbitrary(sz/2))
        }
    }

    /* Simply re-use the shrink code for tuples,
     * and make sure we use the shrinking case ``Nil`` */
    fn shrink(&self) -> ~[UserTree<T>] {
        match self.clone() {
            Node(x, l, r) =>
                concat!(
                    vmap!(Node(a, b, c) for (a, b, c) in (x, l, r).shrink())
                    : Nil
                ),
            Nil => ~[],
        }
    }
}

#[test]
fn test_qc_basic() {
    let mut n = 0;
    quick_check!(|_: int| { n += 1; true} );
    assert_eq!(n, config.trials);

    let mut m = 0;
    quick_check_occurs!(|_: int| { m += 1; m == 20 });
    assert_eq!(m, 20);
}

#[test]
#[should_fail]
fn test_qc_fail() {
    quick_check!(|_: ()| false);
}

#[test]
#[should_fail]
fn test_qc_occurs_fail() {
    quick_check_occurs!(|s: ~str| s.len() == -1);
}

#[test]
fn test_qc_func() {
    let mut n = 0;
    quick_check("7 trials", config.trials(7), |_: int| { n += 1; true} );
    assert_eq!(n, 7);
}

#[test]
fn test_qc_config() {
    quick_check!(config.trials(0), |_: ()| false );
    quick_check!(config.trials(1), |_: ()| true );

    let mut n = 0;
    quick_check!(config.trials(7), |_: ()| { n += 1; true} );
    assert_eq!(n, 7);

    quick_check_occurs!(config.size(1000), |n: SmallN| *n > 1000);
}


#[test]
fn test_qc_smalln() {
    quick_check_occurs!(|n: SmallN| *n == 0);
    quick_check_occurs!(|n: SmallN| *n == 1);
    quick_check_occurs!(|n: SmallN| *n > 10);
}

#[test]
fn test_qc_shrink() {
    /* Test minimal shrinks with false props */
    let v = SmallN(10);
    let shrink = quick_shrink(config, v, |_| false);
    assert_eq!(*shrink, 0);

    let s = ~[0, 1, 1, 2, 1, 0, 1, 0, 1];
    let shrink = quick_shrink(config, s, |_| false);
    assert_eq!(shrink, ~[]);

    let s = NonEmptyVec(~[0, 1, 1, 2, 1, 0, 1, 0, 1]);
    let shrink = quick_shrink(config, s, |_| false);
    assert_eq!(shrink.len(), 1);

    /* Make sure we can shrink nested containers */
    let v = Some(~[Some(~"hi"), None, Some(~""), Some(~"long text from me")]);
    let shrink = quick_shrink(config, v, |_| false);
    assert_eq!(shrink, None);

    let s = ~[Some(~"hi"), None, Some(~"more"), None];
    assert_eq!(quick_shrink(config, s, |v| !v.iter().filter_map(|&x| x).any_(|s| s.contains_char('e'))),
        ~[Some(~"e")]);

    let s = ~"boots are made for walking";
    assert_eq!(quick_shrink(config, s, |v| v.iter().count(|x| x == 'a') <= 1),
        ~"aa");

    let s = ~[0, 1, 1, 2, 1, 0, 1, 0, 1];
    let sum = |v: ~[int]| v.iter().fold(0, |a, &b| a + b);
    let shrink = quick_shrink(config, s, |v| sum(v) < 3);
    assert_eq!(sum(shrink), 3);

    let s = (~"more meat", ~"beef");
    let shrink = quick_shrink(config, s, |(a, b)| !(a.contains_char('e') && b.contains_char('e')));
    assert_eq!(shrink, (~"e", ~"e"));

    let s = (SmallN(1), SmallN(10), SmallN(3));
    let shrink = quick_shrink(config, s, |(a, b, c)| *a + *b + *c == 1);
    assert_eq!(shrink, (SmallN(1), SmallN(1), SmallN(0)));

}

#[test]
#[should_fail]
fn test_qc_tree() {
    quick_check!(config.size(5),
        |u: UserTree<u8>| match u {
            Node(x, ~Node(y, _, _), ~Nil) => (x ^ y) & 0x13 == 0,
            _ => true,
        });
}

#[test]
#[should_fail]
fn test_qc_shrink_fail() {
    quick_check!(config.verbose(true).trials(100),
        |(a, b): (~str, ~str)| !(a.contains_char('e') || b.contains_char('e')));
}


#[deriving(Rand, Clone)]
struct Test_Foo { x: float, u: int }

#[test]
fn test_qc_random() {
    quick_check!(|_: Random<Test_Foo>| true);
}

#[test]
fn test_qc_containers() {
    quick_check!(|v: NonEmptyVec<u8>| v.len() > 0);

    quick_check_occurs!(|v: ~[u8]| v.len() == 0);
    quick_check_occurs!(|v: ~[u8]| v.len() == 1);
    quick_check_occurs!(|v: ~[u8]| v.len() > 10);
    quick_check_occurs!(|v: HashMap<u8, u8>| v.len() == 0);
    quick_check_occurs!(|v: HashMap<u8, u8>| v.len() == 1);

    quick_check!(|s: ~str| s.is_ascii());

    quick_check_occurs!(|s: Unicode| s.len() > 0 && s.is_ascii());
    quick_check_occurs!(|s: Unicode| !s.is_ascii());
}

#[test]
#[should_fail]
fn test_invalid_utf8() {
    /* Demonstrate is_utf8 accepts some invalid utf-8 */
    quick_check!(config.verbose(true).grow(false).trials(5000), |v: ~[u8]| {
        if std::str::is_utf8(v) {
            v.iter().all(|&c| c != 192 && c != 193 && (c < 245))
        } else { true }
    });
}

#[test]
fn test_str() {
    quick_check!(|s: Unicode| std::str::is_utf8(s.as_bytes()));
    quick_check!(|s: ~[char]| {
        let ss = std::str::from_chars(s);
        std::str::is_utf8(ss.as_bytes())
    });

    //assert!(!std::str::is_utf8(&[69, 70, 119, 213, 182, 73, 244, 145, 164, 184]));

    quick_check!(|s: ~str| {
        let bs = s.as_bytes_with_null();
        bs.len() > 0 && bs[bs.len()-1] == 0
    });
    quick_check!(|s: Unicode| { std::str::from_bytes(s.as_bytes()) == *s });

}

#[test]
fn test_random_stuff() {
    quick_check!(|v: ~[int]| { (v.head_opt().is_some()) == (v.len() > 0) });
    quick_check!(|v: ~[~str]| v.head_opt() == v.iter().next());
    quick_check!(|v: NonEmptyVec<float>| v.iter().max().is_some());

    quick_check!(|(v, n): (~[i8], SmallN)| {
        v.iter().take_(*n).len_() == v.len().min(&*n)
    });

    quick_check!(|v: ~[Option<i8>]| { v == v.iter().transform(|&elt| elt).collect() });

    quick_check!(|v: ~[~str]| { v == v.clone() });

    /* Check that chain is correct length */
    quick_check!(|(x,y): (~[u8], ~[u8])| {
        x.len() + y.len() == x.iter().chain_(y.iter()).len_()
    });
    /* Check that chain has the right elements */
    quick_check!(|(x,y): (~[u8], ~[u8])| {
        x.iter().chain_(y.iter()).skip(x.len()).zip(y.iter()).all(|(a, b)| a == b)
    });

    /* Check that enumerate is indexing correctly */
    quick_check!(|x: ~[int]| {
        x.iter().enumerate().all(|(i, &elt)| x[i] == elt)
    });

    quick_check!(|(x,y): (~[u8], ~[u8])| {
        x.iter().zip(y.iter()).len_() == x.len().min(&y.len())
    });

/*
    quick_check!(|(x,y): (~[u8], ~[u8])| {
        let v = [&x, &y];
        let xs = v.iter().flat_map_(|a| a.iter());
        let ys: ~[u8] = xs.transform(|&x: &u8| x).collect();
        ys.iter().zip(x.iter().chain_(y.iter())).all(|(a, b)| *a == *b) &&
            ys.len() == x.len() + y.len()
    });
    */
}
