#![feature(env, fs, io, path)]

extern crate bzip2;

use bzip2::Compress;
use bzip2::reader::BzCompressor;
use std::fs::File;
use std::io::{BufReader, BufWriter, Result};
use std::path::Path;

fn main() {
    let args: Vec<_> = std::env::args().collect();

    match &args[..] {
        [_, ref arg1] => {
            let path = Path::new(&arg1);
            let path_spec = {
                let mut path_spec = arg1.to_string();
                path_spec.push_str(".cmp");
                path_spec
            };
            let compressed_path = Path::new(&path_spec);

            match compress_file(&path, &compressed_path) {
                Ok(count) => { println!("Success! {} bits written.", count); },
                Err(e) => {
                    println!("{}", e.description());
                }
            }
        },
        _ => {
            println!("USAGE: {} FILENAME", args[0]);
        }
    }
}

fn compress_file(input: &Path, output: &Path) -> Result<i64> {
    let reader = match File::open(input) {
        Ok(f) => BufReader::new(f),
        Err(e) => return Err(e),
    };

    let mut writer = match File::create(output) {
        Ok(f) => BufWriter::new(f),
        Err(e) => return Err(e),
    };

    match std::io::copy(
        &mut BzCompressor::new(reader, Compress::Best), 
        &mut writer) 
    {
        Ok(written) => {
            let original_size = std::fs::metadata(input).ok().unwrap().len();
            Ok(if written > original_size { 
                0i64 - (written as i64 - original_size as i64)
            } else {
                (original_size - written) as i64
            })
        },
        Err(e) => Err(e)
    }
}
