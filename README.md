# MultiModule

This is the multi-modules use the mcall example.

## Build the project

Use follow command to build the modules.

```bash
$ make
```

## Run the test.

```bash
$ bls-runtime target/wasm32-wasi/release/module_a.wasm --module=module_b=target/wasm32-wasi/release/module_b.wasm
```

