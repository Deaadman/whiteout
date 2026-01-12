using Microsoft.UI.Xaml;
using Microsoft.Windows.AppLifecycle;
using System;
using System.Threading;
using System.Threading.Tasks;
using Microsoft.UI.Dispatching;

namespace tld_mod_manager;

/// <summary>
/// To make the app single-instanced only <see href="https://johngagefaulkner.github.io/01-Build-a-Single-Instance-WinUI-3-App-2026-Gemini-2.5-Pro.html"/>. 
/// </summary>
public static class Program
{
    private const string AppInstanceKey = "my-unique-app-key-12345";

#pragma warning disable CS8892
    [STAThread]
    static async Task Main(string[] args)
    {
        WinRT.ComWrappersSupport.InitializeComWrappers();
        
        var activationArgs = AppInstance.GetCurrent().GetActivatedEventArgs();
        var mainInstance = AppInstance.FindOrRegisterForKey(AppInstanceKey);

        if (mainInstance.IsCurrent)
        {
            mainInstance.Activated += OnAppActivated;
            StartApp(activationArgs);
        }
        else
        {
            await mainInstance.RedirectActivationToAsync(activationArgs);
            Environment.Exit(0);
        }
    }
#pragma warning restore CS8892
    
    private static void StartApp(AppActivationArguments args)
    {
        Application.Start(_ =>
        {
            var context = new DispatcherQueueSynchronizationContext(DispatcherQueue.GetForCurrentThread());
            SynchronizationContext.SetSynchronizationContext(context);

            new App();
        });
    }

    private static void OnAppActivated(object? sender, AppActivationArguments args)
    {
        if (App.MainWindow is null)
            return;

        App.MainWindow.DispatcherQueue.TryEnqueue(() =>
        {
            (Application.Current as App)?.HandleActivation(args);
        });
    }
}