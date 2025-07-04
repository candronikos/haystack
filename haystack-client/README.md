# Haystack Client
Rust implementation of an async haystack client library and CLI tool and REPL.

## Implemented Ops
##### Standard Operations
- [x] About
- [x] Close
- [x] Defs: `filter`: (optional), `limit`: (optional)
- [x] Libs: `filter`: (optional), `limit`: (optional)
- [x] Ops: `filter`: (optional), `limit`: (optional)
- [x] Filetypes: `filter`: (optional), `limit`: (optional)
- [x] Read: `filter` / `id`
- [x] Nav: `navId`: (optional)
- [x] WatchSub: `watchDis` / `watchId`, `lease`: (optional)
- [x] WatchUnsub: `watchId`, `id` / `close`: (optional)
- [x] WatchPoll: `watchId`, `refresh`: (optional)
- [x] hisRead: `range`, `ids`: (1+), `timezone`: (optional)
- [x] hisWrite (zinc str - single & batch): `data`
- [ ] PointWrite
- [ ] Invoke Action

##### Skyspark Operations
- [ ] backup
- [ ] evalAll
- [ ] io
- [ ] rec
- [ ] export
- [ ] link
- [ ] file
- [ ] funcShim
- [ ] upload
- [ ] ext

## Example CLI uses
```bash
# Reuse haystack bearer token
( # () Opens a sub-shell
  export HAYSTACK_AUTH_CONFIG=`hs default auth`;
  
  # Destination does not need to be included if the environment variable
  # HAYSTACK_AUTH_CONFIG is set.
  
  # I also set up my .bashrc to use the alias "hs" instead of the full name.
  # Stick with `haystack-client` if you haven't configured your .bashrc this way
  hs read --filter "point" --limit=1

  # Example below of correct character escaping in bash shell when passing
  # filters enclosed by either single or double quotes.
  hs read --filter 'point and unit==\"kWh\" and equipRef->siteRef'
  hs read --filter "point and unit==\\\"kWh\\\" and equipRef->siteRef"
)

# Read–eval–print loop (REPL) You will see the prompt "hs〉"
# To learn about the $DEST argument, see the configuration section below.
haystack-client $DEST repl
hs〉about

# About
haystack-client $DEST about

# Read by filter
haystack-client $DEST read --filter "point and unit==\"kWh\" and equipRef->siteRef"

# Read by ids
haystack-client $DEST read --ids @p:demo:r:2f70054a-87f6d1de @p:demo:r:2f70054a-314342cd

# His Read (single)
haystack-client $DEST hisRead yesterday @p:demo:r:2f70054a-87f6d1de

# His Read (batch, could use the optional --timezone argument)
haystack-client $DEST hisRead yesterday @p:demo:r:2f70054a-87f6d1de @p:demo:r:2f70054a-314342cd

# WatchSub
## Can create watches with or without an id list
haystack-client $DEST watchSub -c "test" @p:demo:r:2f70054a-51d71f8e

## Subscribing points to a watch
haystack-client $DEST watchSub -s "w-2f8e0d48-64f17e75" @p:demo:r:2f70054a-51d71f8e @p:demo:r:2f70054a-69f26216

# Unsubscribing and closing a watch
haystack-client $DEST watchUnsub --close w-2f8e1d8b-8efac249
haystack-client $DEST watchUnsub @p:demo:r:2f70054a-51d71f8e

# Watch Poll (with or without --refresh)
haystack-client $DEST watchPoll w-2f8e2739-3c4b3bde --refresh

# Defs
haystack-client $DEST defs "name==\\\"testJob\\\"" --limit 1
```

## Tests
The client library crate requires environment variables below be set to work:
* `TEST_HAYSTACK_SERVER_URL`
* `TEST_HAYSTACK_SERVER_USER`
* `TEST_HAYSTACK_SERVER_PASSWORD`

You may set them in an `.env` file and load them using the command below:
```bash
export $(cat .env | xargs) && cargo test
```

## TODO
* Implement remaining Haystack OPs
* Provide options that allow the user to control the structure and format of returned grid. i.e. when creating new watches, allow the user to return the `watchId` only.
* Provide option for watches to be reopened automatically if an error grid is returned on watch OPs.
* Treat error grids as errors
* Provide some more advanced tooling to suport `hisWrite`
    - Support to map data from CSV to native haystack types
    - Support for different filetypes
        * CSV, ZINC, JSON, Trio
    - Support methods to map to point rec IDs on the server
    - Embed scripting language, i.e. lua or awk?
* Use an alternative library for SCRAM authentication
