use clap::Parser;
use sdat2img_rs::{SparseDecoder, TransferList};
use std::{
    fs::{File, OpenOptions},
    path::PathBuf,
};

#[derive(Parser, Debug)]
#[command(
    author = "Inam Ul Haq",
    version = "1.0",
    about = "Convert Android sparse data files to system images",
    long_about = None
)]
struct Args {
    #[arg(short = 't', long = "transfer-list")]
    transfer_list: PathBuf,

    #[arg(short = 's', long = "sparse-image")]
    new_dat_file: PathBuf,

    #[arg(short = 'o', long = "output")]
    output_image: PathBuf,

    #[arg(short, long)]
    quiet: bool,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let transfer_list = args.transfer_list;
    let transfer_list = TransferList::try_from(transfer_list.as_path()).unwrap();

    if !args.quiet {
        let version_msg = match transfer_list.version() {
            1 => "Android Lollipop 5.0 detected!",
            2 => "Android Lollipop 5.1 detected!",
            3 => "Android Marshmallow 6.x detected!",
            4 => "Android Nougat 7.x / Oreo 8.x detected!",
            _ => "Unknown Android version detected!",
        };
        println!("{}", version_msg);
    }

    let sparse_file = args.new_dat_file;
    let source = File::open(sparse_file)?;

    let output_image = args.output_image;
    let destination = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output_image)?;

    let mut decoder = SparseDecoder::new(transfer_list, source, &destination);
    if args.quiet {
        decoder.enable_quiet();
    }

    decoder.decode()
}
