using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Security.Cryptography;
using System.Security.Cryptography.Pkcs;
using System.Security.Cryptography.X509Certificates;
using System.Text;
using System.Text.RegularExpressions;
using System.Web.Script.Serialization;
using System.Windows.Forms;
using Microsoft.Win32;

internal static class Launch {
    internal sealed class Entry { public string path {get;set;} public string sha256 {get;set;} }
    internal sealed class Manifest { public Entry[] files {get;set;} public string sourceDigest {get;set;} }
    static string Quote(string value) {
        var s = new StringBuilder("\""); int slash = 0;
        foreach (char c in value) {
            if (c == '\\') { slash++; continue; }
            s.Append('\\', c == '"' ? slash * 2 + 1 : slash).Append(c);
            slash = 0;
        }
        return s.Append('\\', slash * 2).Append('"').ToString();
    }
    static void Verify(string root, string signer, string expected) {
        byte[] bytes = File.ReadAllBytes(Path.Combine(root, "manifest.json"));
        var cms = new SignedCms(new ContentInfo(bytes), true);
        cms.Decode(File.ReadAllBytes(Path.Combine(root, "manifest.p7s")));
        cms.CheckSignature(true);
        if (cms.SignerInfos.Count != 1 || cms.SignerInfos[0].Certificate.Thumbprint != signer)
            throw new InvalidDataException("Unexpected development signer.");
        var m = new JavaScriptSerializer { MaxJsonLength = 8 * 1024 * 1024 }.Deserialize<Manifest>(Encoding.UTF8.GetString(bytes));
        if (expected != null && expected != m.sourceDigest) throw new InvalidDataException("Build identity mismatch.");
        var listed = new HashSet<string>(StringComparer.OrdinalIgnoreCase);
        foreach (var e in m.files) {
            if (String.IsNullOrEmpty(e.path) || Path.IsPathRooted(e.path) || e.path.Contains(":") || e.path.Split('/','\\').Any(p => p == ".." || p == "." || p.Length == 0) || !listed.Add(e.path.Replace('\\','/')))
                throw new InvalidDataException("Invalid package path.");
            using (var input = File.OpenRead(Path.Combine(root,e.path)))
            using (var sha = SHA256.Create()) {
                if (!String.Equals(BitConverter.ToString(sha.ComputeHash(input)).Replace("-",""),e.sha256,StringComparison.OrdinalIgnoreCase))
                    throw new InvalidDataException("Installed file failed verification: " + e.path);
            }
        }
        if (!new[] {"inta.exe","inta-desktop.exe","inta-capabilities.exe","inta-views.exe","INTA System Tools.exe"}.All(listed.Contains)) throw new InvalidDataException("Incomplete application package.");
        foreach (string f in Directory.GetFiles(root,"*",SearchOption.AllDirectories)) {
            var rel=f.Substring(root.Length+1).Replace('\\','/');
            if (rel!="manifest.json" && rel!="manifest.p7s" && !listed.Contains(rel)) throw new InvalidDataException("Unlisted installed file: "+rel);
        }
    }
#if CLI
    static int PortableLinks(string home, string action) {
        const string path = @"Software\Classes\inta-tool";
        string exe = Path.Combine(home,"INTA Desktop.exe");
        string command = Quote(exe) + " --tool-action \"%1\"";
        if (action != "register" && action != "remove") throw new ArgumentException("Use portable-links register or portable-links remove.");
        using (var existing = Registry.ClassesRoot.OpenSubKey("inta-tool")) {
            if (existing != null) {
                using (var open = existing.OpenSubKey(@"shell\open\command")) {
                    if (!String.Equals(existing.GetValue("INTA Portable Owner") as string,exe,StringComparison.OrdinalIgnoreCase)
                        || open == null || !String.Equals(open.GetValue("") as string,command,StringComparison.OrdinalIgnoreCase)) {
                        Console.WriteLine("Tool links are owned by another installation. Use its existing handler or unregister that copy first.");
                        return 3;
                    }
                }
            } else if (action == "remove") { return 0; }
        }
        if (action == "remove") {
            using (var root = Registry.CurrentUser.OpenSubKey(path,true)) {
                if (root == null || !String.Equals(root.GetValue("INTA Portable Owner") as string,exe,StringComparison.OrdinalIgnoreCase)) return 3;
                using (var sub = root.OpenSubKey(@"shell\open\command", true)) sub.DeleteValue("");
                root.DeleteValue("URL Protocol",false);
                root.DeleteValue("INTA Portable Owner",false);
                root.DeleteValue("",false);
                foreach (string name in new[]{@"shell\open\command",@"shell\open","shell"}) {
                    bool empty; using (var sub=root.OpenSubKey(name)) empty=sub!=null && sub.ValueCount==0 && sub.SubKeyCount==0;
                    if (empty) root.DeleteSubKey(name);
                }
            }
            using (var root=Registry.CurrentUser.OpenSubKey(path)) {
                if (root!=null && (root.ValueCount>0 || root.SubKeyCount>0)) return 0;
            }
            Registry.CurrentUser.DeleteSubKey(path,false);
        } else {
            using (var root=Registry.CurrentUser.CreateSubKey(path)) {
                root.SetValue("INTA Portable Owner",exe);
                root.SetValue("","URL:INTA portable tool action");
                using(var open=root.CreateSubKey(@"shell\open\command")) open.SetValue("",command);
                root.SetValue("URL Protocol","");
            }
        }
        Console.WriteLine(action == "register" ? "Portable tool links registered for this user." : "Portable tool links removed.");
        return 0;
    }
#endif
    [STAThread] static int Main(string[] args) {
        try {
#if GUI
            if (args.Length > 0) {
                if (args.Length == 1) args = new[] { "--tool-action", args[0] };
                if (args.Length != 2 || args[0] != "--tool-action" ||
                    !Regex.IsMatch(args[1], @"\Ainta-tool://open/[a-z0-9][a-z0-9-]{0,79}/(?:install|launch)\z", RegexOptions.CultureInvariant))
                    return 2;
            }
#endif
            string home = AppDomain.CurrentDomain.BaseDirectory.TrimEnd('\\');
            string data = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData),"INTA","desktop-development");
            string pointer=Path.Combine(data,"active.txt");
            bool portable=File.Exists(Path.Combine(home,"inta-portable.marker"));
            var pin = new X509Certificate2(Path.Combine(home,"channel.cer")).Thumbprint;
#if VERIFY
            if (args.Length!=2 || !Regex.IsMatch(args[1],@"\A[a-f0-9]{64}\z")) throw new InvalidDataException("Expected a payload and source fingerprint.");
            Verify(Path.GetFullPath(args[0]),pin,args[1]);
            return 0;
#else
            string build=Path.Combine(home,"base"); string id=null;
            bool verified=false;
            if (!portable && File.Exists(pointer)) {
                try {
                    id=File.ReadAllText(pointer).Trim();
                    if (!Regex.IsMatch(id,@"\A[a-f0-9]{64}\z")) throw new InvalidDataException("Invalid development build pointer.");
                    var candidate=Path.Combine(data,"revisions",id);
                    Verify(candidate,pin,id);
                    build=candidate;verified=true;
                } catch(Exception error) {
                    id=null;
                    string message="INTA could not verify the development build. Opening the verified installer build.\n"+error.Message;
#if CLI
                    Console.Error.WriteLine(message);
#else
                    MessageBox.Show(message,"INTA development update",MessageBoxButtons.OK,MessageBoxIcon.Warning);
#endif
                }
            }
            if (!verified) Verify(build,pin,id);
#if CLI
            if (args.Length == 2 && args[0] == "portable-links") {
                if (!portable) throw new InvalidOperationException("Use the portable copy of INTA for this command.");
                return PortableLinks(home,args[1]);
            }
            string executable="inta.exe";
            if (args.Length > 0 && (args[0] == "capabilities" || args[0] == "tools" || args[0] == "forge" || args[0] == "schema")) {
                executable="inta-capabilities.exe";
                if (args[0] == "capabilities") args=args.Length==1 ? new[]{"discover"} : args.Skip(1).ToArray();
            }
#else
            string executable="inta-desktop.exe";
#endif
            var info=new ProcessStartInfo(Path.Combine(build,executable),String.Join(" ",args.Select(Quote))) { UseShellExecute=false, WorkingDirectory=build };
            if(portable) {
                if(String.IsNullOrEmpty(info.EnvironmentVariables["INTA_DATA_DIR"])) info.EnvironmentVariables["INTA_DATA_DIR"]=Path.Combine(home,"state");
                var parent=Directory.GetParent(home);
                if(parent!=null && String.IsNullOrEmpty(info.EnvironmentVariables["INTA_TOOLBOX"])) {
                    string box=Path.Combine(parent.FullName,"Toolbox");
                    if(File.Exists(Path.Combine(box,".inta-toolbox.json"))) info.EnvironmentVariables["INTA_TOOLBOX"]=box;
                }
            } else {
                info.EnvironmentVariables["INTA_INSTALL_LAUNCHER"]=Path.Combine(home,"INTA Desktop.exe");
                info.EnvironmentVariables["INTA_ACTIVE_POINTER"]=pointer;
            }
            using (var process=Process.Start(info)) {
#if CLI
                process.WaitForExit(); return process.ExitCode;
#else
                return 0;
#endif
            }
#endif
        } catch(Exception e) {
#if CLI || VERIFY
            Console.Error.WriteLine("INTA: "+e.Message);
#else
            MessageBox.Show(e.Message,"INTA could not start",MessageBoxButtons.OK,MessageBoxIcon.Error);
#endif
            return 1;
        }
    }
}

