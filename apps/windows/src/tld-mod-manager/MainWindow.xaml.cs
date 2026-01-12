using DevWinUI;
using Microsoft.UI.Windowing;
using WinUIEx;

// To learn more about WinUI, the WinUI project structure,
// and more about our project templates, see: http://aka.ms/winui-project-info.

namespace tld_mod_manager
{
    /// <summary>
    /// An empty window that can be used on its own or navigated to within a Frame.
    /// </summary>
    public sealed partial class MainWindow : WindowEx
    {
        public MainWindow()
        {
            InitializeComponent();
            ConfigureTitleBar();
            SetNavigationMenuLocalisations();
            
            this.CenterOnScreen();
        }

        #region TitleBar
        private void ConfigureTitleBar()
        {
            this.ExtendsContentIntoTitleBar = true;
            this.AppWindow.TitleBar.PreferredHeightOption = TitleBarHeightOption.Tall;
            this.SetTitleBar(TitleBar);
            TitleBar.Title = "ProjectName".GetLocalizedResource();
        }
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
