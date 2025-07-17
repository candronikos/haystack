--[[
This script parses a haystack his grid from the haysyack-client CLI and
transforms it into a gnuplot script that can be used to visualise the data.

Usage:
  # To display plot in GUI
  (
    export HAYSTACK_AUTH_CONFIG=`hs default auth`;
    
    POINTS="$(haystack-client read --filter "point and (power or energy) and equipRef->siteMeter")"
    POINT_IDS="$(echo "$POINTS" | lua haystack_to_rec_ids.lua)"
    HIS="$(haystack-client hisRead 2025-01-01,2025-01-07 ${POINT_IDS} |
      lua plot_history.lua <(echo "$POINTS"))"
    gnuplot -psc <(echo "$HIS")
  )

  # To view output in terminal
  (
    export HAYSTACK_AUTH_CONFIG=`hs default auth`;
    
    POINTS="$(haystack-client read --filter "point and (power or energy) and equipRef->siteMeter")"
    POINT_IDS="$(echo "$POINTS" | lua haystack_to_rec_ids.lua)"
    haystack-client hisRead 2025-01-01,2025-01-01 ${POINT_IDS} |
      lua plot_history.lua <(echo "$POINTS")
  )
]]

local hs = require("haystack")

io.input(io.stdin)
local result = io.read("*all")

local his = hs.io.parse.zinc.grid(result)
local his_cols = his:cols()
local his_meta = his:meta()
local his_start = his_meta.hisStart
local his_end = his_meta.hisEnd

local begin = [[
set terminal qt #font 'Verdana,9' persist
set xdata time
set timefmt "%Y-%m-%dT%H:%M:%S"

set autoscale

set title "Energy History"
set xlabel "Time"
set ylabel "Electricity Consumption (kWh)"
set y2label "Natural Gas Consumption (CCF)"
set y2tics
set grid

set xrange ["{{his_start}}":"{{his_end}}"]
]]

begin = begin:gsub("{{his_start}}", tostring(his_start)):gsub("{{his_end}}", tostring(his_end))

print(begin)

print("# Define data block")     
print("$his << EOD")     

local headers = ""
local col_index = {}
for col_i, c in ipairs(his_cols) do
  headers = headers .. c.name
  if c:has("id") then col_index[c:meta().id.id] = c.name end
  if col_i < #his_cols then 
    headers = headers .. "\t"
  end
end
print(headers)

for row_i, r in ipairs(his) do
  local row_str = ""
  for col_i, c in ipairs(his_cols) do
    if c.name=="ts" then
      row_str = row_str .. tostring(r[c.name])
    else
      if r:has(c.name) then
        row_str = row_str .. string.format("%.2f",r[c.name].value)
      else
        row_str = row_str .. "empty"
      end
    end

    if col_i < #his_cols then 
      row_str = row_str .. "\t"
    end
  end
   
  print(row_str)
end

print("EOD\n")

io.input(arg[1])
points_zinc = io.read("*all")

local points = hs.io.parse.zinc.grid(points_zinc)
local plot_str = "plot "

-- Create a table of gnuplot-compatible colors for pretty plots
local colour_list = {
  "rgb \"#1f77b4\"", -- blue
  "rgb \"#ff7f0e\"", -- orange
  "rgb \"#2ca02c\"", -- green
  "rgb \"#d62728\"", -- red
  "rgb \"#9467bd\"", -- purple
  "rgb \"#8c564b\"", -- brown
  "rgb \"#e377c2\"", -- pink
  "rgb \"#7f7f7f\"", -- gray
  "rgb \"#bcbd22\"", -- olive
  "rgb \"#17becf\"", -- cyan
  "rgb \"#aec7e8\"", -- light blue
  "rgb \"#ffbb78\"", -- light orange
  "rgb \"#98df8a\"", -- light green
  "rgb \"#ff9896\"", -- light red
  "rgb \"#c5b0d5\"", -- light purple
  "rgb \"#c49c94\"", -- light brown
  "rgb \"#f7b6d3\"", -- light pink
  "rgb \"#c7c7c7\"", -- light gray
  "rgb \"#dbdb8d\"", -- light olive
  "rgb \"#9edae5\""  -- light cyan
}

function get_color(index, opacity)
  local colour = colour_list[(index % #colour_list)]
  if opacity then
    local opacity_hex = string.format("%02x", math.floor((1 - opacity) * 255))
    return colour:gsub("#", "#"..opacity_hex)
  end
  return colour
end

for i, col in ipairs(points) do
  col_name = col_index[col.id.id]

  local is_gas = col:has("naturalGas")
  local opacity = 1
  
  if is_gas then opacity = 0.2 end
  
  plot_str = plot_str .. "$his using \"ts\":\"" .. col_name .. "\" title \"" .. col.id.dis .. "\" with lines lc " .. get_color(i,opacity) .. " lw 1"

  if is_gas then
    plot_str = plot_str .. " axes x1y2"
  else
    plot_str = plot_str .. " axes x1y1"
  end

  if i < #points then
    plot_str = plot_str .. ",\\\n     "
  end
end

print("set datafile missing \"empty\"\n")
print(plot_str)
