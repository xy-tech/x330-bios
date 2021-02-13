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

## Recommended guide & readings
* [Has important information on how to get started](https://www.chucknemeth.com/laptop/lenovo-x230/flash-lenovo-x230-coreboot#prepare-coreboot)
* [Skulls website with a ton of extra info](https://github.com/merge/skulls/tree/master/x230)
* [My website]()

## Differences between this repo and all the other repos out there
* The VBT is fully patched which should resolve every single multiple screen problem out there
* BIOS dumps are provided for easy flashing

## A short intro of things
* The X230 has 2 BIOS EEPROM (storage) SPI chips concatenanted (joined) together.
* To external flash any image, the image has to be broken up into 2 parts and then flashed separately. 
* A full BIOS image actually contains quite a few things: the management engine (ME), flash descriptor (FD), the actual BIOS ROM etc.
* The stock image stores the actual BIOS portion in the top chip only. 
* Limiting coreboot (BIOS) to 0x400000 (4MB) means the bottom chip (8MB) is untouched between the stock ROM & Coreboot. This means only the top chip (4MB) needs to be flashed.

## External & internal flashing
* Generated coreboot and downloaded images are usually 12MB, with the first 8MB as the bottom chip and the last 4MB as the top chip
* Physical flashing would require the 4MB and 8MB files to be split while an internal flash requires the full 12MB by default
* To join these 2 files together, run `cat bottom.bin top.bin > bios.rom`
* To split an image into it's corresponding files, run `dd if=bios.rom of=top.bin bs=1M skip=8` for the top chip and `dd if=bios.rom of=bottom.bin bs=1M count=8` for bottom chip
* Provided images in this repo are for the convenience of flashing. Join or split them according to what you plan to do.

## How to flash images (for Windows)
Follow the instructions in the Linux section but do it under Windows subsystem for Linux instead.

## How to flash binary images (for Linux)
1. Install [flashrom](https://www.flashrom.org/Flashrom)
1. For internal flashing, run this command: `flashrom -p internal -w image.rom`

*IMPORTANT: CH341a flasher may have issues flashing the stock bottom chip (8MB) as I've painfully experienced. Internally flash it in coreboot instead of trying to external flash it*
*Do not attempt to run the coreboot bottom chip (ME neutered, IFD unlocked) with the stock BIOS as it will not load*

# License
GPLv3 for all scripts etc. Proprietary license for Lenovo stuff. Pls don't sue me. All other stuff belongs to their respective copyright holders

# Thanks
* Alexander Couzens [X330 patch](https://review.coreboot.org/c/coreboot/+/28950) & [VBT patch](https://code.fe80.eu/lynxis/vbtparse)
* [\x for BIOS patches](http://paranoid.anal-slavery.com/biosmods.html)
* [/u/SlowStopper](https://www.reddit.com/r/thinkpad/comments/k6jaie/a_year_in_the_making_hear_my_x330_story/) for figuring out what's the problem
* Everyone else who made the X230 to what it is today, specifically towards BIOS patching and Coreboot

### TBD
Fork 1vyrain and insert a coreboot script so coreboot can be installed painlessly for users for both variants of the machine without external flashing. Not sure if it's useful though as most people who'd install coreboot would also has an external flasher and enough know-how or time to get this working via hardware.
