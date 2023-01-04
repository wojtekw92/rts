# RTS(rust ts)

`rts` adds a timestamp to the beginning of each line of input. The goal is to replace classic `ts` tool.


## Current status

`rts` is adding current UTC time in rfc2822 format at the begging of the each output line.


## TODO

Make it fully backword compatible with `ts`. That means support `ts [-r] [format] `
- If the -r switch is passed, it instead converts existing timestamps in the input to relative times, such as "15m5s ago".
- The optional format parameter controls how the timestamp is formatted, as used by strftime(3). 