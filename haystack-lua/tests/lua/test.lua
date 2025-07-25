--[[
  haystack-types/tests/lua/test.lua

  This file contains unit tests for the Haystack Lua types library.
  It tests the parsing of Zinc formatted data into grids and lists,
  and verifies the functionality of various methods on these types.
  
  To run the tests ensure `luaunit` is installed and use the command:
  lua haystack-types/tests/lua/test.lua
--]]
  
local lu = require('luaunit')
hs = require("haystack")

zinc_str = [[ver:"3.0" a s b
a,b
1,2
]]

zinc_str = [[ver:"3.0"
haystackVersion,projName,serverName,serverBootTime,serverTime,productName,productUri,productVersion,vendorName,vendorUri,moduleName,moduleVersion,tz,whoami,hostDis,hostModel,hostId
"3.9.15.3111","demo","0f2e2bae5e16",2025-07-04T07:27:33.826Z,2025-07-06T03:10:07.57Z,"SkySpark",`https://skyfoundry.com/skyspark`,"3.1.11","SkyFoundry",`https://skyfoundry.com/`,"skyarcd","3.1.11","UTC","su","SkySpark (Linux)","SkySpark (Linux)",NA
]]

-- zinc_str = "ver:\"3.0\"\na m:1,b\n1,2\n"
TestGrid = {}
  function TestGrid:setup()
    self.about_grid = hs.io.parse.zinc.grid(zinc_str)
    self.empty_grid = hs.io.parse.zinc.grid("ver:\"3.0\"\nempty\n")
  end
  function TestGrid:test_parse_zinc()
    row = self.about_grid[1]
    str = hs.io.parse.zinc.grid("ver:\"3.0\"\nval\n\"3.9.15.3111\"\n"):first()["val"]
    
    lu.assertEquals(tostring(row.haystackVersion), "3.9.15.3111")
    lu.assertEquals(tostring(row.projName), "demo")
    lu.assertEquals(tostring(row.serverName), "0f2e2bae5e16")
    lu.assertEquals(tostring(row.serverBootTime), "2025-07-04T07:27:33.826")
    lu.assertEquals(tostring(row.serverTime), "2025-07-06T03:10:07.57")
    lu.assertEquals(tostring(row.productName), "SkySpark")
    lu.assertEquals(tostring(row.productUri), "https://skyfoundry.com/skyspark")
    lu.assertEquals(tostring(row.productVersion), "3.1.11")
    lu.assertEquals(tostring(row.vendorName), "SkyFoundry")
    lu.assertEquals(tostring(row.vendorUri), "https://skyfoundry.com/")
    lu.assertEquals(tostring(row.moduleName), "skyarcd")
    lu.assertEquals(tostring(row.moduleVersion), "3.1.11")
    lu.assertEquals(tostring(row.tz), "UTC")
  end
  function TestGrid:test_index()
    lu.assertEquals(self.about_grid[2], nil)
    lu.assertNotEquals(self.about_grid[1], nil)
  end
  function TestGrid:test_len()
    lu.assertEquals(#self.about_grid, 1)
    lu.assertNotEquals(#self.about_grid, 2)
    lu.assertEquals(#self.empty_grid, 0)
  end
  function TestGrid:test_is_empty()
    lu.assertEquals(self.about_grid:is_empty(),false)
    lu.assertEquals(self.empty_grid:is_empty(),true)
  end
  function TestGrid:test_first()
    lu.assertNotEquals(self.about_grid:first(),nil)
    lu.assertEquals(self.empty_grid:first(),nil)
  end
  function TestGrid:test_last()
    lu.assertNotEquals(self.about_grid:last(),nil)
    lu.assertEquals(self.empty_grid:last(),nil)
  end
  function TestGrid:test_has()
    lu.assertEquals(self.about_grid:has("projName"),true)
    lu.assertEquals(self.empty_grid:has("projName"),false)
  end

TestList = {}
  function TestList:setup()
    self.list = hs.io.parse.zinc.list("[1,2,\"hello\"]")
    self.empty_list = hs.io.parse.zinc.list("[]")
  end
  function TestList:test_parse_zinc()
    lu.assertEquals(self.list[1]:tonumber(), 1)
    lu.assertNotEquals(self.list[1]:tonumber(), 2)
    lu.assertEquals(tonumber(self.list[2]), 2)
    lu.assertNotEquals(tonumber(self.list[2]), 3)
    lu.assertEquals(tostring(self.list[3]), "hello")
  end
  function TestList:test_index()
    lu.assertNotEquals(self.list[1], nil)
    lu.assertNotEquals(self.list[2], nil)
    lu.assertNotEquals(self.list[3], nil)
    lu.assertEquals(self.list[4], nil)
    lu.assertEquals(self.empty_list[1], nil)
  end
  function TestList:test_len()
    lu.assertEquals(#self.list, 3)
    lu.assertEquals(#self.empty_list, 0)
  end
  function TestList:test_is_empty()
    lu.assertEquals(self.list:is_empty(),false)
    lu.assertEquals(self.empty_list:is_empty(),true)
  end
  function TestList:test_first()
    lu.assertNotEquals(self.list:first(),nil)
    lu.assertEquals(self.empty_list:first(),nil)
  end
  function TestList:test_last()
    lu.assertNotEquals(self.list:last(),nil)
    lu.assertEquals(self.empty_list:last(),nil)
  end

TestString = {}
  function TestString:setup()
    self.obj = hs.io.parse.zinc.list("[\"hello\"]"):first()
    self.empty_str = hs.io.parse.zinc.list("[\"\"]"):first()
  end
  function TestString:test_len()
    lu.assertEquals(#self.obj, 5)
    lu.assertEquals(#self.empty_str, 0)
  end

TestNumber = {}
  function TestNumber:setup()
    self.obj = hs.io.parse.zinc.list("[-1.0,5, 10kWh,2.5kW]")
  end
  function TestNumber:test_members()
    lu.assertEquals(self.obj[1].value, -1.0)
    lu.assertEquals(self.obj[1].unit, nil)
    lu.assertEquals(self.obj[2].value, 5.0)
    lu.assertEquals(self.obj[2].unit, nil)
    lu.assertEquals(self.obj[3].value, 10.0)
    lu.assertEquals(self.obj[3].unit, "kWh")
    lu.assertEquals(self.obj[4].value, 2.5)
    lu.assertEquals(self.obj[4].unit, "kW")
    lu.assertEquals(1, 0)
  end

os.exit( lu.LuaUnit.run() )