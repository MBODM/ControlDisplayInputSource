using System.Reflection;

namespace ControlDisplayInputSource
{
    internal static class Helper
    {
        private const string AppName = nameof(ControlDisplayInputSource);

        public static string AppTitle
        {
            get
            {
                var version = AppVersion;
                var tail = "(by MBDOM 2023)";

                return version == string.Empty ? $"{AppName} {tail}" : $"{AppName} {version} {tail}";
            }
        }

        // For Console Apps this seems to be the most simple way, in .NET 5 or higher.
        // It is the counterpart of the "Version" entry, declared in the .csproj file.
        public static string AppVersion => Assembly.
            GetEntryAssembly()?.
            GetCustomAttribute<AssemblyInformationalVersionAttribute>()?.
            InformationalVersion ?? string.Empty;

        // This seems to be the way to go, in .NET 5 or higher.
        public static string AppFolder => Path.GetFullPath(AppContext.BaseDirectory);
    }
}
