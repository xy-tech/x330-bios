# Patched stock rom
**This directory contains a fully patched stock rom with the replaced VBT and a fully patched stock BIOS with 1vyrain patches.**
Just flash the top.bin and bottom.bin (or the ME disabled image) and there should not be anymore secondary displays.
For more explanation on the VBT patch, go to the VBT folder

## What it contains
* Fully patched, VBT replaced, signed v2.72 v2.60 and v2.77 BIOS
* v2.77 is the latest BIOS from Lenovo
* v2.72 is the last version before spectre & meltdown patch for improved performance
* *v2.60 is the version compatible with 1vyrain and is the recommended patched BIOS for most people*
* Signed firmware to get rid of beeping on boot and to use the TPM chip
* 1vyrain patches (view the exact patch in the .txt patch file)

## Manual BIOS patching (using Linux or WSL)
1. Download [UEFIPatch](https://manpages.ubuntu.com/manpages/disco/man1/UEFIPatch.1.html)
1. Run this command to patch an image `uefipatch bios.img patch.txt -o new_bios.img`

## VBT patching for stock ROM
*For more info, read the vbt README.md in this repo.*
1. Download [UEFITool](https://github.com/LongSoft/UEFITool).
1. Open the stock image.
1. Search for `vbt` in text and uncheck unicode.
1. Click on the raw sections at `offset 1h`. (2 sections at File GUID: `6047B8EC-6D17-45C0-9BCF-63D164B41AB3` and `F053B9B5-82F2-4643-A256-CC752CE49058`)
1. Replace both raw sections with the updated (modified) VBT.
1. Save the image and [sign it](https://github.com/thrimbor/thinkpad-uefi-sign) to remove 5-beeps on boot and to use the TPM chip. (5 beeps means UEFI image is modified)

### Credits
BIOS patches are all hardwork from the community and was downloaded from [here](http://paranoid.anal-slavery.com/biosmods.html). Please refer to the [ThinkPad Subreddit](https://www.reddit.com/r/thinkpad) for more info.
