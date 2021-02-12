# X330 BIOS
## What this repo contains
**Stock BIOS**
* Stock BIOS dumps
* BIOS patches & relevant sources
* Guides on patching BIOS
**Coreboot**
* Relevant coreboot files
* Scripts to simplify compilation for first-time users
* VBT modification & explanations

##Basics: How to flash an X230 BIOS image
1. The X230 has 2 BIOS EEPROM (storage) SPI chips concatenanted (joined) together.
1. To flash any image, the image has to be broken up into 2 pieces, 
