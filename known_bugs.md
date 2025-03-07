**MAJOR**

No major bug for the moment 🙏

**MINOR**

`bug_id: 4b3045`
**Authentification Bug:** Even if you fill in valid credentials, the database sync can be buggy, and authentication may fail. Normally, it works on the second try.

`bug_id: 2eb9e3`
**Display:** The app is not displayed and no error message appears (especially if you change user, quit and restart the app). Solution: quit the terminal and try it again.

`bug_id: 2d358c53`
**Mark as finished:** When a title reach the end, mark as finished not always work.

`bug_id: a49eza`
**cvlc error sync:** If you use other command that `shutdown` to quit `cvlc` it may result of a sync issue.

`bug_id: fe4116`
**cvlc macOS:** `cvlc` option is not available for now in macOS.



**FIXED**  
`bug_id: 9bacac` 
**Sync**: If you open VLC to listen X, close VLC and quickly open VLC again to listen Y: X will still be sync — according to Y (normally, only Y has to be sync in this case). **PARTIALLY FIXED** (because not optimal solution): a delay of 5 seconds had been added before open VLC, to give the time to X session to close).  
`bug_id: 86384e` 
**Sync**: Rarely and especially if you open VLC to listen X, close VLC and quickly open VLC again to listen Y: the progress of X is set to 0 seconds.  
`bug_id: 06e548` 
**Terminal broken**: The terminal is broken after the app is quit.  
`bug_id: 6ac5d8` 
**Data loss if app crash or disgracefully quit**: If app crash, the last session is not closed.  
`bug_id: bf10cd` 
**Launch a new media**: Have to close manually VLC to close and sync a session.  
`bug_id: 3f729c` 
**Loading time**: for now, not optimized for a library with a lot of items (long start loading and refresh time)  
`bug_id: dd9a649`
**Listening Session:** Sometimes, the session (that you can see in `yourserveraddress/audiobookshelf/config/sessions`) does not close correctly, especially if you open VLC, quit it quickly, and start another book.  
`bug_id: e0b61c`
**VLC:** `VLC` continue to run after the app is quit.  
`bug_id: fc695f`
**Listening session:** The session (that you can see in `yourserveraddress/audiobookshelf/config/sessions`) does not close when the app is quit.  
`bug_id: 40f48d`
**Cursor:** When you quit the app, terminal cursor disappear. Solution: type `restet` in your terminal.  
