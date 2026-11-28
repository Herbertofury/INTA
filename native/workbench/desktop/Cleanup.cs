using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Data;
using System.Windows.Automation;

namespace Inta.Desktop
{
    internal sealed partial class Workspace
    {
        private sealed class CleanupFile
        {
            public string Id { get; set; }
            public string Path { get; set; }
            public string Rule { get; set; }
            public string State { get; set; }
            public string Sha256 { get; set; }
            public string Bytes { get; set; }
            public string Detail { get; set; }
        }

        private static bool ConfirmCleanup(Window owner, string verb, CleanupFile[] files)
        {
            var dialog = new Window { Title = verb + " selected files", Owner = owner, Width = 710, Height = 390, WindowStartupLocation = WindowStartupLocation.CenterOwner };
            var panel = new DockPanel { Margin = new Thickness(15) };
            var description = new TextBlock { Text = verb == "Quarantine" ? "Move these exact files into this review's same-volume quarantine. INTA checks their identities and hashes again." : "Move these exact files back to their recorded paths. Existing destination files are preserved.", TextWrapping = TextWrapping.Wrap, Margin = new Thickness(0,0,0,10) };
            DockPanel.SetDock(description, Dock.Top); panel.Children.Add(description);
            var buttons = new StackPanel { Orientation=Orientation.Horizontal, HorizontalAlignment=HorizontalAlignment.Right, Margin=new Thickness(0,10,0,0) };
            var cancel = new Button { Content="Cancel", IsCancel=true, MinWidth=75 };
            var ok = new Button { Content=verb + " " + files.Length + " selected", Margin=new Thickness(8,0,0,0), Padding=new Thickness(8,3,8,3) };
            ok.Click += delegate { dialog.DialogResult=true; };
            buttons.Children.Add(cancel);buttons.Children.Add(ok);DockPanel.SetDock(buttons,Dock.Bottom);panel.Children.Add(buttons);
            var entries=new ListBox();foreach(var f in files) entries.Items.Add(f.Path+"\nSHA-256 "+f.Sha256);
            panel.Children.Add(entries);dialog.Content=panel;return dialog.ShowDialog()==true;
        }

