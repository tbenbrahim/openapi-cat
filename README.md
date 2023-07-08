# OpenAPI Cat

## Description

`openapi-cat` is a command line tool for concatenating OpenAPI 3.x files. When components are merged, they are prefixed
with a unique prefix to avoid name collisions between potentially similarly named objects in the different OpenAPI
specification files. The intended use case is to allow for the creation of a single OpenAPI specification file to be
consumed by a WAF to filter traffic, not to generate a human readable specification.

`openapi-cat` was built as a Rust learning exercise, but is fully functional.

## Usage

```
Usage: openapi-cat [OPTIONS] --config <config>

Options:
  -c, --config <config>  configuration file
  -o, --output <output>  output destination [default: openapi.json]
  -q, --quiet            quiet mode
  -h, --help             Print help
  -V, --version          Print version
```

Test files are included in the `test-data` directory. A sample invocation is shown below:

```bash
openapi-cat -c test-data/config.yaml -o openapi.json
```

This generates a file called `openapi.json` in the current directory.

## Configuration

The configuration file is a YAML file that contains a list of OpenAPI files to concatenate.
A sample configuration file is shown below:

```yaml
applications:
  - name: Customer API
    prefix: cust
    path: /customers
    spec: test-data/link-example.yaml
  - name: Products API
    prefix: prod
    path: /products
    spec: test-data/api-with-examples.json
  - name: Marketing API
    prefix: mrk
    path: /marketing
    spec: test-data/petstore.json
  - name: Sales API
    prefix: sales
    path: /sales
    spec: test-data/openapi.yaml
```

An array of applications is defined. Each application has the following properties:

- `name`: The name of the application. This is is optional and for documentation purposes only.
- `prefix`: The prefix to use for prefixing components in the OpenAPI specification. This is required and must be
  unique.
- `path`: The path to prepend to the specications paths. This is required.
- `spec`: The path to the OpenAPI specification file. This is required. If is a relative path, it is relative to the
  working directory.

## Building

```bash
cargo build --release
```