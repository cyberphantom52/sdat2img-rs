use crate::{
    transfer_list::{Command, TransferList},
    BLOCK_SIZE,
};
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

pub struct SparseDecoder<W: Write + Seek, R: Read> {
    transfer_list: TransferList,
    source: BufReader<R>,
    destination: BufWriter<W>,
    quiet: bool,
}

impl<W: Write + Seek, R: Read> SparseDecoder<W, R> {
    pub fn new(transfer_list: TransferList, source: R, destination: W) -> Self {
        SparseDecoder {
            transfer_list,
            source: BufReader::new(source),
            destination: BufWriter::new(destination),
            quiet: false,
        }
    }

    pub fn enable_quiet(&mut self) {
        self.quiet = true;
    }

    pub fn decode(&mut self) -> std::io::Result<()> {
        let commands = self.transfer_list.commands();

        let file_size = BLOCK_SIZE
            * commands
                .iter()
                .flat_map(|cmd| cmd.rangeset().iter())
                .map(|range| range.end())
                .max()
                .unwrap();

        // Dirty hack to match file size
        self.destination.seek(SeekFrom::Start(file_size - 1))?;
        self.destination.write_all(&[0])?;

        for command in commands {
            match command {
                Command::New(rangeset) => {
                    for range in rangeset.iter() {
                        let offset = range.start() * BLOCK_SIZE;
                        let data_size = (range.len() * BLOCK_SIZE) as usize;
                        if !self.quiet {
                            println!("Writing {data_size} bytes at offset {offset}...");
                        }

                        self.destination.seek(SeekFrom::Start(offset))?;

                        let mut buffer = vec![0u8; data_size];
                        self.source.read_exact(&mut buffer)?;
                        self.destination.write_all(&buffer)?;
                    }
                }
                _ if !self.quiet => println!("Skipping command {}...", command),
                _ => {}
            }
        }

        Ok(())
    }
}
