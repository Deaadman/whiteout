using DevWinUI;
using Microsoft.UI.Windowing;
using Microsoft.UI.Xaml;
using TitleBar = Microsoft.UI.Xaml.Controls.TitleBar;

// To learn more about WinUI, the WinUI project structure,
// and more about our project templates, see: http://aka.ms/winui-project-info.

namespace tld_mod_manager
{
    /// <summary>
    /// An empty window that can be used on its own or navigated to within a Frame.
    /// </summary>
    public sealed partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
            ConfigureTitleBar();
            SetNavigationMenuLocalisations();
        }

        #region TitleBar
        private void ConfigureTitleBar()
        {
            this.ExtendsContentIntoTitleBar = true;
            this.AppWindow.TitleBar.PreferredHeightOption = TitleBarHeightOption.Tall;
            this.SetTitleBar(TitleBar);
            TitleBar.Title = "ProjectName".GetLocalizedResource();
        }

        #region Events
        private void TitleBar_OnPaneToggleRequested(TitleBar sender, object args)
        {
            NavigationView.IsPaneOpen = !NavigationView.IsPaneOpen;
        }
        #endregion
        #endregion

        #region Navigation Menu
        private void SetNavigationMenuLocalisations()
        {
            Home.Content = "Home".GetLocalizedResource();
            Mods.Content = "Mods".GetLocalizedResource();
            Downloads.Content = "Downloads".GetLocalizedResource();
            Library.Content = "Library".GetLocalizedResource();
        }
        #endregion
    }
}
