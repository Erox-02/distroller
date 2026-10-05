---

Name: "Erox"
Project Title: "Distroller" 

---


# Oct 6

Today i got a monitor (thx stardance) and connected it to my laptop right away , but my hyprland.lua didnt had anything to assign  to the moniter , so it just went with the default one , a diff workspace and i had to configure  it a lot then finally got it to work but now i think i will make a rust based cli tool to automate those things . Easy right? for users not me lol .

i made a basic poc script idea for now

```sh
distroller list
distroller use 1 (here 1 means the 1st moniter like for laptop , it is eDP-1, while the other moniters will turn off maybe? define the state in distroller.conf )
distroller use 2 (HDMI-A-1)
distroller use 1,2... (it gives each moniter diff workspace)
distroller use 1,2 --share (it makes both share a workspace but more like a single monitor , instead of treating both diff) 
distroller use 1,2 --copy (show the same thing)
```
i made it a codeblock but the comments "()" arent valid so dont copy these things ,

now imma gonna start building the base

ok cargo init'd , gonna make all the things in js one file now and distribute later Yeah ik thts easier but still i like redistribute later one.

so build the initial struct , btw i used u8 for height and width at first but it doesnt store 1920 or 1080 so i used u16 as it is better than u32 atleast i think so , also i used f32 for the refreash rate as hyprland give things like 59.64 sometimes , wait wtf am i explain? its basics nah i dont needa explain, nah, i'wd build .  