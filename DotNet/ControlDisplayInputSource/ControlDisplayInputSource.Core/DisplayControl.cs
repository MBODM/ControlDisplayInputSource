using System.Diagnostics;

namespace ControlDisplayInputSource.Core
{
    public sealed class DisplayControl : IDisplayControl
    {
        private readonly string toolFolder;

        public DisplayControl(string toolFolder)
        {
            if (string.IsNullOrWhiteSpace(toolFolder))
            {
                throw new ArgumentException($"'{nameof(toolFolder)}' cannot be null or whitespace.", nameof(toolFolder));
            }

            this.toolFolder = toolFolder;
        }

        public string ToolFileName => "winddcutil.exe";
        public string ToolFilePath => Path.Combine(toolFolder, ToolFileName);

        public Task<string> DetectAsync(CancellationToken cancellationToken = default) =>
            Run("detect", true, cancellationToken);
        public Task<string> CapabilitiesAsync(int display, CancellationToken cancellationToken = default) =>
            Run($"capabilities {display}", true, cancellationToken);
        public Task<string> GetVcpAsync(int display, int code, CancellationToken cancellationToken = default) =>
            Run($"getvcp {display} {code}", false, cancellationToken);
        public Task<string> SetVcpAsync(int display, int code, int value, CancellationToken cancellationToken = default) =>
            Run($"setvcp {display} {code} {value}", false, cancellationToken);

        private async Task<string> Run(string arguments, bool validateExitCode, CancellationToken cancellationToken = default)
        {
            if (!File.Exists(ToolFilePath))
            {
                throw new InvalidOperationException($"Could not found '{ToolFileName}' file.");
            }

            var psi = new ProcessStartInfo
            {
                FileName = ToolFileName,
                Arguments = arguments,
                RedirectStandardOutput = true,
                RedirectStandardError = true
            };

            using var process = Process.Start(psi) ?? throw new InvalidOperationException($"Could not start '{ToolFileName}' process.");
            await process.WaitForExitAsync(cancellationToken).ConfigureAwait(false);

            if (validateExitCode)
            {
                if (process.ExitCode != 0)
                {
                    var error = await process.StandardError.ReadToEndAsync().ConfigureAwait(false);
                    var output1 = await process.StandardOutput.ReadToEndAsync().ConfigureAwait(false);


                    var innerException = new InvalidOperationException(error);
                    throw new InvalidOperationException($"Exit code of '{ToolFileName}' process was not 0 (SUCCESS).", innerException);
                }
            }

            var output = await process.StandardOutput.ReadToEndAsync().ConfigureAwait(false);
            return output;
        }
    }
}
