# RTS(rust ts)

`rts` adds a timestamp to the beginning of each line of input. The goal is to replace classic `ts` tool.

## install

```
curl -L -o rts https://github.com/wojtekw92/rts/releases/latest/download/rts-x86_64-unknown-linux-gnu
chmod +x rts
cp target/x86_64-unknown-linux-gnu/release/rts ./rts-x86_64-unknown-linux-gnu
```

to test run:

```
echo "Hello World!" | ./rts
```
You should get output: `[Wed, 04 Jan 2023 13:15:12 +0000] Hello World!`


## Building:

```
RUSTFLAGS='-C target-feature=+crt-static' cargo build --release --target x86_64-unknown-linux-gnu
```

## Current status

`rts` is adding current UTC time in rfc2822 format at the begging of the each output line.


## TODO

Make it fully backword compatible with `ts`. That means support `ts [-r] [format] `
- If the -r switch is passed, it instead converts existing timestamps in the input to relative times, such as "15m5s ago".
- The optional format parameter controls how the timestamp is formatted, as used by strftime(3). 