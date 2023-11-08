# Dungeon Quest
Updated version of funny game which I failed to build from original repo.
In this game you can choose hero & game mode, fight monsters, use abilities (not for all heroes), 
proceed through levels. Gameplay is still too poor to consider this project as a game you may want to play,
but it is good to demonstrate how Bevy Engine works.

I am doing this for learning purposes.

Main change during refactoring was that AppState stack was removed, and now there is single State.
So Pause mechanics was refactored dramatically, same as Rewards screen for survival mode.

Feel free to create issues or PR if you want.

```cargo run``` for build and play.


| Version         | Bevy Version |
|-----------------|--------------|
| 0.2.0 (current) | 0.12         |
| -               | -            |



[Legacy project](https://github.com/inFngNam/dungeon-quest)
