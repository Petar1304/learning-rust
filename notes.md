
# commands
- cargo build
- cargo run
- cargo test
- cargo doc
- cargo publish
- cargo --version
- cargo new hello-rust
- cargo add crate-name

# error handling
- unwrap()
- expect(msg)
- unwrap_or(value)
- ok_or(Error)
- ok() (converts from result to option) {
    ok -> Some<value>
    err -> None
  }
```
  match option {
    Some(val) => val,
    None => 0
  }
```
or
```
  if let Some(val) = option {
    do_smt_with_val(val)
  }
```

- Result<String, Error> (successful case, error case)
- Option(Some/None)

- .unwrap() extracts a value in the contrainer if present
- .ok() converts between Result and Option
- ? operator is used to propagate the absence of a value or a success up the stack

my_string.chars().collect::<Vec<char>>()