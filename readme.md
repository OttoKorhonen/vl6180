# VL6180 time of flight sensor/proximity sensor

This is a driver for vl6180 tof sensor written in Rust programming language. Using Embedded-hal crate.

Datasheet:
> https://www.alldatasheet.com/datasheet-pdf/view/1444681/STMICROELECTRONICS/VL6180.html

> http://www.st.com/st-web-ui/static/active/en/resource/technical/document/application_note/DM00122600.pdf

### Description:
The VL6180 is the latest product based on ST’s
patented FlightSense™ technology. This is a
ground-breaking technology allowing absolute
distance to be measured independent of target
reflectance. Instead of estimating the distance by
measuring the amount of light reflected back from
the object (which is significantly influenced by
color and surface), the VL6180 precisely
measures the time the light takes to travel to the
nearest object and reflect back to the sensor
(Time-of-Flight).

Combining an IR emitter and a range sensor in a
two-in-one ready-to-use reflowable package, the
VL6180 is easy to integrate and saves the end-
product maker long and costly optical and
mechanical design optimizations.

The module is designed for low power operation.
Ranging measurements can be automatically
performed at user defined intervals. Multiple
threshold and interrupt schemes are supported to
minimize host operations.

Host control and result reading is performed using
an I2C interface. Optional additional functions,
such as measurement ready and threshold
interrupts, are provided by two programmable
GPIO pins.

A complete API is also associated with the device
which consists of a set of C functions controlling
the VL6180 to enable fast development of end-
user applications. This API is structured in a way
that it can be compiled on any kind of platform
through a well isolated platform layer (mainly for
low level I2C access).

### Features:
1. Two-in-one smart optical module
    - VCSEL light source
    - Proximity sensor
2. Fast, accurate distance ranging
    - Measures absolute range from 0 to 62 cm max (depending on conditions)
    -Independent of object reflectance
    - Ambient light rejection
    - Cross-talk compensation for cover glass
3. Gesture recognition
    - Distance and signal level can be used by host system to implement gesture recognition
    - Demo systems (implemented on Android
smartphone platform) available.
4. Easy integration
    - Single reflowable component
    - No additional optics
    - Single power supply
    - I²C interface for device control and data
    - Provided with a documented C portable
API (Application Programming Interface)
5. Two programmable GPIO
    - Window and thresholding functions for
ranging