# Holes the Movie the Game

![Holes](holes.jpg "Holes the Movie the Game")

## Resources and background

* This project is inspired by Taito's hit 1983 game [Ice Cold Beer](https://en.wikipedia.org/wiki/Ice_Cold_Beer)
* It builds off the initial work done on this [arduino project](https://github.com/mcataford/lukewarmbeer)
* This repo is generated from the [Cortex M Quickstart repo](https://github.com/rust-embedded/cortex-m-quickstart)
* Embedded rust book https://docs.rust-embedded.org/book/start/
* Embedded rust discovery book https://docs.rust-embedded.org/discovery/f3discovery

## Setup

Follow the setup instructions found [here](https://docs.rust-embedded.org/discovery/f3discovery/03-setup/index.html)

## Developing

Run ITM dump:
```
cd /tmp
touch itm.txt
itmdump -F -f itm.txt
```

Run openOCD
```
cd /tmp
openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
```

Run this project
```
cargo run
```

## Bill of Materials

TODO

# License

[MIT](LICENSE)
