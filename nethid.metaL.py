## @file
## @brief Virtual HID daemon (network remote control)

from metaL import *

p = Project(
    title='Virtual HID daemon (network remote control)',
    about='''
* remote control for collective gaming (slot machine, cinema gaming)
* event processing in soft realtime (network lags not included)
* customizable command translation and grouping
* USB Gadget mode for closed-firmware clients

## Links

* https://github.com/nmelihsensoy/virtual-hid-tcp
''') \
    | Rust() | Game()

p.sync()
