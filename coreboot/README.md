# Coreboot files

## Rough installation instructions
1. Download coreboot image from rom folder
1. Flash it in internally using [flashrom](https://www.flashrom.org/Flashrom) if you're running coreboot: `flashrom -p internal -w coreboot.rom`
1. Split it into the 4MB and 8MB binaries
1. Flash it in with an external clip

## Rough instructions on how to build
1. Clone Coreboot repository to local
1. Copy extra and dotconfig into the root of the Coreboot folder
1. Rename dotconfig to .config
1. Cherry pick the FHD patch
1. Build the image

## More information
* The extra folder contains all the required files for an X330 build.
* The dotconfig file contains configs necessary to build Coreboot.
* The VBT is modified to remove the internal display in Windows.
* This is based on tianocore which does not have backwards support for MBR/BIOS OS.
* This build uses libgfxinit as the graphics init with no VGA blobs.
* The Intel ME is neutered for this build. Might affect hackintosh installs although I've not done any testing on it.
* This build is also 4MB which is suitable for 1vyrain installs directly without any hardware flash.
