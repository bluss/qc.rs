// vim: sts=4 sw=4 et

use lazy::Lazy;
use super::std;

use std::cell::Cell;

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
impl<T> Shrink for @T {}
impl<T> Shrink for @mut T {}

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
    ($($T:ident),+ -> $($S:expr),+) => (
    impl<$($T: Send + Clone + Shrink),+> Shrink for ($($T),+) {
        fn shrink(&self) -> Lazy<($($T),+)> {
            do Lazy::create |L| {
                match self {
                    &($(ref $T),+) => {
                        $(
                            L.push_map_env($T.shrink(), self.clone(), |s, t| $S);
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
    (s, t.n1().clone()),
    (t.n0().clone(), s))

shrink_tuple!(
    A, B, C ->
    (s, t.n1().clone(), t.n2().clone()),
    (t.n0().clone(), s, t.n2().clone()),
    (t.n0().clone(), t.n1().clone(), s))

shrink_tuple!(
    A, B, C, D ->
    (s, t.n1().clone(), t.n2().clone(), t.n3().clone()),
    (t.n0().clone(), s, t.n2().clone(), t.n3().clone()),
    (t.n0().clone(), t.n1().clone(), s, t.n3().clone()),
    (t.n0().clone(), t.n1().clone(), t.n2().clone(), s))

shrink_tuple!(
    A, B, C, D, E ->
    (s, t.n1().clone(), t.n2().clone(), t.n3().clone(), t.n4().clone()),
    (t.n0().clone(), s, t.n2().clone(), t.n3().clone(), t.n4().clone()),
    (t.n0().clone(), t.n1().clone(), s, t.n3().clone(), t.n4().clone()),
    (t.n0().clone(), t.n1().clone(), t.n2().clone(), s, t.n4().clone()),
    (t.n0().clone(), t.n1().clone(), t.n2().clone(), t.n3().clone(), s))

shrink_tuple!(
    A, B, C, D, E, F ->
    (s, t.n1().clone(), t.n2().clone(), t.n3().clone(), t.n4().clone(), t.n5().clone()),
    (t.n0().clone(), s, t.n2().clone(), t.n3().clone(), t.n4().clone(), t.n5().clone()),
    (t.n0().clone(), t.n1().clone(), s, t.n3().clone(), t.n4().clone(), t.n5().clone()),
    (t.n0().clone(), t.n1().clone(), t.n2().clone(), s, t.n4().clone(), t.n5().clone()),
    (t.n0().clone(), t.n1().clone(), t.n2().clone(), t.n3().clone(), s, t.n5().clone()),
    (t.n0().clone(), t.n1().clone(), t.n2().clone(), t.n3().clone(), t.n4().clone(), s))

impl<T: Send + Clone + Shrink> Shrink for Option<T> {
    fn shrink(&self) -> Lazy<Option<T>> {
        do Lazy::create |L| {
            match *self {
                None => {}
                Some(ref x) => {
                    L.push(None);
                    L.push_map(x.shrink(), |y| Some(y));
                }
            }
        }
    }
}

impl<T: Send + Clone + Shrink, U: Send + Clone + Shrink> Shrink for Result<T, U> {
    fn shrink(&self) -> Lazy<Result<T, U>> {
        do Lazy::create |L| {
            match *self {
                Ok(ref x) => L.push_map(x.shrink(), |y| Ok(y)),
                Err(ref x) => L.push_map(x.shrink(), |y| Err(y)),
            }
        }
    }
}

impl<T: Send + Clone + Shrink, U: Send + Clone + Shrink> Shrink for Either<T, U> {
    fn shrink(&self) -> Lazy<Either<T, U>> {
        do Lazy::create |L| {
            match *self {
                Left(ref x) => L.push_map(x.shrink(), |y| Left(y)),
                Right(ref x) => L.push_map(x.shrink(), |y| Right(y)),
            }
        }
    }
}

impl<T: Send + Shrink> Shrink for ~T {
    fn shrink(&self) -> Lazy<~T> {
        do Lazy::create |L| {
            L.push_map((**self).shrink(), |u| ~u);
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

impl<T: Send + Clone + Shrink> Shrink for ~[T] {
    fn shrink(&self) -> Lazy<~[T]> {
        let mut L = Lazy::new();
        if self.len() == 0 {
            return L;
        }

        L.push(~[]);

        do L.push_thunk(self.clone()) |L, v| {
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
            do L.push_thunk(v) |L, v| {
                for std::uint::range(0, v.len()) |index| {
                    /* remove one at a time */
                    do L.push_thunk((index, v.clone())) |L, (index, v)| {
                        let mut v1 = v.clone();
                        v1.remove(index);
                        L.push(v1);
                        /* shrink one at a time */
                        do L.push_thunk((index, v)) |L, (index, v)| {
                            do L.push_map_env(v[index].shrink(), (index, v)) |selt, &(index, v)| {
                                let mut v1 = v.clone();
                                v1[index] = selt;
                                v1
                            }
                        }
                    }
                }
            }
        }
        L
    }
}


impl<T: Send + Clone + Shrink> Shrink for Cell<T> {
    fn shrink(&self) -> Lazy<Cell<T>> {
        do Lazy::create |L| {
            if !self.is_empty() {
                let v = self.with_ref(|x| x.clone());
                L.push(Cell::new_empty());
                L.push_map(v.shrink(), |y| Cell::new(y));
            }
        }
    }
}
