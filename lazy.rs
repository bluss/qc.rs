// vim: sts=4 sw=4 et

/*!
 Lazy is a Lazily generated sequence, only traversable once, implementing Iterator.

 It allows lazy generation by allowing generators to tack on thunks of closures
 that are not called until the list is traversed to that point.

 Only has list structure if all thunks are nested inside each other. Otherwise
 it is more like a tree.

 Uses a custom construction with ~Thunk and ~Eval to allow moving a value into
 a once-callable Thunk, and mutating/moving that value when evaluating it.


 This library was first implemented using ~fn but I switched to extern fn.

 */

/// Lazily generated sequence, only traversable once
pub struct Lazy<T> {
    priv head: ~[T],
    priv thunks: ~[~Eval<Lazy<T>>],
}

trait Eval<L> {
    fn eval(~self, &mut L);
}

/// A frozen computation that can be resolved in the context of an L value (a Lazy)
struct Thunk<L, Up> {
    upvar: Up,
    f: extern fn(&mut L, Up),
}

impl<L, Up> Eval<L> for Thunk<L, Up> {
    fn eval(~self, x: &mut L) {
        (self.f)(x, self.upvar)
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
            next.eval(self);
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
    pub fn push_thunk<Up: Send>(&mut self, x: Up,
                                f: &'static fn:'static(&mut Lazy<T>, Up)) {
        let f_extern: extern fn(&mut Lazy<T>, Up) = func_unwrap(f);
        let t = ~Thunk { upvar: x, f: f_extern };
        self.thunks.push(t as ~Eval<Lazy<T>>)
    }

    /// lazily map from the iterator `a` using function `f`, appending the results to self.
    /// Static function without environment.
    pub fn push_map<A, J: Send + Iterator<A>>(&mut self, it: J,
                                              f: &'static fn:'static(A) -> T) {
        let f_extern: extern fn(A) -> T = func_unwrap(f);
        do self.push_thunk((f_extern, it)) |L, mut (f, it)| {
            match it.next() {
                None => {}
                Some(x) => {
                    L.push(f(x));
                    L.push_map(it, f);
                }
            }
        }
    }

    /// Static function with ref to supplied environment.
    pub fn push_map_env<A, J: Send + Iterator<A>, Env: Send>
        (&mut self, it: J, env: Env,
         f: &'static fn:'static(A, &mut Env) -> T) {
        let f_extern: extern fn(A, &mut Env) -> T = func_unwrap(f);
        do self.push_thunk((f_extern, it, env)) |L, mut (f, it, env)| {
            match it.next() {
                None => {}
                Some(x) => {
                    L.push(f(x, &mut env));
                    L.push_map_env(it, env, f);
                }
            }
        }
    }
}

impl<T> Iterator<T> for Lazy<T> {
    fn next(&mut self) -> Option<T> { self.next() }
}

fn func_unwrap<F, R>(f: F) -> R {
    /* Workaround &'static fn() not being Send/Sendable */
    /* this is "safe" for &'static fn to extern fn */
    unsafe {
        let (f, p): (R, *()) = ::std::cast::transmute(f);
        assert!(p.is_null());
        f
    }
}

#[test]
fn test_lazy_list() {
    let mut L = do Lazy::create |L| {
        L.push(3);
        do L.push_thunk(~[4, 5]) |L, mut v| {
            L.push(v.shift());
            do L.push_thunk(v) |L, mut v| {
                L.push(v.shift());
            }
        }
    };

    assert_eq!(L.next(), Some(3));
    assert_eq!(L.next(), Some(4));
    assert_eq!(L.next(), Some(5));
    assert_eq!(L.next(), None);

    let mut M = Lazy::new();
    M.push_map(Lazy::new_from(~[3,4,5]), |x| (x, 1));
    assert_eq!(M.next(), Some((3,1)));
    assert_eq!(M.next(), Some((4,1)));
    assert_eq!(M.next(), Some((5,1)));
    assert_eq!(M.next(), None);
}
