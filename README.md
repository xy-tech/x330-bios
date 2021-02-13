# X230 & X330 (modified display) BIOS
## Content

**Stock BIOS**
* Stock BIOS dumps
* BIOS patches & relevant sources
* Guides on patching BIOS

**Coreboot**
* Relevant coreboot files
* Scripts to simplify compilation for first-time users
* VBT modification & explanations

## Recommended guide
* [Has important information on how to get started](https://www.chucknemeth.com/laptop/lenovo-x230/flash-lenovo-x230-coreboot#prepare-coreboot)
* [My website]()

## A short intro of things
* The X230 has 2 BIOS EEPROM (storage) SPI chips concatenanted (joined) together.
* To external flash any image, the image has to be broken up into 2 parts and then flashed separately. 
* A full BIOS image actually contains quite a few things: the management engine (ME), flash descriptor (FD), the actual BIOS ROM etc.
* The stock image stores the actual BIOS portion in the top chip only. 
* Limiting coreboot (BIOS) to 0x400000 (4MB) means the bottom chip (8MB) is untouched between the stock ROM & Coreboot. This means only the top chip (4MB) needs to be flashed.


## How to flash images (for Windows)
Follow the instructions in the Linux section but do it under Windows subsystem for Linux instead.

## How to flash binary images (for Linux)
1. Install [flashrom](https://www.flashrom.org/Flashrom)
1. For internal flashing, run this command: `<flashrom -p internal -w image.rom>`

# License
GPLv3 for all scripts etc. Proprietary license for Lenovo stuff. Pls don't sue me. All other 

# Thanks
* Alexander Couzens [X330 patch](https://review.coreboot.org/c/coreboot/+/28950) & [VBT patch](https://code.fe80.eu/lynxis/vbtparse)
* [\x for BIOS patches](http://paranoid.anal-slavery.com/biosmods.html)
* [/u/SlowStopper](https://www.reddit.com/r/thinkpad/comments/k6jaie/a_year_in_the_making_hear_my_x330_story/) for figuring out what's the problem
* Everyone else who made the X230 to what it is today, specifically towards BIOS patching and Coreboot

### TBD
Fork 1vyrain and insert a coreboot script so coreboot can be installed painlessly for users for both variants of the machine without external flashing. Not sure if it's useful though as most people who'd install coreboot would also has an external flasher and enough know-how or time to get this working via hardware.
