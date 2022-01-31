## Rust Password Generator Wasm Binding

This library implements wasm binding for rust password generator (https://github.com/antonmisa/rust_password)
in pure Rust. It's easy to use it in web projects implementing React, NodeJs or Vanilla Html and s.o.


## Build manual

```console
> wasm-pack build
```

## Installation manual
```nodejs package.json
"dependencies": {
    "rust-password-wasm": "file:path to rust-password-wasm/pkg"
```

## Installation npm
```
npm install @antonmisa/rust-password-wasm --save
```

## Usage

```nodejs
import * as wasm from "rust-password-wasm";

export function calculate(length, num, spec, noUpper, allowRepeat) {
    const retval = wasm.get_next(length, num, spec, noUpper, allowRepeat);
}

export function calculate() {
    const retval = wasm.get_default();
}
```

## License

This code is licensed under the MIT license.
