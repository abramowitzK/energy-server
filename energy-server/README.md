# Energy Server!

MD processes lots of power data and what we need is a server that tells us the 
total amount of energy metered by a system for a given timeframe.  A series of 
sensor data values (time in seconds, voltage in volts, and current in amps) are 
located in the file "sensors.json", and the server should respond with an approximate 
value for kWh measured over the given interval.

It need not be fast, but the code should be readable, and the results should be correct.

## Expected Behavior

  $ ./energy-server &
  [1] 21507
  $ curl -d "starttime=1234&endtime=5678" http://localhost:8080/
  {"results":{"energy":307.668,"units":"kWh"}}
  $

## Notes

Given that file, it took one of us about an hour to implement something that
worked correctly. You're welcome to take it however far you want, but we're
expecting something along those lines.

And if there's anything special we have to do to run your program, just let us
know. A Makefile never hurt anyone.

