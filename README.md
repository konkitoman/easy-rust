# Simplify or Easy Rust

## This is a proof of concept!

So in rust if we want a struct with a life time we need to do this:

```rust
struct Context<'a>{
  data: &'a str
}
```

What if we can do this insted:

```rust
struct Context{
  data: &str
}
```

With easy-lg we can do this like:

```rust
#[easy_lg::simple]
struct Context{
  data: &str
}
```

And with generics:

```rust
#[easy_lg::simple]
struct Context{
  data: &T
}
```

## This is for a proposeal that i'am working on!