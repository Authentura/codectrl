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

See below for an example JSON output. Please note **the data must be sent as CBOR, the JSON
is only provided as a readable alternative**\*.

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
  "language": "rust"
}
```

## Important notes

- This schema is not stable and should be expected to be modified and/or changed in the 
future.
- As said above, data should be sent to codeCTRL in the CBOR format, not JSON\*. Most, if
  not all languages have support via 1<sup>st</sup> or 3<sup>rd</sup> party libraries or
  packages. See [here](https://cbor.io/impls.html) for a full list of implementations, 
  courtesy of the people who made CBOR.
  
**\* This maybe changing at some point in the near future, where the server accepts both CBOR and JSON, or just JSON. It is TBD.**
