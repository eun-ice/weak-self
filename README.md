# WeakSelf
 
[![Build Status](https://travis-ci.org/eun-ice/weak-self.svg?branch=master)](https://travis-ci.org/eun-ice/weak-self)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/eun-ice/weak-self)
[![Cargo](https://img.shields.io/crates/v/weak-self.svg)](https://crates.io/crates/weak-self)
[![Documentation](https://docs.rs/weak-self/badge.svg)](https://docs.rs/weak-self)

WeakSelf is simple way to have a Weak pointer inside a data structure pointing to itself.


## Use Case


Sometimes you want to create a ```struct``` with a pointer to itself or just some other recursive data structure.


```rust
struct Foo {
    me: &Foo
}

impl Foo {
    pub fn new() -> Foo {
        let foo = Foo{
            me: ????
        };
        foo
    }
}

```

This create helps you do that: 

```rust
pub struct Foo {
    weak_self: WeakSelf<Foo>
}

impl Foo {
    pub fn new() -> Arc<Foo> {
        let foo = Arc::new(Foo{
            weak_self: WeakSelf::new()
        });
        foo.weak_self.init(&foo);
        foo
    }
    
    fn weak(&self) -> Weak<Self> {
        self.weak_self.get()
    }
}

```



## Dependencies

This package depends on std only

## Usage

To use WeakSelf, add this to your `Cargo.toml`:

```toml
[dependencies]
weakself = "1.0.2"
```


## License

Licensed under the terms of MIT license and the Apache License (Version 2.0).

See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for details.

