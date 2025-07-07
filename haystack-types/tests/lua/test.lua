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
    self.about_grid = hs.io.grid.from_zinc(zinc_str)
    self.empty_grid = hs.io.grid.from_zinc("ver:\"3.0\"\nempty\n")
  end
  function TestGrid:test_from_zinc()
    lu.assertEquals(self.about_grid[1].haystackVersion, "3.9.15.3111")
    lu.assertEquals(self.about_grid[1].projName, "demo")
    lu.assertEquals(self.about_grid[1].serverName, "0f2e2bae5e16")
    lu.assertEquals(self.about_grid[1].serverBootTime, hs.io.time.from_iso("2025-07-04T07:27:33.826Z"))
    lu.assertEquals(self.about_grid[1].serverTime, hs.io.time.from_iso("2025-07-06T03:10:07.57Z"))
    lu.assertEquals(self.about_grid[1].productName, "SkySpark")
    lu.assertEquals(self.about_grid[1].productUri, "https://skyfoundry.com/skyspark")
    lu.assertEquals(self.about_grid[1].productVersion, "3.1.11")
    lu.assertEquals(self.about_grid[1].vendorName, "SkyFoundry")
    lu.assertEquals(self.about_grid[1].vendorUri, "https://skyfoundry.com/")
    lu.assertEquals(self.about_grid[1].moduleName, "skyarcd")
    lu.assertEquals(self.about_grid[1].moduleVersion, "3.1.11")
    lu.assertEquals(self.about_grid[1].tz, "UTC")
  end
  function TestGrid:test_index()
    lu.assertEquals(self.about_grid[2], nil)
    lu.assertNotEquals(self.about_grid[1], nil)
  end
  function TestGrid:test_len()
    lu.assertEquals(#self.about_grid, 1)
    lu.assertNotEquals(#self.about_grid, 2)
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

--[[ grid = hs.io.grid.from_zinc(zinc_str)
print("Print Grid")
print(grid)
print("Print Grid Row")
row = grid[1]
print(row)
print(row["projName"])
print(row["helllo"])
row = grid[2]
print(row)
 ]]
os.exit( lu.LuaUnit.run() )