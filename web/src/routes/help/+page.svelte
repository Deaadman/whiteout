<script>
    import * as Accordion from "$lib/components/ui/accordion/index.js";
    import * as Card from "$lib/components/ui/card/index.js";
</script>

<div class="grid grid-cols-2 gap-2 py-2">
    <Card.Root>
        <Card.Content>
            <h2 class="scroll-m-20 mb-4 text-3xl font-semibold tracking-tight transition-colors first:mt-0">Installing Mods Guide</h2>
            <p class="mb-4">Mods are only compatible / supported with Steam versions of the game.</p>
            <p class="mb-4">This website is in no way affiliated with Hinterland Games and only aims to provide easy access to published mods for the game.</p>
            <ol class="my-6 ms-6 list-decimal [&>li]:mt-2">
                <li>
                    If you haven't turned off sharing crash reports and analytics, launch TLD and turn these off first.
                    <br/>
                    It's important! Let's not bug Hinterland with issues unless you're certain it's their fault.
                </li>
                <li>
                    Make sure to backup your saves before modding, mods are usually tested, but bad stuff happens sometimes.
                    <br/>
                    You can find your saves in C:\Users\[your-windows-user]\AppData\Local\Hinterland\TheLongDark.
                </li>
                <li>
                    First of all, make sure you have the Melon Loader requirements installed. For current version of TLD these are:
                    <ul class="ms-6 list-disc [&>li]:mt-2">
                        <li>
                            Microsoft Visual C++ 2015-2019 Redistributable 64 Bit
                        </li>
                        <li>
                            .NET 6.0 Runtime.
                        </li>
                    </ul>
                </li>
                <li>
                    Next, we need MelonLoader:
                    <ul class="ms-6 list-disc [&>li]:mt-2">
                        <li>
                            Download MelonLoader here
                        </li>
                        <li>
                            Run MelonLoader.
                        </li>
                        <li>
                            Select TheLongDark from the detected game list
                            <br/>
                            (or Add Game Manually if not detected)
                        </li>
                        <li>
                            Make sure to install the 0.7.2 version by selecting the version from the list
                            <br/>
                            (Make sure to tick "Enable Nightly Builds" and pick the latest)
                        </li>
                        <li>
                            Wait for the download and installation to complete!
                        </li>
                    </ul>
                </li>
                <li>
                    Download any mods you want using the  icon, (extract the zip if packaged as one) and place the .dll/.modcomponent files inside the Mods folder that MelonLoader created in your game directory
                    (Make sure to read any instructions for the mod, some contain multiple versions zipped up)
                </li>
                <li>
                    Make sure you've also installed any dependency the mod might need
                </li>
                <li>
                    Launch TLD and enjoy your modded game :)
                </li>
            </ol>
        </Card.Content>
    </Card.Root>
    <Card.Root class="col-start-2">
        <Card.Content>
            <h2 class="scroll-m-20 mb-4 text-3xl font-semibold tracking-tight transition-colors first:mt-0">Frequently Asked Questions</h2>
            <Accordion.Root type="multiple">
                <Accordion.Item value="item-1">
                    <Accordion.Trigger>MelonLoader errors with 'IL2CPP-init detour failed: one of the dependent libraries is missing'</Accordion.Trigger>
                    <Accordion.Content>
                        You need to install .NET 6 Runtime.
                    </Accordion.Content>
                </Accordion.Item>
                <Accordion.Item value="item-2">
                    <Accordion.Trigger>MelonLoader shows something about missing UnhollowerBaseLib in the console</Accordion.Trigger>
                    <Accordion.Content>
                        The mod you have installed isn't updated to the current version of The Long Dark, remove it from your mods folder.
                    </Accordion.Content>
                </Accordion.Item>
                <Accordion.Item value="item-3">
                    <Accordion.Trigger>The game crashes when opening it</Accordion.Trigger>
                    <Accordion.Content>
                        Make sure you have installed version ... of MelonLoader.
                    </Accordion.Content>
                </Accordion.Item>
                <Accordion.Item value="item-4">
                    <Accordion.Trigger>I have the game on the Epic Games Launcher, I have installed MelonLoader but it keeps crashing</Accordion.Trigger>
                    <Accordion.Content>
                        Epic Games Launcher has an anti-tamper system that prevents MelonLoader from loading, so currently - it isn't possible to be modded at this time.
                    </Accordion.Content>
                </Accordion.Item>
                <Accordion.Item value="item-5">
                    <Accordion.Trigger>I want to uninstall MelonLoader</Accordion.Trigger>
                    <Accordion.Content>
                        Run the installer again and select the tld.exe, the install button should change into an uninstall one.
                        <br/>
                        Alternatively, you can delete MelonLoader, Mods, Plugins, UserData and UserLibs folders and version.dll files from your game's folder.
                    </Accordion.Content>
                </Accordion.Item>
                <Accordion.Item value="item-6">
                    <Accordion.Trigger>Do mods work on Wintermute (Story Mode)?</Accordion.Trigger>
                    <Accordion.Content>
                        Currently, not. Survival and Story have been separated and are two different games now, in theory, installing a different MelonLoader with its own mods to story should work, but there seems to be some problems with it loading.
                    </Accordion.Content>
                </Accordion.Item>
            </Accordion.Root>
        </Card.Content>
    </Card.Root>
</div>