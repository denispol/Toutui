const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn changelog() -> String {
    let mut changelog = String::new();

let changelog_01 = format!(
        "Changelog Toutui v0.1.0-beta (02/21/2025) \n\
         Fixed:\n\
         \n\
         First release.
         \n\
         Changed:\n\
         \n\
         First release.
         \n\
         Enjoy!\n
         ####\n"
    );
let changelog_02 = format!(
        "Changelog Toutui v0.1.1-beta (02/24/2025) \n\
         Fixed:\n\
         \n\
         - App crash (out of bounds) when API send empty values.
         - Close listening session not always working (bug_id: fixed_dd9a64)
         \n\
         Changed:\n\
         \n\
         No change.
         \n\
         Enjoy and be toutui!\n
         ####\n",
    );
let changelog_03 = format!(
        "Changelog Toutui v0.1.2-beta (02/24/2025) \n\
         Fixed:\n\
         \n\
         - Partially fixed, becsause not optimal: bug_id: 9bacac Sync: If you open VLC to listen X, close VLC and quickly open VLC again to listen Y: X will still be sync — according to Y (normally, only Y has to be sync in this case).

         \n\
         Changed:\n\
         \n\
         No change.
         \n\
         Enjoy and be toutui!\n
         ####\n",
    );
let changelog_04 = format!(
        "Changelog Toutui v0.1.3-beta (02/03/2025) \n\
         Fixed:\n\
         \n\
         - Fix bug_id: 3f729c Loading time not optimized for library with a lot of items (long start loading and refresh time)
         \n\
         Changed:\n\
         \n\
         - Script `hello_toutui` to make installation easier.
         \n\
         Contributors:\n\
         \n\
         - dougy147, dhonus
         \n\
         Enjoy and be toutui!\n
         ####\n",
);
let changelog_05 = format!(
    "Changelog Toutui v0.2.0-beta (07/03/2025) \n\
CAUTION: This version is not compatible with the previous one.  
You need to remove the database in ~/.config/toutui before proceeding. 
         Fixed:\n\
         \n\
         - From known_bugs.md, fixed:

    Find a robust solution for bug_id: 9bacac
    Fix bug_id: 86384e
    Fix bug_id: 6ac5d8
    Fix bug_id: 06e548
    Fix bug_id: e0b61c
    Fix bug_id: fc695f
    Fix bug_id: 40f48d
    Fix bug_id: bf10cd

         \n\
         Changed:\n\
         \n\
         - 
         \n\
         Contributors:\n\
         \n\
         - AlbanDAVID
         \n\
         Enjoy and be toutui!\n
         ####\n",
);
let changelog_06 = format!(
    "Changelog Toutui v0.3.0-beta (24/03/2025) \n\
CAUTION: This version is not compatible with the previous one.  
To make it work properly, perform a fresh reinstall.
\n\
         Added:\n\
         - Integrated player. Keep calm and stay in your terminal! :)
         \n\
         Fixed:\n\
         \n\
         - Fixed: issue where pressing R twice was required to refresh the app.
         - Fixed: issue causing the cursor to disappear when the application is closed.
         - Fixed: issue if app is quitted for the first time and that listening session is empty.
         \n\
         Changed:\n\
         \n\
         - Faster loading time to play an item.
         - Improved synchronization accurary.
         - Removed warning during compilation time.
         \n\
         Contributors:\n\
         \n\
         - AlbanDAVID, dougy147
         \n\
         Enjoy and be toutui!\n
         ####\n",
);
let changelog_07 = format!(
    "Changelog Toutui v0.3.1-beta (25/03/2025) \n\
CAUTION: This version is not compatible with v0.2.0-beta and bellow.  
To make it work properly, perform a fresh reinstall.
\n\
         Fixed:\n\
         \n\
         - Fixed: incorrect merge
         \n\
         Contributors:\n\
         \n\
         - AlbanDAVID
         \n\
         Enjoy and be toutui!\n
         ####\n",
);
let changelog_08 = format!(
    "Changelog Toutui v{} (26/03/2025) \n\
         Added:\n\
         \n\
         - macOS compatibility.
         \n\
         Fixed:\n\
         \n\
         - Issue with VLC buffer (if a chapter is manually changed or during jump/backward).
         - Display issue on small monitors.
         \n\
         Changed:\n\
         \n\
         - hello_toutui script improved
         \n\
         Contributors:\n\
         \n\
         - AlbanDAVID, dougy147
         \n\
         Enjoy and be toutui!\n
         ####\n",
         VERSION
);

    changelog.push_str(&changelog_08); 
    changelog.push_str(&changelog_07); 
    changelog.push_str(&changelog_06); 
    changelog.push_str(&changelog_05); 
    changelog.push_str(&changelog_04); 
    changelog.push_str(&changelog_03); 
    changelog.push_str(&changelog_02); 
    changelog.push_str(&changelog_01); 


changelog
}
