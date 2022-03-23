# Coreboot

## Rough installation instructions
1. Download coreboot image from releases

### Internal flashing with coreboot
1. Flash it in internally using [flashrom](https://www.flashrom.org/Flashrom) if you're running coreboot: `flashrom -p internal -w coreboot.rom --ifd -i bios`

### External flashing with clip
1. Split it into the 4MB `dd if=coreboot.rom of=top.bin bs=1M skip=8` and 8MB binaries `dd if=coreboot.rom of=bottom.bin bs=1M count=8`
1. Flash it in with an external clip `sudo flashrom -p ch341a_spi -c <chip> -w <rom.bin>`

## Rough instructions on how to build
1. Clone Coreboot repository to local: `git clone --recurse-submodules https://review.coreboot.org/coreboot.git`
1. Cherry pick the FHD patch: `git fetch https://review.coreboot.org/coreboot refs/changes/50/28950/15 && git cherry-pick FETCH_HEAD`
1. Copy extra and dotconfig into the Coreboot root folder
1. Rename dotconfig to .config
1. Build the image: `make`

## More information
* The extra folder contains all the required files for a minimal X330 4MiB build.
* The dotconfig file contains configs necessary to build Coreboot.
* The VBT is modified to remove the internal display in Windows. See the VBT folder for more info.
* This is based on tianocore which does not have backwards support for MBR/BIOS OS.
* This build uses libgfxinit as the graphics init.
* The Intel ME is neutered for this build. Might affect hackintosh installs and battery life although I've not done any testing on it.
* This build is also 4MB which is suitable for 1vyrain installs directly without any hardware flash. Fully tested with 1vyrain.