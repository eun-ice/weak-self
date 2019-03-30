use std::cell::UnsafeCell;
use std::fmt;
use std::sync::{Arc, Weak};

///WeakSelf is simple way to have a Weak pointer inside a data structure pointing to itself.
///
///
///## Use Case
///
///
///Sometimes you want to create a ```struct``` with a pointer to itself or just some other recursive data structure.
///
///
///```rust ignore
/// struct Foo {
///     me: &Foo
/// }
///
/// impl Foo {
///     pub fn new() -> Foo {
///         let foo = Foo{
///             me: ????
///         };
///         foo
///     }
/// }
///
///```
///
///This create helps you do that:
///
///```rust
/// use weak_self::WeakSelf;
/// use std::sync::{Arc, Weak};
/// pub struct Foo {
///     weak_self: WeakSelf<Foo>
/// }
///
/// impl Foo {
///     pub fn new() -> Arc<Foo> {
///         let foo = Arc::new(Foo{
///             weak_self: WeakSelf::new()
///         });
///         foo.weak_self.init(&foo);
///         foo
///     }
///
///     fn weak(&self) -> Weak<Self> {
///         self.weak_self.get()
///     }
/// }
///
///```
///
///
///
///## Dependencies
///
///This package depends on std only
///
///## Usage
///
///To use WeakSelf, add this to your `Cargo.toml`:
///
///```toml
///[dependencies]
///weakself = "1.0.1"
///```
///
///
///## License
///
///Licensed under the terms of MIT license and the Apache License (Version 2.0).
///
///See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.
///
pub struct WeakSelf<T: ?Sized> {
    cell: UnsafeCell<Option<Weak<T>>>
}

impl<T: ?Sized> WeakSelf<T> {
    /// Constructs a new empty WeakSelf<T>
    pub fn new() -> WeakSelf<T> {
        WeakSelf {
            cell: UnsafeCell::new(None)
        }
    }


    /// Initialize the WeakSelf<T> with an Arc.
    ///
    /// Note: content must point be the only existing Arc, otherwise this method will panig
    pub fn init(&self, content: &Arc<T>) {
        if Arc::strong_count(content) != 1 || Arc::weak_count(content) != 0 {
            panic!("Exclusive access to Arc<T> is required while initializing WeakSelf<T>");
        }
        let weak = Arc::downgrade(content);
        unsafe {
            *self.cell.get() = Some(weak);
        }
    }

    /// get Some Weak<T> pointer to the content, or None if not yet initialized
    pub fn try_get(&self) -> Option<&Weak<T>> {
        unsafe {
            match *self.cell.get() {
                Some(ref weak) => Some(&weak),
                None => None
            }
        }
    }

    /// get a Weak<T> pointer to the content, or panic if not yet initialized
    pub fn get(&self) -> Weak<T> {
        self.try_get().expect("expected WeakSelf to be initialized").clone()
    }
}

unsafe impl<T: ?Sized + Sync + Send> Sync for WeakSelf<T> {}

unsafe impl<T: ?Sized + Sync + Send> Send for WeakSelf<T> {}


impl<T: ?Sized + fmt::Debug> fmt::Debug for WeakSelf<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.try_get() {
            None => { write!(f, "Empty WeakSelf<T>") }
            Some(weak) => fmt::Debug::fmt(weak, f),
        }
    }
}


impl<T: ?Sized> Default for WeakSelf<T> {
    fn default() -> Self {
        WeakSelf::new()
    }
}

