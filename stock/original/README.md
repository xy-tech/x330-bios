# This folder contains a stock BIOS dump. Flash this at your own risk.
*Firmware is v2.72, right before spectre and meltdown for the best performance.*

## Firmware extraction from Lenovo stock BIOS
*As the provided firmware is in [eltorito format](https://codedump.net/blog/extracting-a-bootable-disk-image-from-an-iso-image), it needs to be converted to an img file before we can extract the image*

*IMPORTANT: CH341a flasher may have issues flashing the stock bottom chip as I've painfully experienced. Internally flash it in coreboot instead of trying to external flash it*

*Do not attempt to run an IFD unlocked bottom chip with the stock BIOS as it will not load*

Note: stock firmware updates do not overwrite the bottom chip (the 8MB file)

1. Download firmware ISO from Lenovo
1. Install [geteltorito](http://manpages.ubuntu.com/manpages/trusty/man1/geteltorito.1.html)
1. Extract the img file with geteltorito `geteltorito bios.iso -o bios.img`
1. Navigate and find a 4.2MB .fl1 file which is the size of the BIOS
1. Run this to get a top chip dump `dd if=bios.fl1 bs=1 of stock.bin skip=464 count=4194304` ([Courtesy from 1vyrain](https://github.com/n4ru/1vyrain/blob/master/tools/patcher/patch.sh))
1. This binary can then be flased externally directly or internally after combining with the bottom chip binary

## Bottom image
The bottom image contains my dumped Lenovo image and will cause the Gbe to have the same mac address. It is highly recommended to use your own bottom chip in case any issues appears.

## TO-DO:
A small script to download any Lenovo image and extract the .bin BIOS file from it. 
