namespace ControlDisplayInputSource.Core
{
    public interface IDisplayControl
    {
        string ToolFileName { get; }
        string ToolFilePath { get; }

        Task<string> DetectAsync(CancellationToken cancellationToken = default);
        Task<string> CapabilitiesAsync(int display, CancellationToken cancellationToken = default);
        Task<string> GetVcpAsync(int display, int code, CancellationToken cancellationToken = default);
        Task<string> SetVcpAsync(int display, int code, int value, CancellationToken cancellationToken = default);
    }
}
