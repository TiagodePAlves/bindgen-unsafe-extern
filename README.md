# bindgen-unsafe-extern

```fish
> cargo build
   Compiling bindgen-unsafe-extern v0.1.0 (/home/marmis/Documents/pamrs/bindtest)
error: extern blocks must be unsafe
 --> src/cool.rs:3:1
  |
3 | / extern "C" {
4 | |     pub fn cool_function(i: ::std::os::raw::c_int, c: ::std::os::raw::c_char);
5 | | }
  | |_^

error: could not compile `bindgen-unsafe-extern` (bin "bindgen-unsafe-extern") due to 1 previous error
```