        private void OpenCleanupReview()
        {
            var panel = new DockPanel { Margin=new Thickness(14) };
            var dialog = new Window { Title="Review cache files", Owner=window, Width=1000,Height=560,MinWidth=680,MinHeight=370,WindowStartupLocation=WindowStartupLocation.CenterOwner,Content=panel };
            var bar = new StackPanel { Orientation=Orientation.Horizontal, Margin=new Thickness(0,0,0,8) };
            var folder = new Button { Content="Review a folder…",Padding=new Thickness(8,3,8,3) };
            var reviews = new ComboBox { MinWidth=240,MaxWidth=660,Margin=new Thickness(10,0,0,0),DisplayMemberPath="Value" };
            AutomationProperties.SetName(reviews,"Saved cleanup reviews");
            bar.Children.Add(folder);bar.Children.Add(reviews);DockPanel.SetDock(bar,Dock.Top);panel.Children.Add(bar);
            var scope=new TextBlock { Text="Eligible files include Python bytecode with its source, pytest state and hash-named temporary files in marked cache directories. Protected folders, open files, cloud placeholders and links are excluded.", TextWrapping=TextWrapping.Wrap, Margin=new Thickness(0,0,0,8) };
            DockPanel.SetDock(scope,Dock.Top);panel.Children.Add(scope);
            var status = new TextBlock { TextWrapping=TextWrapping.Wrap,Margin=new Thickness(0,5,0,0) };
            AutomationProperties.SetName(status,"Cleanup result");DockPanel.SetDock(status,Dock.Bottom);panel.Children.Add(status);
            var bottom=new StackPanel { Orientation=Orientation.Horizontal,Margin=new Thickness(0,7,0,0) };
            var quarantine=new Button { Content="Quarantine selected…",Padding=new Thickness(8,3,8,3), IsEnabled=false };
            var restore=new Button { Content="Restore selected…",Padding=new Thickness(8,3,8,3),Margin=new Thickness(8,0,0,0), IsEnabled=false };
            var audit=new Button { Content="File activity…",Padding=new Thickness(8,3,8,3),Margin=new Thickness(8,0,0,0),IsEnabled=false };
            bottom.Children.Add(quarantine);bottom.Children.Add(restore);bottom.Children.Add(audit);DockPanel.SetDock(bottom,Dock.Bottom);panel.Children.Add(bottom);
            var grid=new DataGrid { AutoGenerateColumns=false,IsReadOnly=true,CanUserAddRows=false,SelectionMode=DataGridSelectionMode.Extended,SelectionUnit=DataGridSelectionUnit.FullRow };
            AutomationProperties.SetName(grid,"Exact cleanup candidates");
            foreach(string column in new[]{"Path","Rule","State","Bytes","Sha256","Detail"})
                grid.Columns.Add(new DataGridTextColumn { Header=column=="Sha256"?"SHA-256":column, Binding=new Binding(column), Width=column=="Path"?new DataGridLength(2,DataGridLengthUnitType.Star):DataGridLength.Auto,MaxWidth=column=="Sha256"?210:520 });
            panel.Children.Add(grid);
            string current=null;bool busy=false,loading=false;
            Action buttons=delegate {
                var set=grid.SelectedItems.Cast<CleanupFile>().ToArray();
                quarantine.IsEnabled=!busy && set.Length>0 && set.Length<=32 && set.All(f=>f.State=="planned"||f.State=="restored");
                restore.IsEnabled=!busy && set.Length>0 && set.Length<=32 && set.All(f=>f.State=="quarantined");
                folder.IsEnabled=!busy;reviews.IsEnabled=!busy;grid.IsEnabled=!busy;
                audit.IsEnabled=!busy&&current!=null;
            };
            Action<Dictionary<string,object>> render=delegate(Dictionary<string,object> review) {
                current=Json.Text(review,"id");
                var files=Json.Array(Json.Get(review,"files")).Select(Json.Object).Select(f=>new CleanupFile { Id=Json.Text(f,"id"),Path=Json.Text(f,"path"),Rule=Json.Text(f,"rule"),State=Json.Text(f,"state"),Sha256=Json.Text(f,"sha256"),Bytes=Json.Text(f,"bytes"),Detail=Json.Text(f,"detail") }).ToList();
                grid.ItemsSource=files;
                status.Text=files.Count+" exact candidates · "+Json.Text(review,"root")+(Json.Text(review,"bounded")=="True"?" · Review limit reached":"");
                buttons();
            };
            Func<Task> history=async delegate {
                var data=Json.Array(await core.Read(new[]{"cleanup","reviews"},CancellationToken.None)).Select(Json.Object).ToList();
                loading=true;
                try { reviews.ItemsSource=data.Select(d=>new KeyValuePair<string,string>(Json.Text(d,"id"),Json.Text(d,"created")+"  "+Json.Text(d,"root"))).ToList(); reviews.SelectedValuePath="Key";if(current!=null)reviews.SelectedValue=current; }
                finally {loading=false;}
            };
            Func<Func<Task>,Task> work=async delegate(Func<Task> operation) {
                if(busy)return;busy=true;buttons();status.Text="Checking the selected files…";
                try{await operation();}catch(Exception e){status.Text=e.GetBaseException().Message;}finally{busy=false;buttons();}
            };
            folder.Click+=async delegate{string path=ChooseFolder("Choose a folder to review for rebuildable cache files.");if(path!=null)await work(async delegate { render(Json.Object(await core.Read(new[]{"cleanup","plan",path},CancellationToken.None))); await history(); });};
            reviews.SelectionChanged+=async delegate{if(loading||busy||reviews.SelectedValue==null)return;string id=Convert.ToString(reviews.SelectedValue);await work(async delegate{render(Json.Object(await core.Read(new[]{"cleanup","review",id},CancellationToken.None)));});};
            grid.SelectionChanged+=delegate{buttons();};
            audit.Click+=async delegate {
                if(current==null)return;
                await work(async delegate {
                    var lines=Json.Array(await core.Read(new[]{"cleanup","audit",current},CancellationToken.None)).Select(Json.Object).Select(e=>Json.Text(e,"at")+"  "+Json.Text(e,"state")+"\n"+Json.Text(e,"path")+"\n"+Json.Text(e,"detail")).ToList();
                    if(lines.Count==0)lines.Add("This review has no file moves.");
                    var activity=new Window { Owner=dialog,Title="Cleanup file activity",Width=740,Height=350,WindowStartupLocation=WindowStartupLocation.CenterOwner,Content=new ListBox { ItemsSource=lines,Margin=new Thickness(10) } };
                    activity.ShowDialog();status.Text=lines.Count+" activity entries shown.";
                });
            };
            Func<string,Task> move=async delegate(string verb){
                var files=grid.SelectedItems.Cast<CleanupFile>().ToArray();
                if(files.Length==0||files.Length>32||current==null||!ConfirmCleanup(dialog,verb,files))return;
                await work(async delegate{var result=Json.Object(await core.Read(new[]{"cleanup",verb.ToLowerInvariant(),current,String.Join(",",files.Select(f=>f.Id)),"--approve"},CancellationToken.None));render(result);status.Text=Json.Text(result,"result")+" · "+status.Text;});
            };
            quarantine.Click+=async delegate{await move("Quarantine");}; restore.Click+=async delegate{await move("Restore");};
            dialog.Closing+=delegate(object sender,System.ComponentModel.CancelEventArgs e){if(busy)e.Cancel=true;};
            dialog.Loaded+=async delegate{await work(async delegate{await history();status.Text="Choose a folder to plan a review or open a saved review. Use Ctrl or Shift to select exact files.";});};
            dialog.ShowDialog();
        }
    }
}
