# Patched stock rom
**This directory contains a fully patched stock rom with the replaced VBT and a fully patched stock BIOS with 1vyrain patches.**
Just flash the top.bin and bottom.bin (or the ME disabled image) and there should not be anymore secondary displays.
For more explanation on the VBT patch, go to the VBT folder

## Manual BIOS patching (using Linux or WSL)
1. Download UEFIPatch
1. Run this command to patch an image `uefipatch bios.img patch.txt -o new_bios.img`

## VBT patching for stock ROM
*For more info, read the vbt README.md in this repo.*
1. Download [UEFITool](https://github.com/LongSoft/UEFITool).
1. Open the stock image.
1. Search for `vbt` in text and uncheck unicode.
1. Click on the raw section at `offset 1h`. 
1. Replace it with the updated (modified) VBT.
1. Save the image and flash it into the X230

### Credits
BIOS patches are all hardwork from the community and was downloaded from [here](http://paranoid.anal-slavery.com/biosmods.html). Please refer to the [ThinkPad Subreddit](https://www.reddit.com/r/thinkpad) for more info.
