# Haystack Client
Rust implementation of an async haystack client library and CLI tool.

## Implemented Ops
- [x] About
- [x] Close
- [ ] Defs
- [ ] Libs
- [x] Ops
- [x] Filetypes
- [x] Read
    - [x] By filter
    - [x] By id
- [x] Nav
- [ ] WatchSub
- [ ] WatchUnsub
- [ ] WatchPoll
- [ ] PointWrite
- [x] HisRead
    - [x] Single hisRead
    - [x] Batch hisRead
- [*] HisWrite
    - [ ] Single hisWrite
    - [ ] Batch hisWrite
- [ ] Invoke Action

## Example CLI uses
```{bash}
# About
haystack-client default about

# Read by filter
haystack-client default read --filter "point"

# Read by ids
haystack-client default read --ids @p:demo:r:2f70054a-87f6d1de @p:demo:r:2f70054a-314342cd

# His Read (single)
haystack-client default hisRead yesterday @p:demo:r:2f70054a-87f6d1de

# His Read (batch, could use the optional --timezone argument)
haystack-client default hisRead yesterday @p:demo:r:2f70054a-87f6d1de @p:demo:r:2f70054a-314342cd

```