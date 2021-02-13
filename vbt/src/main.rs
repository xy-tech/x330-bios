pub mod vbt;

use std::fs;
use vbt::*;

use quick_error::quick_error;

quick_error! {
    /// All errors returned and used by the radiotap module.
    #[derive(Debug)]
    pub enum Error {
        /// The internal cursor on the data returned an IO error.
        ParseError(err: std::io::Error) {
            from()
            description(err.description())
        }
        IncompleteError {
            display("Incomplete Bdb header or corrupt file.")
        }
        VbtSizeError {
            display("Incomplete Bdb header or corrupt file.")
        }
        BlockNotFound {
            display("Can not find the requested block.")
        }
        GeneralDefinitionInvalidSize {
            display("There a bytes left in GeneralDefinition childs")
        }
        EDPPortNotFound {
            display("Can not find the eDP port.")
        }
    }
}

fn main() {
    let data = fs::read("data.vbt").unwrap();

    let mut new_bdb: Vec<u8>;

    let (vbt, _remains) = Vbt::parse(&data).unwrap();

    let sli = &data.split_at(vbt.bdb_offset as usize).1;
    let (mut bdbheader, _remains) = BdbHeader::parse(sli).unwrap();

    let mut vec: Vec<BdbBlock> = Vec::new();

    {
        let mut remains = &bdbheader.body[..];

        while remains.len() > 8 {
            let result = BdbBlock::parse(remains);
            let block: BdbBlock;
            match result {
                Err(e) => {
                    println!("{:?}", e);
                    break;
                },
                Ok(a) => {
                    block = a.0;
                    remains = a.1;
                    vec.push(block);
                },
            }
        }
    }

    /* modify */
    replace_ldvs_block(&mut vec).unwrap();
    set_driver_features(&mut vec).unwrap();

    /* rebuild the blocks back into BdbHeader */
    new_bdb = vec![0; (bdbheader.bdb_size - bdbheader.header_size) as usize];
    let length = new_bdb.len();
    let mut position: usize = 0;
    {
        for block in vec {
            let written: u64 = block.export(&mut new_bdb[position..length]).unwrap();
            position += written as usize;
        }

        if position != length {
            println!("Position not at the end of the buffer... {:?} != {:?}", position, length);
        }
    }

    bdbheader.body = new_bdb;

    write_to_file(&vbt, &bdbheader, "out.vbt").unwrap();
}
