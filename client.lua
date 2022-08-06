local http = net.HTTPClient();
local ip = '192.168.10.102';
local port = '4000'
local addr = 'http://'..ip..':'..port;
--fibaro:debug (addr)

--1,10,37
http : request(addr..'/carChargerUsage', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('easeeLadeEffekt', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--2,11,38
http : request(addr..'/easeeLadeMengde', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('easeeLadeMengde', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--3,12,39
http : request(addr..'/easeeEnergyPerHour', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('easeeEnergyPerHour', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--Fra tibberSubscribe.py:
--4,13,40
http : request(addr..'/APIpower', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('APIpower', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--5,14
http : request(addr..'/APIpower', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('APIpower_kW', math.floor(response.data * 0.1)/100) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)

--6,15
http : request(addr..'/APIlastMeterConsumption', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('APIlstMtrConsump', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)

--7,21
http : request(addr..'/APIaccumulatedConsumption', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('APIaccCons', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)

--9,20,41
http : request(addr..'/APIaccumulatedConsumptionLastHour', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('APIaccConsLstHr', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)

--8,19
http : request(addr..'/APIminPower', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('APIminPower', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)

--18
http : request(addr..'/APIaveragePower', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('APIavrPower', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)

--30
http : request(addr..'/APImaxPower', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('APImaxPower', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)

--31
http : request(addr..'/8hLow', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('Pris8hLow', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--32
http : request(addr..'/in6Low8', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('Price6of8Low', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)  
--29,33
http : request(addr..'/0_6High', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PriceLev0_6High', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)--PrisnivåBeregning

http : request(addr..'/6_12High', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PriceLev6_12High', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--34
http : request(addr..'/12_18High', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PriceLev12_18High', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--35
http : request(addr..'/18_24High', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PriceLev18_24High', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--28
http : request(addr..'/90_115', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PriceLev90_115', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--36
http : request(addr..'/60_90', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PriceLev60_90', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--27
http : request(addr..'/0_60', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PriceLev0_60', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--26,44
http : request(addr..'/115_140', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PriceLev115_140', response.data) fibaro:debug(response) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--25,43
http : request(addr..'/140_999', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PriceLev140_999', response.data) fibaro:debug(response.data) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
--16,42
http : request(addr..'/PrisTime', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PrisTime', math.floor(tonumber(response.data) * 100)/100) fibaro:debug(response.data) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)--17,41
http : request(addr..'/PrisForhold24', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PrisForhold24', math.floor(tonumber(response.data) * 100)/1) fibaro:debug(response.data) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
  )
--24
http : request(addr..'/PrisMax', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PrisMax', math.floor(tonumber(response.data) * 100)/100) fibaro:debug(response.data) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
  )
--23
http : request(addr..'/PrisMin', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PrisMin', math.floor(tonumber(response.data) * 100)/100) fibaro:debug(response.data) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
   )
--22
http : request(addr..'/PrisSnitt24', 
  {options = {method = "GET", data = "help"},
    success = function(response) fibaro:setGlobal ('PrisAvg', math.floor(tonumber(response.data) * 100)/100) fibaro:debug(response.data) end,
    error = function(err) fibaro:debug ("Error:" ..err) end}
)
