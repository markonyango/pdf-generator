## Prerequisites

You need the following to be installed and setup first:

- [wasm-pack]([https://github.com/drager/wasm-pack])
- [Rust]([https://rustup.rs])

## Compile for browser

Compile with `wasm-pack` first:

```shell
wasm-pack build --target web --no-opt --dev

or

wasm-pack build --target web --release
```

Building the release version may take some time due to the WASM output being optimized for size.
The compiled package will (by default) be written to `./pkg`. Simply import the wrapper JS file
to load the WASM module like this:

```html
<html>
<head>
<meta charset="utf-8"> <!-- This is very important because Rust expects strings to be UTF-8 apparently -->
</head>
<body>
<script type="module">
    import init, { render_pdf } from './pkg/your_app_name_in_cargo_toml.js';

    // Load WASM module
    init()
        .then(() => {
            // Create options object to pass template and font to be passed to Typst compiler
            const render_options = {
                template: `...`, // <-- Your typst template here
                font: [] // <-- Pass the font as array of integers. Can be created from Uint8Array
            };

            // Call the function imported from WASM
            let pdf_bytes = render_pdf(render_options); // Function returns PDF as array of bytes

            const blob = new Blob([pdf_bytes], { type: 'application/pdf' });
            const url = URL.createObjectURL(blob);

            // Open PDF in new tab/window
            window.open(url);
        })
        .catch(console.error);

</script>
</body>
</html>
```

The character set must be set to UTF-8 or else strings sent from JS to WASM are not correctly formated as Rust
expects strings to be valid UTF-8.

Start a web server from the projects root directory (e.g. `python3 -m http.server`) and navigate to the respective URL to see the example 
template ready to be converted to a PDF.


## Compile for NodeJS

Compile with `wasm-pack` first:

```shell
wasm-pack build --target nodejs --no-opt --dev

or

wasm-pack build --target nodejs --release
```

The NodeJS module provides the identical API into the WASM module, therefor calling `render_pdf` with identical
input yields an array of bytes too here.

```js
const fs = require('node:fs');
const { render_pdf } = require('your-apps-name-here');

const minimal_template_example = `
#set page(paper: "a4")

= Example Title

This is example Text
`;

const font = fs.readFileSync('/path/to/font/file/here');
const render_options = { template: minimal_template_example, font };
// In this example we are not providing external data to the compiler to used when creating the PDF
const data = null;

const result = render_pdf(render_options, data);

fs.writeFileSync('output.pdf', result);
```

Go from here and experiment!

# License

This project is licensed under the [MIT License](http://opensource.org/licenses/MIT)

# See also

- [Relacibo/typst-as-lib](https://github.com/Relacibo/typst-as-lib) (this work is based on that awesome wrapper package)
- [rustwasm/wasm-bindgen](https://github.com/rustwasm/wasm-bindgen])
