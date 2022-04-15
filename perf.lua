wrk.method = "POST"
wrk.headers["Content-Type"] = "application/json"
wrk.headers["Accept"] = "application/json"
wrk.headers["Cookie"] = "auth=9twc+xFIbER3uobvA3QIZ896xpDtBakfJDNhqFGv6kYXMZDwBp+Kvck6d/pwFKK0eODUPn1rhgix0R/xx6O4d0jdMXmyAFKol3K/ciGPfNHHbyuUVVn9T21DtXDfJV0Yjqw83ONElKfirRE2I4XCLgZwnRfvaLV8cme6bw=="

query = [[
  query usersQuery {
    users(limit: 10, offset: 0) {
      name
      createdAt
      email
      role
      userId
      userUuid
    }
  }
]]
-- variables = [[
--   "ids": [1,2,3,4]
-- ]]
-- wrk.body ='{"query": "' .. string.gsub(query, '\n', '') .. '", "variables": {' .. string.gsub(variables, '\n', '') .. '} }'
wrk.body ='{"query": "' .. string.gsub(query, '\n', '') .. '"}'
