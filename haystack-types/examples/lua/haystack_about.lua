--[[
This script can read the `about` response from a Haystack server
and print the server information to the console.
It expects the input to be in Zinc format, which the standard format
for Haystack data exchange.

It uses the `haystack` Lua module to parse the Zinc data.

Usage:
  haystack-client default about | lua haystack_about.lua
]]

hs = require("haystack")

io.input(io.stdin)
result = io.read("*all")

grid = hs.io.parse.zinc.grid(result)
rec = grid:first()

print("Haystack Server Info:")
print("---------------------")
print(string.format("Version:          %s", rec.haystackVersion))
print(string.format("Project:          %s", rec.projName))
print(string.format("Server Name:      %s", rec.serverName))
print(string.format("Server Boot Time: %s", rec.serverBootTime))
print(string.format("Server Time:      %s", rec.serverTime))
print(string.format("Product Name:     %s", rec.productName))
print(string.format("Product URI:      %s", rec.productUri))
print(string.format("Product Version:  %s", rec.productVersion))
print(string.format("Vendor Name:      %s", rec.vendorName))
print(string.format("Vendor URI:       %s", rec.vendorUri))
print(string.format("Module Name:      %s", rec.moduleName))
print(string.format("Module Version:   %s", rec.moduleVersion))
print(string.format("Timezone:         %s", rec.tz))
print(string.format("Current User:     %s", rec.whoami))
print(string.format("Host Label:       %s", rec.hostDis))
print(string.format("Host ID:          %s", rec.hostId))
