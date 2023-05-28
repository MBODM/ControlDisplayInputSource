using ControlDisplayInputSource;
using ControlDisplayInputSource.Core;

const string DisplayName = "Dell U2410";

var displayControl = new DisplayControl(Helper.AppFolder);

Console.WriteLine();
Console.WriteLine(Helper.AppTitle);
Console.WriteLine();

try
{
    if (!File.Exists(displayControl.ToolFilePath))
    {
        throw new InvalidOperationException($"Could not found '{displayControl.ToolFileName}' in this folder.");
    }

    await displayControl.SetVcpAsync(0, 60, 0x03);
    Console.WriteLine("Have a nice day.");
    Environment.Exit(0);

    //01 03 04 05 0C 0F 11

    Console.WriteLine("Expected display type:");
    Console.WriteLine(DisplayName);
    Console.WriteLine("Supported VCP Codes:");
    Console.WriteLine("0x01 (VGA), 0x03 (DVI-1), 0x04 (DVI-2), 0x05 (Composite), 0x0C (Component), 0x0F (DisplayPort), 0x11 (HDMI)");

    var output1 = await displayControl.DetectAsync();
    if (!output1.Contains(DisplayName))
    {
        throw new InvalidOperationException("Could not detect expected display type.");
    }

    Console.WriteLine("Detected display:");
    Console.WriteLine();
    Console.WriteLine(output1);

    var output2 = await displayControl.CapabilitiesAsync(0);
    Console.WriteLine("Capabilities of display:");
    Console.WriteLine();
    Console.WriteLine(output2);

    var vcp60Delimiter = " 60(";
    if (!output2.Contains(vcp60Delimiter))
    {
        throw new InvalidOperationException("Display not supports VCP 60.");
    }
    
    Console.WriteLine("Supported VCP 60 codes:");
    var vcp60Codes = output2?.Split(vcp60Delimiter).Last().Split(')').First();
    Console.WriteLine(vcp60Codes);
    Console.WriteLine();

    var output3 = await displayControl.GetVcpAsync(0, 60);
    Console.WriteLine("Actual VCP 60 code:");
    Console.WriteLine();
    Console.WriteLine(output3);
    Console.WriteLine();

    
    Console.WriteLine("Have a nice day.");
    Environment.Exit(0);
}
catch (InvalidOperationException e)
{
    Console.WriteLine($"Error: {e.Message}");
    Environment.Exit(1);
}


// Todo:
// Wenn keine param dann kompletten scan mit test ausführen und usage anzeigen
// Wenn param (nur 1 einzige zahl) dann zahl als VCP 60 code interpretieren und auf den ausgang umschalten
