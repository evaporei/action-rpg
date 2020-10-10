# action-rpg

This is the translation of a Godot tutorial that uses GDScript to using GDNative with Rust.

The code is not finished, but most features are implemented (attack, movement, rolling, enemies, hitboxes, collision).

Also, there are some optimizations I still need to do on the code, just caching some find on Nodes, so they don't get called every frame, which is kinda expensive (not much given the size of the project, still its good to fix later).

Tutorial YouTube playlist: https://www.youtube.com/playlist?list=PL9FzW-m48fn2SlrW0KoLT4n5egNdX-W9a

Code using GDScript: https://github.com/uheartbeast/youtube-tutorials/tree/master/Action%20RPG

How the game looks like:

![Screenshot from 2020-10-10 09-18-35](https://user-images.githubusercontent.com/15306309/95654913-d739c300-0ad9-11eb-8b62-495ef19990c9.png)
