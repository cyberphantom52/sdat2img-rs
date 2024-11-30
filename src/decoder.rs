use crate::{
    transfer_list::{Command, TransferList},
    BLOCK_SIZE,
};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

pub struct SparseDecoder<W: Write + Seek, R: Read> {
    transfer_list: TransferList,
    source: BufReader<R>,
    destination: BufWriter<W>,
}

impl<W: Write + Seek, R: Read> SparseDecoder<W, R> {
    pub fn new(transfer_list: TransferList, source: R, destination: W) -> Self {
        SparseDecoder {
            transfer_list,
            source: BufReader::new(source),
            destination: BufWriter::new(destination),
        }
    }

    pub fn decode(&mut self) -> std::io::Result<()> {
        let commands = self.transfer_list.commands();
        for command in commands {
            match command {
                Command::New(rangeset) => {
                    for range in rangeset.iter() {
                        let start = range.start() * BLOCK_SIZE;
                        let num_blocks = range.len();
                        println!("Copying {num_blocks}...");
                        self.destination.seek(SeekFrom::Start(start))?;
                        let mut buffer = vec![0u8; (num_blocks * BLOCK_SIZE) as usize];
                        self.source.read_exact(&mut buffer)?;
                        self.destination.write_all(&buffer)?;
                    }
                }
                _ => println!("Skipping command {}", command),
            }
        }

        Ok(())
    }
}
