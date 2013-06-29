// vim: sts=4 sw=4 et

/*!
 Lazy is a Lazily generated sequence, only traversable once, implementing Iterator.

 It allows lazy generation by allowing generators to tack on thunks of closures
 that are not called until the list is traversed to that point.

 Only has list structure if all thunks are nested inside each other. Otherwise it is
 more like a tree.

 Uses a custom ~Thunk and ~Callable to allow moving in and then mutating values in
 the closure.
 */

/// Lazily generated sequence, only traversable once
pub struct Lazy<T> {
    priv head: ~[T],
    priv thunks: ~[~Callable<Lazy<T>>],
}

trait Callable<T> {
    fn call(~self, &mut T);
}

struct Thunk<A, B> {
    value: A,
    f: ~fn(A, &mut B),
}

impl<A, B> Callable<B> for Thunk<A, B> {
    fn call(~self, x: &mut B) {
        (self.f)(self.value, x)
    }
}

impl<T> Lazy<T> {
    pub fn new() -> Lazy<T> {
        Lazy::new_from(~[])
    }

    pub fn new_from(v: ~[T]) -> Lazy<T> {
        Lazy{head: v, thunks: ~[]}
    }

    pub fn create(f: &fn(&mut Lazy<T>)) -> Lazy<T> {
        let mut L = Lazy::new();
        f(&mut L);
        L
    }

    pub fn next(&mut self) -> Option<T> {
        while self.head.len() == 0 && self.thunks.len() > 0 {
            let next = self.thunks.shift();
            next.call(self);
        }
        if self.head.len() > 0 {
            Some(self.head.shift())
        } else {
            None
        }
    }

    /// push a value to the end of the Lazy.
    pub fn push(&mut self, x: T) {
        self.head.push(x);
    }

    /// push a thunk to the end of the thunk list of lazy.
    /// ordered after all immediate push values.
    pub fn push_thunk<A: Owned>(&mut self, x: A, f: ~fn(A, &mut Lazy<T>)) {
        let t = ~Thunk { value: x, f: f};
        self.thunks.push(t as ~Callable<Lazy<T>>)
    }

    /// lazily map from the iterator `a` using function `f`, appending the results to self
    pub fn push_map<A, J: Owned + Iterator<A>>(&mut self, a: J, f: ~fn(A) -> T) {
        do self.push_thunk((f, a)) |mut (f, a), L| {
            match a.next() {
                None => {},
                Some(x) => {
                    L.push(f(x));
                    L.push_map(a, f);
                }
            }
        }
    }
}

impl<T> Iterator<T> for Lazy<T> {
    fn next(&mut self) -> Option<T> { self.next() }
}

#[test]
fn test_lazy_list() {
    let mut L = do Lazy::create |L| {
        L.push(3);
        do L.push_thunk(~[4,5]) |mut v, L| {
            L.push(v.shift());
            do L.push_thunk(v) |mut v, L| {
                L.push(v.shift());
            }
        }
    };

    assert_eq!(L.next(), Some(3));
    assert_eq!(L.next(), Some(4));
    assert_eq!(L.next(), Some(5));
    assert_eq!(L.next(), None);
}
