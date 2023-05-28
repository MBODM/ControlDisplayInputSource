# ControlDisplayInputSource
Some stuff for changing a display´s input source by using DDC

The projects in this repository are tiny wrappers of a tool named [ControlMyMonitor](https://www.nirsoft.net/utils/control_my_monitor.html), written by Nir Sofer (NirSoft).

It is possible to change the input source of a display/monitor with software (in contrast to the hardware buttons of your display), by using DDC commands. The DDC VCP 60 command is the key component here. Since i needed a simple way to quickly switch my display´s input source between 2 computers in 1 click, i found above mentioned tool and wrote some wrappers around it, for my personal use. The reason why i not just used some batch script has to do with some weird behaviour of Windows, when you stick a .lnk to a batch file to the Windows TaskBar.

The wrappers are:
- A .NET 6 class library, offering the DDC VCP 60 
- A .NET 6 console application (same as Rust version)
- A Rust CLI executable (same as .NET version)


