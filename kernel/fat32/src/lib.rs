#![no_std]

use libk::{
    io::{self, Read, Seek},
    string::String,
};

#[derive(Debug)]
pub enum FatInfo {
    // TODO parse exfat boot record
    ExFat,
    Fat12(FatBPB, Fat16EBPB),
    Fat16(FatBPB, Fat16EBPB),
    Fat32(FatBPB, Fat32EBPB),
}

pub struct Fat<T>
where
    T: Read + Seek,
{
    disk: T,
    pub fat_info: FatInfo,
}

impl<T> Fat<T>
where
    T: Read + Seek,
{
    pub fn new(mut disk: T) -> Self {
        let mut boot_record = [0u8; 512];

        disk.seek(io::SeekFrom::Start(0)).unwrap();
        disk.read(&mut boot_record).unwrap();

        // TODO add proper error handling

        let bpb: FatBPB = unsafe { libk::mem::transmute(boot_record) };
        let ebpb16: Fat16EBPB = unsafe { libk::mem::transmute(bpb.ebpb) };
        let ebpb32: Fat32EBPB = unsafe { libk::mem::transmute(bpb.ebpb) };

        let total_sectors = if bpb.total_sectors_16 == 0 {
            bpb.total_sectors_32
        } else {
            bpb.total_sectors_16 as u32
        };

        let sectors_per_fat = if bpb.sectors_per_fat == 0 {
            ebpb32.sectors_per_fat
        } else {
            bpb.sectors_per_fat as u32
        };

        let root_dir_sectors =
            ((bpb.root_entry_count * 32) + (bpb.bytes_per_sector - 1)) / bpb.bytes_per_sector;

        let data_sectors = total_sectors
            - (bpb.reserved_sector_count as u32
                + (bpb.table_count as u32 * sectors_per_fat)
                + root_dir_sectors as u32);

        let total_clusters = data_sectors / bpb.sectors_per_cluster as u32;

        let fat_info = if bpb.bytes_per_sector == 0 {
            FatInfo::ExFat
        } else if total_clusters < 4085 {
            FatInfo::Fat12(bpb, ebpb16)
        } else if total_clusters < 65525 {
            FatInfo::Fat16(bpb, ebpb16)
        } else {
            FatInfo::Fat32(bpb, ebpb32)
        };

        Self { disk, fat_info }
    }

    pub fn read_sector(&mut self) -> Result<[u8; 512], io::Error> {
        let mut buf = [0u8; 512];

        let size = self.disk.read(&mut buf)?;

        // TODO try to gracefully handle this case
        assert_eq!(size, buf.len());
        Ok(buf)
    }
}

#[repr(packed)]
#[derive(Debug)]
pub struct FatBPB {
    _boot_jmp: [u8; 3],
    oem_ident: [u8; 8],
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sector_count: u16,
    table_count: u8,
    root_entry_count: u16,
    total_sectors_16: u16,
    media_type: u8,
    sectors_per_fat: u16,
    sectors_per_track: u16,
    head_side_count: u16,
    hidden_sector_count: u32,
    total_sectors_32: u32,

    ebpb: [u8; 476],
}

#[derive(Debug)]
struct Fat16EBPB {
    bios_drive_num: u8,
    _reserved: u8,
    boot_signature: u8,
    volume_id: u32,
    volume_label: [u8; 11],
    fat_type_label: [u8; 8],
    _boot_code: [u8; 448],
    _bootable_partition_signature: u16,
}

#[derive(Debug)]
struct Fat32EBPB {
    sectors_per_fat: u32,
    flags: u16,
    fat_version: u16,
    root_cluster: u32,
    fsinfo: u16,
    backup_bs_sector: u16,
    _reserved_0: [u8; 12],
    drive_number: u8,
    _reserved_1: u8,
    boot_signature: u8,
    volume_id: u32,
    volume_label: [u8; 11],
    fat_type_label: [u8; 8],
    _boot_code: [u8; 420],
    _bootable_partition_signature: u16,
}

// #[derive(Debug, Clone)]
// pub struct Fat32BPB {
//     oem_ident: String,
//     bytes_per_cluster: u16,
//     sectors_per_cluster: u8,
//     reserved_sector_count: u16,
//     table_count: u8,
//     root_entry_count: u16,
//     total_sectors: u32,
//     media_type: u8,
//     sectors_per_track: u16,
//     head_side_count: u16,
//     hidden_sector_count: u32,

//     sectors_per_fat: u32,
//     flags: u16,
//     fat_version: u16,
//     root_cluster: u32,
//     fsinfo: u16,
//     backup_bs_sector: u16,
//     drive_number: u8,
//     boot_signature: u8,
//     volume_id: u32,
//     volume_label: String,
//     fat_type_label: String,
// }

// impl From<FatBPBRaw> for Fat32BPB {
//     fn from(value: FatBPBRaw) -> Self {
//         Self {
//             oem_ident: String::from_utf8(value.oem_ident.to_vec()).unwrap(),
//             bytes_per_cluster: value.bytes_per_cluster,
//             sectors_per_cluster: value.sectors_per_cluster,
//             reserved_sector_count: value.reserved_sector_count,
//             table_count: value.table_count,
//             root_entry_count: value.root_entry_count,
//             media_type: value.media_type,
//             sectors_per_track: value.sectors_per_track,
//             head_side_count: value.head_side_count,
//             hidden_sector_count: value.hidden_sector_count,
//             total_sectors: if value.total_sectors_16 != 0 {
//                 value.total_sectors_16 as u32
//             } else {
//                 value.total_sectors_32
//             },
//             sectors_per_fat: value.sectors_per_fat,
//             flags: value.flags,
//             fat_version: value.fat_version,
//             root_cluster: value.root_cluster,
//             fsinfo: value.fsinfo,
//             backup_bs_sector: value.backup_bs_sector,
//             drive_number: value.drive_number,
//             boot_signature: value.boot_signature,
//             volume_id: value.volume_id,
//             volume_label: String::from_utf8(value.volume_label.to_vec()).unwrap(),
//             fat_type_label: String::from_utf8(value.fat_type_label.to_vec()).unwrap(),

//         }
//     }
// }
