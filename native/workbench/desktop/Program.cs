using System;
using System.IO;
using System.Reflection;
using System.Windows;
using System.Windows.Markup;

[assembly: AssemblyTitle("INTA Desktop")]
[assembly: AssemblyProduct("INTA Desktop")]
[assembly: AssemblyCompany("INTA Computer Services")]
[assembly: AssemblyVersion("0.1.0.0")]
[assembly: AssemblyFileVersion("0.1.0.0")]

namespace Inta.Desktop
{
    internal static class Program
    {
        [STAThread]
        internal static int Main(string[] arguments)
        {
            try
            {
                ToolActionRoute route;
                if (!ToolActionRoute.Parse(arguments, out route)) return 2;
                string payload = AppDomain.CurrentDomain.BaseDirectory;
                string configuredState = Environment.GetEnvironmentVariable("INTA_DATA_DIR");
                // The core opens its catalog database on startup. Do not let an
                // unsafe override reach the core or another child of the workspace.
                if (configuredState != null &&
                    (String.IsNullOrWhiteSpace(configuredState) || WorkbenchStateDirectory.Resolve(payload) == null))
                    throw new IOException("INTA_DATA_DIR must name an absolute, non-redirected state folder outside the signed application payload.");
                var app = new Application();
                Window window;
                using (var stream = Assembly.GetExecutingAssembly().GetManifestResourceStream("Workspace.xaml"))
                {
                    window = (Window)XamlReader.Load(stream);
                }
                var workspace = new Workspace(window, payload, route);
                int result = app.Run(window);
                GC.KeepAlive(workspace);
                return result;
            }
            catch (Exception error)
            {
                MessageBox.Show(error.GetBaseException().Message, "INTA Desktop could not start", MessageBoxButton.OK, MessageBoxImage.Error);
                return 1;
            }
        }
    }
}
