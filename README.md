# termordle


Wordle Applet in terminal. Heavy inspiration from [Wordlet](https://github.com/scottluptowski/wordlet); if you want a higher quality applet, I suggest you check this out.


## How to run

If you are on a **linux or macOS** operated machine, just download rust by running this in terminal
(btw, do not copy the dollar signs)
```
$ curl https://sh.rustup.rs -sSf | sh
```
If you are on a **windows** operated machine, go to this [installer](https://win.rustup.rs/)

Afterwards, download the source as a .zip and unzip the files. I'd suggest you put the files into a new, decluttered folder you can easily access.


Head to that folder by running
```
$ cd [YOUR FOLDER NAME]
```
This folder would contain all the source files. After that, just run 
```
$ cargo build
```
and then
```
$ cargo run
```
in order to initialize the applet!

Alternatively, run:
```
$ git clone https://github.com/RedFlame2112/termordle.git
$ cd termordle
$ cargo build
$ cargo run
```

Hope you enjoy playing wordle ;)

also, I'd heavily suggest not looking at the `solutions.txt` text file while playing. That'd be cheating after all! (;

Have fun.

## How to play
After running, just type in letters for your word, just like classic wordle. 
On the attempt you are on, use `backspace` in order to re-enter a different word
**Correct** characters are in purple
**Partially Correct** characters are in cyan
**Missed** characters are in grey/are not highlighted
Press enter to log the attempt
if you are correct within 6 tries, EPIC gamer
if you are incorrect, welp. Better luck next time! The solution is given then. Add the word to your personal dictionary ;)

GLHF!

### Example game:
![example](https://media.discordapp.net/attachments/819417070185480202/956218088888602644/unknown.png)
