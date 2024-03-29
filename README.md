# Parameters written in Rust

## Project Outlines

The project outlines as described in my blog post about [Open Source Software Collaboration](https://blog.fox21.at/2019/02/21/open-source-software-collaboration.html).

- The main purpose of this software is to handle one template file and generate another file out of that. This is used for CI/CD pipelines, where a .env.dist file is taken to generate a .env file for production environment.
- This list is open. Feel free to request features.

## Examples

TODO

## Installation

```bash
cargo install parameters
```

## Similar projects

- [Parameters written in C++](https://github.com/TheFox/parameters)

## Dev

```bash
./bin/dev.sh -i ./.env.dist -o ./.env.production -e production -n instance1 -r SYMF_ -s @
./bin/dev.sh -i ./.env.dist -r ^
echo -n hello | ./bin/dev.sh -i - -o ./.env.production -e production -n instance1 -r ^SYMF_ -s @
```
