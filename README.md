# mcw

Manage commands in multiple repos at once. And also lets you select which on 
the fly with the tui.



### Installation

Go to the release page and download the latest executable and make sure it's
available in PATH


## Support commands

### exec: Execute code in repositories


simple usage: ``mcw exec git status`` for simple commands   
advanced usage: ``mcw exec -- git commit -m "Bing bong"`` for commands with embedded  flags

exec commands takes over current terminal so vim and such should work.


### version: Shows the current version of the mcw
simple usage: ``mcw version`` 


### gitlog : Shows the latest 5 commits of the selected repos
simple usage: ``mcw gitlog`` 


