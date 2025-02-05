# Space Frontiers pre-alpha
<img src="/data/project/sflogo.png?raw=true" data-canonical-src="/data/project/sflogo.png?raw=true" width="175" height="175"/>

![Continuous integration](https://github.com/starwolves/space/actions/workflows/rust.yml/badge.svg?branch=0.0.3-snap)

![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg) ![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)

![Screenshot of Space Frontiers gameplay](/data/project/sfss.png?raw=true)

## Description

  

A modular & moddable multi-threaded sci-fi headless community game server written in Rust with the [Bevy ECS](https://bevyengine.org/) game engine. Made to communicate exclusively with the official moddable Space Frontiers client which is being built with the [Godot Engine](https://godotengine.org/) and [godot-rust](https://github.com/godot-rust/godot-rust).
This game server is designed to run well on modern processors that have multiple CPU cores.

### Media
You can see gameplay videos of Space Frontiers on [YouTube, subscribe](https://www.youtube.com/channel/UC6D7lcx9eL_ChA7HzzvhLtA)!

### Documentation
There is also a work-in-progress [documentation](https://sf-docs.starwolves.io) available for code contributors or modders.

### Discord
[Official Space Frontiers Discord server.](https://discord.gg/Hdxz9h33E7)

### Features (All Moddable & Modular)
* Decentralized gameplay, each community can host their own servers. 👑
* Parallelized ECS (Entity Component System) architecture. 📡
* Pure Rust. No garbage collection & high parallel game logic execution speeds. 🌟
* Data-oriented, everything is its own entity with components within a thread-safe and strictly compiled environment. It is easy to add and remove entities, systems, components, map cells and more simply by managing [plugins](https://bevyengine.org/learn/book/getting-started/plugins/) that will get compiled with the project. 🔭
* Using the cutting-edge [Rapier 3D Physics engine](https://rapier.rs/). 🚀
* Character meshes and animations are fully integrated with [Mixamo](https://www.mixamo.com/) for rigging. ☄
* Inventory system, pick up, wear, attach, place and equip items with character entities.
* Melee & projectile combat systems, damage player, ship walls or other entities with various types of damage and the ability to target specific body parts.
* Advanced bbcode chat, with support for examining entities, modular (radio) channels and proximity communication.
* Actions and tab menu's to easily interact with the world while also offering protection against cheaters.
* Configurable console commands, including rcon admin commands.
* Clients can load in custom content on a per server basis thanks to a traditional content folder approach. Allowing modders to create new entities such as items, characters, sounds, ship cells and more.
* Godot Addressable references are used for efficient and dynamic netcode that works well with custom content.
* Cell based map support including a GUI editor with support for sizes up to 1km by 1km with 100k+ dynamic (de)constructable ship cells as map size is currently bottlenecked by the FOV algorithm. 
* Atmospherics simulation including temperature, pressure, diffusion, gravity and the vacuum of space.
![Screenshot of Space Frontiers atmospherics simulation](/data/project/sfatmosss.png?raw=true)

## Getting Started

### Dependencies



* [Rust](https://www.rust-lang.org/)

  

  

### Executing game server

  

To compile and run the game server:
* Select latest branch from this repository and download that code.
* In your terminal navigate to the project folder you have just obtained and run:

```
cargo run
```

### Space Frontiers client
You can get the latest stable releases of the closed-source client on [Discord](https://discord.gg/yYpMun9CTT).
Ensure your server has the right git branch with the same version as the obtained client and not the master branch!

The client is built on top of the latest stable Godot 3.4.x release. This also means that there are graphical artifacts present on certain hardware. The client is relatively demanding of hardware it runs on due to the limited dynamic lighting rendering performance of Godot 3.
However, most devices made for video-games should expect no such problems.

When Godot 4 is stable enough, the client will be upgraded and moved to Godot 4 for better 3D rendering in favour of the Vulkan API  which aims to resolve the aforementioned issues.

## More information

### Overview
Space Frontiers has an official community [Discord server](https://discord.gg/yYpMun9CTT).
This project is oriented towards long-term development, meaning it is here to stay and to be developed for years to come.
Feedback, bug reports, suggestions and critique are very much appreciated. Github issues will be reviewed and considered.

### Contributing
The hopes are to financially reward and/or hire the most suitable people for their contributions in the much further future.
It is possible to contribute in all kinds of ways and you reaching out for possibilities will be appreciated!
Also looking for both 2D & 3D digital artists, concept artists, writers and game(play) designers.
People who are genuinely interested in contributing are suggested to contact the developers through Discord, when this interest arises high priority will be put into tutorial videos and releasing the GUI tools of the project for custom map and custom entity creation.

All contributors of this project have to agree to our [Collaberative License Agreement](https://github.com/starwolves/contributor-license-agreement/blob/main/CLA). Our automated CLA assisstant will give you instructions on how to agree the first time you contribute on Github.

When contributing please do not work on the `master` branch, but work on the snapshot branch (named `*-snap`). Create pull requests when you would like us to review and merge your code.

Space Frontiers allows each community of players to compile and host a server instance themselves meaning the gameplay, community moderation & servers remain decentralized and sovereign.

A developer web forum is planned.
![Screenshot of Space Frontiers GUI project map and content editor](/data/project/sfeditorss.png?raw=true)

### License

This repository is licensed under a [special license](https://github.com/starwolves/space/blob/master/LICENSE).


### [StarWolves.io](https://starwolves.io)
Star Wolves is a gaming community that is pioneering the game Space Frontiers by hosting official servers for it and more.
The roots of Space Frontiers are partially in this community, but Space Frontiers is managed in a way that effectively makes Space Frontiers and its community remain a predominantly separated entity; apart from the fact that Star Wolves technically owns Space Frontiers.
Star Wolves will be one of the first communities to host a 24/7 server for Space Frontiers.
