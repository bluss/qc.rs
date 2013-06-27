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
impl Shrink for int {}
impl Shrink for uint {}
impl Shrink for float {}
impl Shrink for char {}
impl Shrink for i8 {}

impl<T: Owned + Shrink> Shrink for ~T {
    /* FIXME: This impl causes crashes? */
    fn shrink(&self) -> Lazy<~T> {
        do Lazy::create |L| {
            L.push_map((**self).shrink(), |u| ~u);
        }
    }
}

impl Shrink for u8 {
    fn shrink(&self) -> Lazy<u8> {
        do Lazy::create |L| {
            let n = *self;
            if n > 0 {
                L.push(0);
            }
            if n == 2 {
                L.push(1);
            }
            if n > 8 {
                do L.push_thunk(n) |n, list| {
                    list.push(n/2);
                    list.push(n - n/4);
                    list.push(n - n/8);
                }
            }
            if n > 2 {
                do L.push_thunk(n) |n, L| {
                    L.push(n-3);
                    L.push(n-2);
                    L.push(n-1);
                }
            }
        }
    }
}

impl<A: Owned + Clone + Shrink, B: Owned + Clone + Shrink> Shrink for (A, B) {
    fn shrink(&self) -> Lazy<(A, B)> {
        do Lazy::create |L| {
            match self {
                &(ref A, ref B) => {
                    let ac = A.clone();
                    let bc = B.clone();
                    L.push_map(A.shrink(), |a| (a, bc.clone()));
                    L.push_map(B.shrink(), |b| (ac.clone(), b));
                }
            }
        }
    }
}

impl<A: Owned + Clone + Shrink,
     B: Owned + Clone + Shrink,
     C: Owned + Clone + Shrink> Shrink for (A, B, C) {
    fn shrink(&self) -> Lazy<(A, B, C)> {
        let mut L = Lazy::new();
        match *self {
            (ref A, ref B, ref C) => {
                let Ac = A.clone();
                let Bc = B.clone();
                let Cc = C.clone();
                L.push_map(A.shrink(), |s| (s, Bc.clone(), Cc.clone()));
                L.push_map(B.shrink(), |s| (Ac.clone(), s, Cc.clone()));
                L.push_map(C.shrink(), |s| (Ac.clone(), Bc.clone(), s));
            }
        }
        L
    }
}

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

