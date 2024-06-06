use color_eyre::eyre::Result;
use fat32::Fat;
use libk::io::{Read, Seek, SeekFrom, Write};
use std::fs::File;

struct FileWrapper(File);

impl Read for FileWrapper {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, libk::io::Error> {
        Ok(std::io::Read::read(&mut self.0, buf).unwrap())
    }
}

impl Write for FileWrapper {
    fn write(&mut self, buf: &[u8]) -> Result<usize, libk::io::Error> {
        Ok(std::io::Write::write(&mut self.0, buf).unwrap())
    }

    fn flush(&mut self) -> Result<(), libk::io::Error> {
        Ok(std::io::Write::flush(&mut self.0).unwrap())
    }
}

impl Seek for FileWrapper {
    fn seek(&mut self, seek_from: SeekFrom) -> Result<u64, libk::io::Error> {
        let seek_from = match seek_from {
            SeekFrom::Start(n) => std::io::SeekFrom::Start(n),
            SeekFrom::End(n) => std::io::SeekFrom::End(n),
            SeekFrom::Current(n) => std::io::SeekFrom::Current(n),
        };
        Ok(std::io::Seek::seek(&mut self.0, seek_from).unwrap())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let image = FileWrapper(File::open("test.img")?);
    let fat_reader = Fat::new(image);

    dbg!(fat_reader.fat_info);

    Ok(())
}
