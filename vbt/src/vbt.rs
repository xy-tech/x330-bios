
use std::result;
use std::io::{Cursor, Read, Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::OpenOptions;

use crate::Error;

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Vbt {
    pub signature: [u8; 20],
    pub version: u16,
    pub header_size: u16,
    pub vbt_size: u16,
    pub vbt_checksum: u8,
    pub reserved0: u8,
    pub bdb_offset: u32,
    pub aim_offset: [u32; 4],
}

impl Vbt {
    pub fn parse(input: & [u8]) -> Result<(Vbt, &[u8])> {
        let mut cursor = Cursor::new(input);
        let mut signature: [u8; 20] = [0; 20];
        cursor.read_exact(&mut signature)?;
        let version = cursor.read_u16::<LittleEndian>()?;
        let header_size = cursor.read_u16::<LittleEndian>()?;
        let vbt_size = cursor.read_u16::<LittleEndian>()?;
        let vbt_checksum = cursor.read_u8()?;

        let reserved0 = cursor.read_u8()?;
        let bdb_offset = cursor.read_u32::<LittleEndian>()?;
        let mut aim_offset: [u32; 4] = [0; 4];
        for i in 0..4 {
            aim_offset[i] = cursor.read_u32::<LittleEndian>()?;
        }

        if input.len() < vbt_size as usize {
            return Err(Error::VbtSizeError);
        }

        let vbt = Vbt {
            signature: signature,
            version: version,
            header_size: header_size,
            vbt_size: vbt_size,
            vbt_checksum: vbt_checksum,
            reserved0: reserved0,
            bdb_offset: bdb_offset,
            aim_offset: aim_offset,
        };

        Ok((vbt, input.split_at(cursor.position() as usize).1))
    }

    pub fn export(&self, buffer: &mut [u8]) -> Result<u64> {
        if buffer.len() < self.header_size as usize {
            return Err(Error::VbtSizeError);
        }

        let mut cursor = Cursor::new(buffer);
        cursor.write(&self.signature)?;
        cursor.write_u16::<LittleEndian>(self.version)?;
        cursor.write_u16::<LittleEndian>(self.header_size)?;
        cursor.write_u16::<LittleEndian>(self.vbt_size)?;
        cursor.write_u8(self.vbt_checksum)?;
        cursor.write_u8(self.reserved0)?;
        cursor.write_u32::<LittleEndian>(self.bdb_offset)?;
        for i in 0..4 {
            cursor.write_u32::<LittleEndian>(self.aim_offset[i])?;
        }

        Ok(cursor.position())
    }
}

#[derive(Debug)]
pub struct BdbHeader {
    pub signature: [u8; 16],
    pub version: u16,
    pub header_size: u16,
    pub bdb_size: u16,
    pub body: Vec<u8>,
}

impl BdbHeader {
    pub fn parse(input: &[u8]) -> Result<(BdbHeader, &[u8])> {
        let mut cursor = Cursor::new(input);
        let mut signature: [u8; 16] = [0; 16];
        let version: u16;
        let header_size: u16;
        let bdb_size: u16;

        cursor.read_exact(&mut signature)?;
        version = cursor.read_u16::<LittleEndian>()?;
        header_size = cursor.read_u16::<LittleEndian>()?;
        bdb_size = cursor.read_u16::<LittleEndian>()?;

        if input.len() < bdb_size as usize {
            return Err(Error::IncompleteError);
        }

        let (body, remains) = input.split_at(bdb_size as usize);
        let mut bodyvec: Vec<u8> = vec![0; (bdb_size - header_size) as usize];
        bodyvec.copy_from_slice(&body[header_size as usize..body.len()]);

        let bdb = BdbHeader {
            signature: signature,
            version: version,
            header_size: header_size,
            bdb_size: bdb_size,
            body: bodyvec,
        };

        Ok((bdb, remains))
    }

    pub fn export(&self, buffer: &mut [u8]) -> Result<u64> {
        if buffer.len() < self.header_size as usize {
            return Err(Error::VbtSizeError);
        }

        let mut cursor = Cursor::new(buffer);
        cursor.write(&self.signature)?;
        cursor.write_u16::<LittleEndian>(self.version)?;
        cursor.write_u16::<LittleEndian>(self.header_size)?;
        cursor.write_u16::<LittleEndian>(self.bdb_size)?;
        cursor.write(&self.body)?;

        Ok(cursor.position())
    }
}

#[derive(Debug, Clone)]
pub struct BdbBlock {
    pub id: u8,
    /** most block have u16 size, but some still have u32 */
    pub size: u32,
    pub body: Vec<u8>,
}

impl BdbBlock {
    pub fn parse(input: &[u8]) -> Result<(BdbBlock, &[u8])> {
        let mut cursor = Cursor::new(input);

        let id = cursor.read_u8()?;
        let size = cursor.read_u16::<LittleEndian>()? as u32;

        if input.len() < (cursor.position() + (size as u64)) as usize {
            return Err(Error::IncompleteError);
        }

        let body = input.split_at(cursor.position() as usize).1;
        let mut bodyvec: Vec<u8> = vec![0; size as usize];
        bodyvec.copy_from_slice(body.split_at(size as usize).0);

        let block = BdbBlock {
            id: id,
            size: size,
            body: bodyvec,
        };

        Ok((block, input.split_at((cursor.position() + size as u64) as usize).1))
    }

    pub fn export(&self, buffer: &mut [u8]) -> Result<u64> {
        if buffer.len() < (self.size + 3) as usize {
            return Err(Error::VbtSizeError);
        }

        let mut cursor = Cursor::new(buffer);
        cursor.write_u8(self.id)?;
        cursor.write_u16::<LittleEndian>(self.size as u16)?;
        cursor.write(&self.body)?;

        Ok(cursor.position())
    }
}

pub fn write_to_file(
        vbt: &Vbt,
        bdbheader: &BdbHeader,
        filename: &str) -> Result<bool> {

    /* open a new file */
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(filename)?;

    /* bdb_header */
    let mut bdb_complete = vec![0; bdbheader.bdb_size as usize];
    bdbheader.export(&mut bdb_complete[..])?;

    /* vbt header */
    let mut vbt_data = vec![0; 48];
    vbt.export(&mut vbt_data[..])?;

    file.write_all(&vbt_data[..])?;
    file.seek(SeekFrom::Start(vbt.bdb_offset as u64))?;
    file.write_all(&bdb_complete[..])?;
    file.write_all(b"\x00")?;

    Ok(true)
}

const BDB_GENERAL_DEFINITIONS:      u8 = 2;
const BDB_DRIVER_FEATURES:          u8 = 12;

pub fn find_block(id: u8, blocks: &Vec<BdbBlock>) -> Result<usize> {
    for i in 0..blocks.len() {
        if blocks[i].id == id {
            return Ok(i);
        }
    }

    Err(Error::BlockNotFound)
}

#[derive(Debug, Default)]
pub struct GeneralDefinitions {
    /* DDC GPIO */
    pub crt_ddc_gmbus_pin: u8,

    pub dpms: u8,

    /* boot device bits */
    pub boot_display: u16,

    /* how big a child dev is */
    pub child_dev_size: u8,

    pub childs: Vec<u8>,
}

impl GeneralDefinitions {
    pub fn parse(input: &[u8]) -> Result<(GeneralDefinitions, &[u8])> {
        let mut cursor = Cursor::new(input);

        let crt_ddc_gmbus_pin = cursor.read_u8()?;
        let dpms = cursor.read_u8()?;
        let boot_display = cursor.read_u16::<LittleEndian>()?;
        let child_dev_size = cursor.read_u8()?;

        let remains = input.len() as u64 - cursor.position() as u64;

        if remains % child_dev_size as u64 != 0 {
            return Err(Error::GeneralDefinitionInvalidSize);
        }

        let childs = input.split_at(cursor.position() as usize).1;

        let block = GeneralDefinitions {
            crt_ddc_gmbus_pin: crt_ddc_gmbus_pin,
            dpms: dpms,
            boot_display: boot_display,
            child_dev_size: child_dev_size,
            childs: childs.to_vec(),
        };

        Ok((block, input.split_at((cursor.position()) as usize).1))
    }

    pub fn export(&self, buffer: &mut [u8]) -> Result<u64> {
        let mut cursor = Cursor::new(buffer);

        cursor.write_u8(self.crt_ddc_gmbus_pin)?;
        cursor.write_u8(self.dpms)?;
        cursor.write_u16::<LittleEndian>(self.boot_display)?;
        cursor.write_u8(self.child_dev_size)?;
        cursor.write(&self.childs)?;

        Ok(cursor.position())
    }
}

pub fn replace_ldvs_block(blocks: &mut Vec<BdbBlock>) -> Result<()> {
    let block = find_block(BDB_GENERAL_DEFINITIONS, &blocks)?;

    {
        /* edp_child */
        let mut edp_child: usize = 0;
        let mut edp_found: usize = 0;

        let raw_block = &mut blocks[block];
        let mut general = (GeneralDefinitions::parse(&raw_block.body)?).0;

        let mut new_childs: Vec<u8> = general.childs.clone();

        let child_len = new_childs.len();
        let num_childs = child_len / general.child_dev_size as usize;
        let mut cursor = Cursor::new(general.childs.clone());

        /* find edp child */
        for child in 0..num_childs {
            /* search for the handle 0x0020 (EFP 3 (HDMI/DVI/DP)) */
            let offset = child * general.child_dev_size as usize;
            cursor.set_position(offset as u64);
            let device_handle = cursor.read_u16::<LittleEndian>()?;
            let device_type = cursor.read_u16::<LittleEndian>()?;
            println!("Child: {:#x?} {:#x?}", device_handle, device_type);

            if device_handle == 0x20 {
                edp_child = offset;
                edp_found = 1;
                break;
            }
        }

        if edp_found == 0 {
            return Err(Error::EDPPortNotFound);
        }

        {
            /* copy the edp to the first port */
            let mut write = Cursor::new(&mut new_childs);
            write.write(&general.childs[edp_child..(edp_child+general.child_dev_size as usize)])?;

            write.set_position(0);

            /* DEVICE_HANDLE_LPF1 */
            write.write_u16::<LittleEndian>(0x08)?;
            write.write_u16::<LittleEndian>(0x78c6)?;

            /* remove the old edp child */
            write.set_position(edp_child as u64);
            for _ in 0..general.child_dev_size {
                write.write_u8(0x0)?;
            }
        }

        general.childs = new_childs;
        let mut general_vec: Vec<u8> = vec![0; raw_block.body.len()];
        general.export(&mut general_vec).unwrap();

        // Swap out body
        raw_block.body = general_vec;
    }

    Ok(())
}

pub fn set_driver_features(blocks: &mut Vec<BdbBlock>) -> Result<()> {
    let block = find_block(BDB_DRIVER_FEATURES, &blocks)?;
    let old = &mut blocks[block];

    old.body[8] |= 3 << 3;
    Ok(())
}
