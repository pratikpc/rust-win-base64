# Win-Base64 API

## Description

A simple wrapper over the [Windows API](https://github.com/microsoft/windows-rs) Base64 Encoding and Decoding

1. String to Base64 (`encode`)
2. Base64 to String (`decode`)

## [Sample Code](examples/encode-decode-sample/main.rs)

```rust
use win_base64::decode;
use win_base64::encode;
let text = "What's up?😊🥘🍲🚩🏁🚀👌😁👍😍❤️💕🎶";
let base64_string = encode(&text)?;
let decoded_text = decode(&base64_string)?;
// If it worked, text and decoded_text would be the same
println!("Original: {}", text);
println!("Decoded : {}", decoded_text);
```

### OUTPUT

> Original: What's up?😊👌😁👍😍❤️💕🎶  
> Decoded: What's up?😊👌😁👍😍❤️💕🎶

Yes this supports Emojis.

### Similar code to [base64 library](https://crates.io/crates/base64)

```rust
let a = "hello world";
let b = "aGVsbG8gd29ybGQ=";

assert_eq!(encode(a)?, b);
assert_eq!(a, &decode(b)?[..]);
```

### Multiple Decodes

1. Decode as `&mut [u8]` (`decode_as_mut8`)  
   Probably the fastest  
   But the sizes are to be specified manually.  
   So probably the riskiest as well.

   ```rust
    let text = "What's up?";
    let base64_string = encode(&text)?;
    let mut decoded_arr = vec![0u8; 10];
    use win_base64::decode_as_mut8;
    decode_as_mut8(&base64_string, &mut decoded_arr)?;
    // If it worked, text and decoded_text would be the same
    println!("Original: {}", text);
    println!("Decoded : {:?}", decoded_arr);
   ```

2. Decode as `Vec<u8>` (`decode_as_vecu8`)  
   Allocates a vec of the correct size
   Returns this vec with decoded values
   Safer than mut u8 as it knows the decoded size
   ```rust
   let text = "What's up?";
   let base64_string = encode(&text)?;
   use win_base64::decode_as_vecu8;
   let decoded_vec = decode_as_vecu8(&base64_string)?;
   // If it worked, text and decoded_text would be the same
   println!("Original: {}", text);
   println!("Decoded : {:?}", decoded_vec);
   ```
3. Decode as String (`decode`)  
   Present in top most example
   ```rust
   use win_base64::decode;
   use win_base64::encode;
   let text = "What's up?😊🥘🍲🚩🏁🚀👌😁👍😍❤️💕🎶";
   let base64_string = encode(&text)?;
   let decoded_text = decode(&base64_string)?;
   // If it worked, text and decoded_text would be the same
   println!("Original: {}", text);
   println!("Decoded : {}", decoded_text);
   ```
