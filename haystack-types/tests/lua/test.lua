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
    
    lu.assertEquals(row.haystackVersion, str)
    lu.assertEquals(row.projName, "demo")
    lu.assertEquals(row.serverName, "0f2e2bae5e16")
    lu.assertEquals(row.serverBootTime, hs.io.time.from_iso("2025-07-04T07:27:33.826Z"))
    lu.assertEquals(row.serverTime, hs.io.time.from_iso("2025-07-06T03:10:07.57Z"))
    lu.assertEquals(row.productName, "SkySpark")
    lu.assertEquals(row.productUri, "https://skyfoundry.com/skyspark")
    lu.assertEquals(row.productVersion, "3.1.11")
    lu.assertEquals(row.vendorName, "SkyFoundry")
    lu.assertEquals(row.vendorUri, "https://skyfoundry.com/")
    lu.assertEquals(row.moduleName, "skyarcd")
    lu.assertEquals(row.moduleVersion, "3.1.11")
    lu.assertEquals(row.tz, "UTC")
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
    lu.assertEquals(self.list[1], 1)
    lu.assertEquals(self.list[2], 2)
    lu.assertEquals(self.list[3], "hello")
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

os.exit( lu.LuaUnit.run() )