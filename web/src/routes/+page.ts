import type { PageLoad } from './$types';

interface Mod {
    DisplayName: String,
    Name: String,
    Type: String,
    Author: String,
    DisplayAuthor: String[],
    Description: String,
    Aliases: String[],
    Replaces: String[],
    ModUrl: String,
    RepoName: String,
    Dependencies: String[],
    Version: String,
    Error: Boolean,
    AutoUpdate: Boolean,
    Download: String,
    Downloads: String[],
    AuthorUrl: String,
    SupportUrl: String,
    Categories: String[],
    TestedOn: TestedOn,
    Released: String,
    Updated: String,
    Images: String[],
    Status: Status,
    PreviousAuthors: String,
    Source: String,
    GameVersion: String[]
}

interface TestedOn {
    tld: String,
    ml: String
}

interface Status {
    Working: Boolean,
    Beta: Boolean,
    PatchNotes: String,
    Notes: String,
    Issues: String
}

const apiUrl = '/tldmods/api.php?details&pp';

export const load: PageLoad = async ({ fetch }) => {
    const res = await fetch(apiUrl);
    if (!res.ok) throw new Error(`Failed to fetch: ${res.status}`);

    const mods = await res.json();
    return { mods };
};