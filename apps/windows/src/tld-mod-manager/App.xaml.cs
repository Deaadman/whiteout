using Microsoft.UI.Xaml;
using Microsoft.Windows.AppLifecycle;

// To learn more about WinUI, the WinUI project structure,
// and more about our project templates, see: http://aka.ms/winui-project-info.

namespace tld_mod_manager
{
    /// <summary>
    /// Provides application-specific behavior to supplement the default Application class.
    /// </summary>
    public partial class App : Application
    {
        public static MainWindow? MainWindow { get; private set; }
        
        private Window? _window;

        /// <summary>
        /// Initializes the singleton application object.  This is the first line of authored code
        /// executed, and as such is the logical equivalent of main() or WinMain().
        /// </summary>
        public App()
        {
            InitializeComponent();
        }

        /// <summary>
        /// Invoked when the application is launched.
        /// </summary>
        /// <param name="args">Details about the launch request and process.</param>
        protected override void OnLaunched(LaunchActivatedEventArgs args)
        {
            _window = new MainWindow();
            _window.Activate();
            
            // Forces the app to be single instanced.
            // https://johngagefaulkner.github.io/01-Build-a-Single-Instance-WinUI-3-App-2026-Gemini-2.5-Pro.html
            var activationArgs = AppInstance.GetCurrent().GetActivatedEventArgs();
            HandleActivation(activationArgs);
        }
        
        public void HandleActivation(AppActivationArguments args)
        {
            MainWindow?.Activate();
        }
    }
}
