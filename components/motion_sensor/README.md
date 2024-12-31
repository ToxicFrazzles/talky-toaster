# Motion Sensor Component

This component is responsible for interfacing with the Waveshare mmWave presence sensor.

Public documentation found [on the Waveshare Wiki](https://www.waveshare.com/wiki/HMMD_mmWave_Sensor)

Undocumented protocol found via packet sniffing with the configuration tool or from a (currently pending) support request.

## Learned info
* Byte 11 of the commands to switch modes on the documentation is incorrect. Normal mode and Report mode packets should be swapped in the docs.
* The distance value reported in report mode appears to be in centimetres (from observing the values while standing at various distances)