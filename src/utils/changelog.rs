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
        "Changelog Toutui v{} (02/03/2025) \n\
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
         VERSION
    );

    changelog.push_str(&changelog_04); 
    changelog.push_str(&changelog_03); 
    changelog.push_str(&changelog_02); 
    changelog.push_str(&changelog_01); 


changelog
}
