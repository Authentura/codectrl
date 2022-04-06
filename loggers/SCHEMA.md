# Table of contents

- [Log schema](#schema-for-language-loggers)
	- [Important notes](#important-notes)
- [API](#api-for-language-loggers)
	- [Important notes](#important-notes-1)

# Schema for language loggers

A logger for any language ***must*** implement the following schema:

- `stack : array<trace>`: A list of stack traces
    - Where each `trace` contains the following:
        - `name : string`
        - `file_path : string`
        - `line_number : uint`
        - `column_number : uint`
        - `code : string`
- `line_number : uint`: The line number where the log function was called.
- `code_snippet : map<string, string>`: Where each `string` key represents the line 
  number, and where each `string` value is the line of code.
- `message : string`: A custom message to be displayed in codeCTRL. Can 
  technically be anything but it would be better if a more informative message was supplied.
- `message_type : string`: The type of the message that was sent through the log function.
- `file_name : string`: The file name that the log function was called in.
- `address : string`: The address of the host that sent the log data.
- `language : string`: The full name of the language of the logged code.

See below for an example JSON output.

```json
{
  "stack": [
    ...
    {
      "name": "code_ctrl_logger::tests::test_backtrace_final_layer::{{closure}}",
      "file_path": "/var/home/sboyden/Code/codeCTRL/logger/rust/src/tests.rs",
      "line_number": 8,
      "column_number": 16,
      "code": "let a = || crate::Log::log(\"Hello\", Some(2), None);"
    }
  ],
  "line_number": 8,
  "code_snippet": {
    "6": "",
    "7": "fn test_backtrace_final_layer() {",
    "8": "    let a = || crate::Log::log(\"Hello\", Some(2), None);",
    "9": "    let _ = a();",
    "10": "}"
  },
  "message": "\"Hello\"",
  "message_type": "&str",
  "file_name": "src/tests.rs",
  "address": "127.0.0.1",
  "language": "Rust"
}
```

## Important notes

- This schema is not stable and should be expected to be modified and/or changed in the 
future.
- Language loggers _must_ send data as either CBOR or JSON. Other formats are not supported and not planned.

# API for language loggers

Language loggers should implement the required functions at the very least to be compatible with a consistent public API with every other language logger. Naming conventions should be followed on a per-language basis, and do not need to be consistent throughout languages, as long as the actual names are still the same (i.e `log_if` in Rust and `logIf` in Java).

The following are the required functions (using `snake_case` naming convention):

1. `log` - The basic log function. Should take 4 total parameters: `message : T` (required), `surround : uint` (optional), `host : string` (optional), `port : string/int` (optional).
2. `log_if` - A conditional log function. Essentially a wrapper over `log`. Should take 5 total parameters: `condition : closure/lambda/anonymous function` (required), `message : T` (required), `surround : uint` (optional), `host : string` (optional), `port : string/int` (optional).
	- `condition` _must_ return a `bool` and _must_ be required. If `condition` evaluates to `true`, then the log must be sent.
3. `log_if_env` - Another conditional log function, though this only logs if the `CODECTRL_DEBUG` environment variable is set. Should take 4 total parameters: `message : T` (required), `surround : uint` (optional), `host : string` (optional), `port : string/int` (optional).

As each of the log functions can take a `message`, `surround`, `host`, and `port`, here is some requirements for each parameter: 

- `message` _must not_ be an optional parameter and can be any `T` where `T` has an acceptable readable print-out. In the case of Rust, `log` takes any `T` which implements `Debug` and uses that as its readable print-out.
- `surround` must be optional<sup>1</sup>. `surround` must be a non-negative number (hence the `uint` typing). If the language has no concept of unsigned integers, please make sure that the log function explicitly checks that the `surround` argument is not negative and returns an acceptably descriptive error. The default value must be 3 for consistency.
- `host` must be optional<sup>1</sup>. The default value must be "127.0.0.1" for consistency.
- `port` must be optional<sup>1</sup>. The default value must be "3001" for consistency.

## Important notes
- If the target language is a compiled language, please document clearly that the logger will not work properly without debug symbols compiled into the binary. This can be done by hard-coding a warning that a log function was called in a non-debug environment. See [here](https://github.com/Authentura/codectrl-rust-logger/blob/718fc215854de2dc72c7eabba5174797fcd106a0/src/lib.rs#L123-L130) for an example.

---
1. if not possible in the language, use an `Option` type like in Rust/Haskell/etc. If that's not possible then make all parameters required and __clearly__ document default values in a place easily found by developers, preferably in doc comments which should appear as pop-up documentation in sensible code-editors and IDEs.