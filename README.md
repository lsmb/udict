<div align="center">
<h1>udict - Urban Dictionary CLI</h1>
</div>

<p align=center>
A CLI tool for fetching Urban Dictionary definitions.
</p>

<p align=center>
  <img src=https://user-images.githubusercontent.com/69675545/164534602-3a0be421-42c5-47f9-a783-1ee040f8b522.gif>
</p>

# Installation

### cargo
`cargo install udict`

### Arch Linux
Using an AUR package manager, for example [paru](https://github.com/Morganamilo/paru) or [yay](https://github.com/Jguer/yay)
`paru -S udict-bin`

# Usage

`udict <term>` -- Search the term from urban dictionary, can include spaces

`udict hello world`

`udict [n] <term>` -- Show the `n` number of definitions

`udict 4 hello world`
