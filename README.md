## Mongo raw access experiment

Simple binary to allow for experiments. Just pipe a command doc over stdin into the binary or write it as first argument.
Find the command syntax here: https://docs.mongodb.com/manual/reference/command/

### Setup & Usage

#### Rust

- Have a Rust toolchain.
- `cargo build` to build the binary.
- `docker-compose up -d` for the test MongoDB.
- `./target/debug/mongo-commands <your-json-cmd-string>` to invoke the binary with an arg.
- OR: `cat your.json | ./target/debug/mongo-commands` to invoke the binary with stdin.

#### JS

- Have Node.js installed
- `npm install`
- `node src/main.js <your-json-cmd-string>` to invoke the binary with an arg.
- OR: `cat your.json | node src/main.js` to invoke the binary with stdin.

### Examples

Take a look at the examples folder. Just use it like: `cat ./examples/insert.json | ./target/debug/mongo-commands`
