> [!IMPORTANT]
> **Note:** This project is actively being developed, and users may encounter breaking changes when using it in libraries or automation scripts and upgrading between versions. Because of this, please ensure you thoroughly test your workflows when adopting new versions of this tool until the project stabilizes.
>
> If you have any feedback or suggestions for improvement, feel free to raise an issue on [Github](https://github.com/candronikos/haystack).

## Overview
As far as I'm aware, utilities for interacting with zinc or SkySpark native data do not exist. This project aims to fill that gap by providing a suite of tools inspired by the GNU/Linux ecosystem. These tools enable remote access, data reads and writes, and efficient data wrangling of supported data formats.

Users familiar with Linux shells will find it easy to connect to Haystack servers with the same convenience as `ssh` or `mosh`, and chain multiple CLI calls without needing to open a browser. 

Project page can be found [here](https://candronikos.com/projects/haystack-rust/).

## Sub crates
### haystack-client (CLI)
Command line utility for interacting with a haystack server and supporting SkySpark-specific REST calls. The CLI also includes a `repl`.

#### Usage
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
<!-- ```bash
haystack-client $DEST about
# Output printed to STDOUT

# Haystack read query
haystack-client $DEST read --filter "site and geoState==\"NSW\""

# His read HAYSTACK_REFS is a single or multiple Refs beginning with '@' separated by white space
haystack-client $DEST hisRead $HAYSTACK_REFS

# Interactive prompt
# Commands can be entered from here and will be directed to the destination referred to by $DEST
haystack-client $DEST repl
hs〉about
# Output printed to STDOUT
``` -->

#### Configuration
When the destination is entered by name, the utility searches for the `config.yaml` under `%LOCALAPPDATA%` in Windows and `~/.config` in Linux. The tool is configured similarly to ssh config enabling `haystack-client` to choose pre-configured destinations. If destination is not configured the user must enter entire server URL and credentials as command-line parameters. 

```yaml
# identifier used as $DEST in command line
name: default
url: https://localhost/api/demo
username: superUser
password: accountPassword
# Set to true in my case. Should be false in production
accept-invalid-certs: true
```
> [!IMPORTANT]
> Info on `accept-invalid-certs` [here](https://docs.rs/reqwest/0.12.15/reqwest/struct.ClientBuilder.html#method.danger_accept_invalid_certs). If used in production, this should either be set to false or excluded from the config as it will accept any certificate for any site it's connected to.

#### Supported REST Operations
- [x] About
- [x] Close
- [x] Defs
  - [x] filter
  - [x] limit
- [x] Libs
  - [x] filter
  - [x] limit
- [x] Ops
  - [x] filter
  - [x] limit
- [x] Filetypes
  - [x] filter
  - [x] limit
- [x] Read
  - [x] By filter
  - [x] By id
- [x] Nav
- [x] WatchSub
- [x] WatchUnsub
- [x] WatchPoll
- [ ] PointWrite
- [x] hisRead
  - [x] Single
  - [x] Batch
- [x] hisWrite (Must be valid zinc str)
  - [x] Single
  - [x] Batch
- [ ] Invoke Action

### haystack-client (Library)
The underlying API library implementing an asynchronous haystack client. API may change and will be documented further once finalised.

### haystack-types
Rust-native implementations of haystack data types as well as zinc parsers for each type enabling interactivity with haystack servers.

<!-- ### haystack-awk
A `zinc`-flavoured and haystack-aware `awk` implementation for smart processing of zinc-formatted output. This implementation will support haystack data types, provide the ability to construct and interact with haystack values and transform zinc grids in a similar manner to other implementations of `awk`.
 -->
## Project Status
The project is currently in active development:

* The CLI and library components are functional and undergoing improvements.
* `haystack-types` is well under development but its API is subject to change.
<!-- * `haystack-awk` is in the early stages of development. -->

## Conclusion
This project is actively being developed, and support will continue to improve over time.
You may download or build the source code, via the repository link below.

[Github Repository](https://github.com/candronikos/haystack)
