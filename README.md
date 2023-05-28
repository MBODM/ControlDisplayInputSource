# ControlDisplayInputSource
Some stuff for changing a display´s input source by using DDC

### What it is

The projects in this repository are tiny wrappers of a tool named [ControlMyMonitor](https://www.nirsoft.net/utils/control_my_monitor.html), written by Nir Sofer (NirSoft).

The wrappers are:
- A tiny .NET 6 class library, offering the DDC VCP 60 functionality in .NET projects.
- A tiny .NET 6 console application (same as Rust version), to quickly switch the display input source.
- A tiny Rust CLI executable (same as .NET version), to quickly switch the display input source.

### Why it exists

It is possible to change the input source of a display/monitor with software (in contrast to the hardware buttons of your display), by using DDC commands. The DDC VCP 60 command is the key component here. Since i needed a simple way to quickly switch my display´s input source between 2 computers in 1 click, i found above mentioned tool and wrote some wrappers around it, for my personal use. The reason why i not just used some batch script has to do with some weird behaviour of Windows, when you stick a .lnk to a batch file to the Windows TaskBar.

### Used external tool(s)

- As mentioned above, i use a tool name "_ControlMyMonitor_" for the real DDC stuff. The tool is written by Nir Sofer (NirSoft) and distributed under a Freeware license. So please respect the license terms!
- You can find all information about the tool or the license at --> https://www.nirsoft.net/utils/control_my_monitor.html
- Please also have a look into the [Readme.txt](https://github.com/MBODM/ControlDisplayInputSource/tree/main/Tool/Readme.txt) file inside the [Tool](https://github.com/MBODM/ControlDisplayInputSource/tree/main/Tool) folder in this repository.

@Nir: Thanks a lot for this great tool!

#### Have fun.





