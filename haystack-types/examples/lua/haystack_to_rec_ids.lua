--[[
Parses a zinc grid of records and transforms it into a list of record IDs.

Usage:
  As one line
  * haystack-client default read --filter "point and (power or energy) and equipRef->siteMeter" | lua haystack_to_rec_ids.lua | haystack-client default hisRead 2025-01-20

  As a script
  ( # () Opens a sub-shell
    export HAYSTACK_AUTH_CONFIG=`hs default auth`;
    
    POINT_IDS="$(haystack-client read --filter "point and (power or energy) and equipRef->siteMeter" | lua haystack_to_rec_ids.lua)"
    haystack-client hisRead 2025-01-01,2025-06-30 ${POINT_IDS}
  )
]]

hs = require("haystack")

io.input(io.stdin)
result = io.read("*all")

grid = hs.io.parse.zinc.grid(result)

for i=1, #grid do
  print("@" .. grid[i].id["id"])
end