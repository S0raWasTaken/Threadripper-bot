<p>
  <img src="./threadripper-bot.png" width="100" align="left">
  </br>
</p>

# Threadripper-bot
A Discord bot for control and decentralization of thread management


## About
Threadripper aims to serve some extra utilities to take better care of thread channels.<br>
Discord lacks a lot in functionality for threads, so that's why I made this bot

## Goals and TODOs
- Actually get a VPS to host this bot ✓ (thanks to Github + DigitalOcean)
- Implement thread owner system
  - Full thread control to owner (messages, members etc)
- Mass thread actions (channel & guild)
  - Mass delete, archive, clean...
- TMC management ✓

## Concepts
### Thread owner
> A thread owner is a member that created a thread or unarchived it.<br>
> It can delete messages inside its own thread channel, remove members from it and do mass actions in his own thread. 

### TMC (Threadded media channel)
> A TMC is an invented concept that describes a pictures/videos/etc channel that members are not allowed to chat.<br>
> A normal TMC will delete messages from everyone that doesn't include an attachment or the bot's prefix/mention in its message.<br>
> It will also automatically create a thread for each message that contains an attachment and set their owners accordingly.

### Thread inheritance
> Thread inheritance happens when an archived thread is unarchived by a server member.<br>
> The member that unarchived the thread will inherit it from its previous owner.<br>
> The old owner will be able to take its ownance back if it requests for it in less than 48h.<br>
> The original owner is always able to take the ownance back.

## Help
### Commands
Every command (except ping) has a `-h, --help` flag that can be used to gather information about the usage and options it has.
The commands are built in a UNIX-like way, as if they were individual CLI programs.
Here's a list of commands:
```toml
[General]
ping
prefix
help (wip)

[ThreadManagement]
set_media_channel (setmedia, smc...)
remove_media_channel (rmmedia, rmc...)

# More commands will be added on the future
```
### Unusual Behaviour
You can always make an issue [here](https://github.com/S0raWasTaken/Threadripper-bot/issues) to report this kind of behaviour.<br>
If you do prefer, you can message me on Matrix or send me an email. Contact information is listed in [my profile](https://github.com/S0raWasTaken)

### Adding Threadripper to your server
You can add it to your server by clicking [here](https://discord.com/oauth2/authorize?client_id=907572233835257876&scope=bot&permissions=67234840).<br><br>
Threadripper doesn't actually need administrator permissions, but it's better to have it on than losing your mind trying to fix permission issues.
Since it's open source, why not? If you don't trust me, you can deploy your own Threadripper!

## Contributing
Issues and Pull Requests are always welcome. I also do accept ideas in the [Issues](https://github.com/S0raWasTaken/Threadripper-bot/issues) page

## Nerdy curiosities
- Rustbreak is included in the project, as a `Client::data`+`Context::data` file database manager, so when the bot turns off, it does save stuff
- Clap is included as the command and arguments handler, this is why the commands' syntaxes are so similar to UNIX commands, I'm literaly using a crate that builds most of CLI apps in Rust
- As you might expect, Threadripper's name comes from the AMD threadripper series, because I do like AMD processors and GPUs, since they play so nice on Linux
- Serenity's documentation sucks.
- `cargo build` generates a binary of size >150MB, `cargo build --release`'s bin size is 9MB and `make release` reduces the binary size to <2MB
