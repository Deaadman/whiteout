using DevWinUI;
using Microsoft.UI.Windowing;
using WinUIEx;

namespace perseverance;

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