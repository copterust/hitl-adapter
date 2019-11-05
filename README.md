Hardware In The Loop Adapter
============================

This repository provides a link between a copterust stack running on a hardware
and [gazebo simulator](http://gazebosim.org/). Tested on gazebo version 9.5.0
and [ardupilot_gazebo](https://github.com/khancyr/ardupilot_gazebo) bb3471d.

HOW TO
------

1. Install gazebo9.
2. Install gazebo9 development libraries.
3. Compile and install ardupilot_gazebo plugin. If that fails around OGRE apply
```ardupilot_gazebo_patch``` from this repo and try again.  
```bash
git clone https://github.com/khancyr/ardupilot_gazebo
cd ardupilot_gazebo
mkdir build
cd build
cmake ..
make -j4
sudo make install
```
4. On a separate terminal run ```gazebo --verbose worlds/iris_arducopter_runway.world```
5. Next run ```cargo run``` and watch telemetry while copter flies away.
