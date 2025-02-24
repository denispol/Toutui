**MAJOR**

`bug_id: 9bacac` 
**Sync** If you open VLC and listening X, close VLC and and quickly open VLC to listen Y: X and Y will be both sync (normally, only Y has to be sync in this case).

**MINOR**

`bug_id: 4b30`
**Authentification Bug:** Even if you fill in valid credentials, the database sync can be buggy, and authentication may fail. Normally, it works on the second try.

`bug_id: 2d358c53`
**Mark as finished:** When a title reach the end, mark as finished not always work.

`bug_id: 40f48`
**Cursor:** When you quit the app, terminal cursor disappear.

`bug_id: a49e`
**cvlc error sync:** If you use other command that `shutdown` to quit `cvlc` it may result of a sync issue.

**FIXED**

`bug_id: fixed_dd9a649`
**Listening Session:** Sometimes, the session (that you can see in `yourserveraddress/audiobookshelf/config/sessions`) does not close correctly, especially if you open VLC, quit it quickly, and start another book.  
