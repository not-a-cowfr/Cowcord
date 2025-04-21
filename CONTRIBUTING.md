# Setting up

## Requirements

- [ ] git
- [ ] Some non ancient rustc version

## Testing

1. Clone the repo `git clone https://github.com/not-a-cowfr/cowcord.git && cd cowcord`
2. intsall Dioxus cli `cargo install dioxus-cli`
3. Run `dx serve --platform <web|desktop>`

## Building

cowcord currently has 3 main cargo profiles, dev, release and beta, release is optimized for binary size and beta is optimized for speed, at the expensive of a larger binary and more of a possibility to crash, and then of course dev is just for the fastest build times with performance out the window

### Web

1. Run thiss
```sh
dx build --release --profile <beta|release|dev> # no needs to specify platform, web is the default
cp -r ./target/dx/Cowcord/release/web/public ./dist
cp ./dist/index.html ./dist/404.html
```
(yes --release is necessary even when using beta profile)
<!--
2. Optimize wasm, run this
```sh
sudo apt-get update
sudo apt-get install -y binaryen
wasm-opt dist/assets/dioxus/Cowcord_bg.wasm -o dist/assets/dioxus/Cowcord_bg.wasm -O4 # O4 for speed OZ for binary size
```
-->
2. Done! everything should now be in the `/Dist` directory

### Desktop

1. Run this
```sh
dx build --release --profile <beta|release|dev> --platform desktop
```

# Consistency

<details><summary><h2>Endpoints</h2></summary>

### 1. Declaring Endpoints

If the endpoint has no changing string query fields or a part of the url is not always the same, then define it as a const, like this:
```rust
pub const SUPER_COOL_ENDPOINT: &str = "/super/cool";
```

However, with a lot of endpoints they have something that changes, like maybe a part of the url is a guild id, or it needs some strign qiery parameters, in this case you would define it as a function, keeping the upper snake case, example:
```rust
pub fn SUPER_COOL_ENDPOINT_ENDPOINT(some_id: Snowflake, query: QueryStringParamsStruct) -> String {
	format!("/super/{}/cool{}", some_id, to_string_query(query))
}
```

Also important, make sure to end the variable/struct/function/type name with what is format
```rust
/// notice the ENDPOINT at the end
pub const SUPER_COOL_ENDPOINT: &str = "/super/cool";

pub struct SuperCoolRequest {}

pub type SuperCoolResponse = SomeOtherThing;
```

And finally, make sure to include importnat info witht he endpoint, for example:
```rust
/// Type: post
///
/// supports Super-Cool-Header header
///
/// requires SUPER_COOL permission
pub const SUPER_COOL_ENDPOINT: &str = "/super/cool";
```

<!-- ### 2. Keep request and response structs seperate, even if they're the same
This is because if in the future if one changes its very easy to edit them and it just -->

</details>

# Troubleshooting
