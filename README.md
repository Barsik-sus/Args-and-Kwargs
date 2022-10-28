# Rust ARGS and KWARGS

This crate allows you to use tail `args` and `kwargs`.

If you want an unknown argument size, you can simply do like this:
```rust
#[ extend_args( ARGS( <TYPE> ) ) ]
fn <FN NAME>( <IMPORTANT ARG> : <IMPORTANT ARG TYPE> ) -> <RETURN TYPE>
{
  // do something
}
```
And calls like this:
```rust
<FN NAME>!( <IMPORTANT VALUE>, <OTHER VALUES>, ... );
```
You can set as many required arguments as you need and the rest will be stored into args.

Also you can use named arguments. Do it like this:
```rust
#[ extend_args( KWARGS( <TYPE> ) ) ]
fn <FN NAME>( <IMPORTANT ARG> : <IMPORTANT ARG TYPE> ) -> <RETURN TYPE>
{
  // do something
}
```
This is the same to `ARGS` but now, lets look at how it calls:
```rust
<FN NAME>!( <IMPORTANT VALUE>, <KEY NAME> = <ARG VALUE>, ... );
```

And, ofcourse, you can use both of it:
```rust
#[ extend_args( ARGS( <ARGS TYPE> ), KWARGS( <KWARGS TYPE> ) ) ]
fn <FN NAME>( <IMPORTANT ARG> : <IMPORTANT ARG TYPE> ) -> <RETURN TYPE>
{
  // do something
}
```
Note that named and unnamed arguments are separated by `;`
```rust
<FN NAME>!( <IMPORTANT VALUE>, <OTHER POS ARGS>, ... ; <KEY NAME> = <ARG VALUE>, ... );
```

For more examples see [here](examples/examples.md)