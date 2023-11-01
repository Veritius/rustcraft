# Logger

Provides an API surface for the `logging` Rust crate.

| Function | Arguments                             | Returns | Description                |
| -------- | ------------------------------------- | ------- | -------------------------- |
| error    | `origin` (String), `message` (String) | Nothing | Logs at the error level.   |
| warn     | `origin` (String), `message` (String) | Nothing | Logs at the warning level. |
| info     | `origin` (String), `message` (String) | Nothing | Logs at the info level.    |
| debug    | `origin` (String), `message` (String) | Nothing | Logs at the debug level.   |
| trace    | `origin` (String), `message` (String) | Nothing | Logs at the trace level.   |
