// vim: sts=4 sw=4 et

use lazy::Lazy;
use super::std;

/**
 The Shrink trait is used when trying to reduce a testcase to a minimal testcase.
 Shrink should generate "simpler" values, the simplest first.
 */
#[allow(default_methods)]
pub trait Shrink {
    fn shrink(&self) -> Lazy<Self> {
        Lazy::new()
    }
}

impl Shrink for () {}
impl Shrink for bool {}
impl Shrink for char {}
impl Shrink for float {}
impl Shrink for i8 {}
impl Shrink for int {}

impl<T: Owned + Shrink> Shrink for ~T {
    /* FIXME: This impl causes crashes? */
    fn shrink(&self) -> Lazy<~T> {
        do Lazy::create |L| {
            L.push_map((**self).shrink(), |u| ~u);
        }
    }
}

fn mpowers_of_two<T: Num + Ord>(n: T) -> ~[T] {
    /* generate ~[0, n/2, n - n/4, n - n/8, n - n/16, .., n - 1] */
    use std::num::One;
    let mut ret = ~[std::num::Zero::zero()];
    let two = One::one::<T>() + One::one();
    let mut div = One::one::<T>() + One::one();
    /* check for end or overflow */
    while div < n && div >= two{
        let next = n/div;
        ret.push(n - next);
        div = div * two;
    }
    ret
}

macro_rules! shrink_uint(
    ($x:expr) => (match $x {
            0 => ~[],
            1 => ~[0],
            2 => ~[0, 1],
            n @ 3 .. 8 => ~[n-3, n-2, n-1],
            n => mpowers_of_two(n),
    })
)

impl Shrink for u8 {
    fn shrink(&self) -> Lazy<u8> { Lazy::new_from(shrink_uint!(*self)) }
}

impl Shrink for uint {
    fn shrink(&self) -> Lazy<uint> { Lazy::new_from(shrink_uint!(*self)) }
}

macro_rules! shrink_tuple(
    ($($T:ident),+ -> $($U:ident),+ -> $($S:expr),+) => (
    impl<$($T: Owned + Clone + Shrink),+> Shrink for ($($T),+) {
        fn shrink(&self) -> Lazy<($($T),+)> {
            do Lazy::create |L| {
                match self {
                    &($(ref $T),+) => {
                        $(
                            let $U = $T.clone();
                        )+
                        $(
                            L.push_map($T.shrink(), |s| $S);
                        )+
                    }
                }
            }
        }
    }
    )
)

shrink_tuple!(
    A, B ->
    a, b -> 
    (s, b.clone()),
    (a.clone(), s))

shrink_tuple!(
    A, B, C ->
    a, b, c -> 
    (s, b.clone(), c.clone()),
    (a.clone(), s, c.clone()),
    (a.clone(), b.clone(), s))

shrink_tuple!(
    A, B, C, D
    ->
    a, b, c, d
    ->
    (s, b.clone(), c.clone(), d.clone()),
    (a.clone(), s, c.clone(), d.clone()),
    (a.clone(), b.clone(), s, d.clone()),
    (a.clone(), b.clone(), c.clone(), s))

impl<T: Owned + Clone + Shrink> Shrink for Option<T> {
    fn shrink(&self) -> Lazy<Option<T>> {
        do Lazy::create |L| {
            match *self {
                None => {},
                Some(ref x) => {
                    L.push(None);
                    L.push_map(x.shrink(), |y| Some(y));
                }
            }
        }
    }
}

impl Shrink for ~str {
    fn shrink(&self) -> Lazy<~str> {
        do Lazy::create |L| {
            if self.len() > 0 {
                let v = self.iter().collect::<~[char]>();
                L.push_map(v.shrink(), |v| std::str::from_chars(v));
            }
        }
    }
}

impl<T: Owned + Clone + Shrink> Shrink for ~[T] {
    fn shrink(&self) -> Lazy<~[T]> {
        let mut L = Lazy::new();
        if self.len() == 0 {
            return L;
        }

        L.push(~[]);

        do L.push_thunk(self.clone()) |v, L| {
            if v.len() > 2 {
                /* splitting a vec is awkward with Clone .. */
                L.push(
                    v.iter().transform(|x| x.clone()).skip(v.len()/2).collect()
                );
                L.push({
                    let mut v1 = v.clone();
                    v1.truncate(v.len()/2);
                    v1
                })
            }
            do L.push_thunk(v.clone()) |v, L| {
                /* remove one at a time */
                for std::uint::range(0, v.len()) |index| {
                    do L.push_thunk(v.clone()) |mut v, L| {
                        v.remove(index);
                        L.push(v);
                    }
                }

                /* shrink one at a time */
                for std::uint::range(0, v.len()) |index| {
                    do L.push_thunk(v.clone()) |v, L| {
                        let elt = &v[index];
                        for elt.shrink().advance |selt| {
                            let mut v1 = v.clone();
                            v1[index] = selt;
                            L.push(v1);
                        }
                    }
                }
            }
        }
        L
    }
}

