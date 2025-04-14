# Haystack Client
Rust implementation of an async haystack client library and CLI tool and REPL.

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
- [x] WatchSub
- [x] WatchUnsub
- [x] WatchPoll
- [ ] PointWrite
- [x] HisRead
    - [x] Single hisRead
    - [x] Batch hisRead
- [*] HisWrite (Must be valid zinc str)
    - [x] Single hisWrite
    - [x] Batch hisWrite
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

# WatchSub
## Can create watches with or without an id list
haystack-client default watchSub -c "test" @p:demo:r:2f70054a-51d71f8e

## Subscribing points to a watch
haystack-client default watchSub -s "w-2f8e0d48-64f17e75" @p:demo:r:2f70054a-51d71f8e @p:demo:r:2f70054a-69f26216

# Unsubscribing and closing a watch
haystack-client default watchUnsub --close w-2f8e1d8b-8efac249
haystack-client default watchUnsub @p:demo:r:2f70054a-51d71f8e

# Watch Poll (with or without --refresh)
haystack-client default watchPoll w-2f8e2739-3c4b3bde --refresh
```

## TODO
* Implement remaining Haystack OPs
* Provide options that allow the user to control the structure and format of returned grid. i.e. when creating new watches, allow the user to return the `watchId` only.
* Provide option for watches to be reopened automatically if an error grid is returned on watch OPs.
* Treat error grids as errors
* Provide some more advanced tooling to suport `hisWrite`
    - Support to map data from CSV to native haystack types
    - Support for different filetypes
        * CSV, ZINC, JSON
    - Support methods to map to point rec IDs on the server
* Use an alternative library for SCRAM authentication