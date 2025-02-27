**MAJOR**

`bug_id: 9bacac` 
**Sync**: If you open VLC to listen X, close VLC and quickly open VLC again to listen Y: X will still be sync — according to Y (normally, only Y has to be sync in this case). **PARTIALLY FIXED** (because not optimal solution): a delay of 5 seconds had been added before open VLC, to give the time to X session to close).

**MINOR**

`bug_id: 4b3045`
**Authentification Bug:** Even if you fill in valid credentials, the database sync can be buggy, and authentication may fail. Normally, it works on the second try.

`bug_id: 2eb9e3`
**Display:** The app is not displayed and no error message appears (especially if you change user, quit and restart the app). Solution: quit the terminal and try it again.

`bug_id: 2d358c53`
**Mark as finished:** When a title reach the end, mark as finished not always work.

`bug_id: 40f48d`
**Cursor:** When you quit the app, terminal cursor disappear. Solution: type `restet` in your terminal.

`bug_id: a49eza`
**cvlc error sync:** If you use other command that `shutdown` to quit `cvlc` it may result of a sync issue.

**FIXED**

`bug_id: fixed_dd9a649`
**Listening Session:** Sometimes, the session (that you can see in `yourserveraddress/audiobookshelf/config/sessions`) does not close correctly, especially if you open VLC, quit it quickly, and start another book.  
