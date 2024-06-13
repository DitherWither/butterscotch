// use libk::io::{self, Read, Seek};
// use pretty_hex::*;

// #[derive(Debug)]
// pub enum FatInfo {
//     // TODO add exfat
//     ExFAT,
//     Fat12 { bpb: FatBPB, ebpb: Fat16EBPB },
//     Fat16 { bpb: FatBPB, ebpb: Fat16EBPB },
//     Fat32 { bpb: FatBPB, ebpb: Fat32EBPB },
// }

// impl FatInfo {
//     pub fn get_bpb(&self) -> Option<&FatBPB> {
//         match self {
//             FatInfo::Fat12 { bpb, .. } => Some(bpb),
//             FatInfo::Fat16 { bpb, .. } => Some(bpb),
//             FatInfo::Fat32 { bpb, .. } => Some(bpb),
//             FatInfo::ExFAT => None,
//         }
//     }

//     pub fn is_fat32(&self) -> bool {
//         match self {
//             FatInfo::Fat32 { .. } => true,
//             _ => false,
//         }
//     }

//     pub fn is_fat16(&self) -> bool {
//         match self {
//             FatInfo::Fat16 { .. } => true,
//             _ => false,
//         }
//     }

//     pub fn is_fat12(&self) -> bool {
//         match self {
//             FatInfo::Fat12 { .. } => true,
//             _ => false,
//         }
//     }

//     pub fn is_exfat(&self) -> bool {
//         false
//     }
// }

// pub struct Fat<T>
// where
//     T: Read + Seek,
// {
//     disk: T,
//     pub fat_info: FatInfo,
// }

// impl<T> Fat<T>
// where
//     T: Read + Seek,
// {
//     pub fn new(mut disk: T) -> Self {
//         let mut boot_record = [0u8; 512];

//         disk.seek(io::SeekFrom::Start(0)).unwrap();
//         disk.read(&mut boot_record).unwrap();

//         // TODO add proper error handling

//         let bpb: FatBPB = unsafe { libk::mem::transmute(boot_record) };
//         let ebpb16: Fat16EBPB = unsafe { libk::mem::transmute(bpb.ebpb) };
//         let ebpb32: Fat32EBPB = unsafe { libk::mem::transmute(bpb.ebpb) };

//         let total_sectors = if bpb.total_sectors_16 == 0 {
//             bpb.total_sectors_32
//         } else {
//             bpb.total_sectors_16 as u32
//         };

//         let sectors_per_fat = if bpb.sectors_per_fat == 0 {
//             ebpb32.sectors_per_fat
//         } else {
//             bpb.sectors_per_fat as u32
//         };

//         let root_dir_sectors = (((bpb.root_entry_count * 32) + (bpb.bytes_per_sector - 1))
//             / bpb.bytes_per_sector) as u32;

//         let first_data_sector = bpb.reserved_sector_count as u32
//             + (bpb.table_count as u32 * bpb.sectors_per_fat as u32)
//             + root_dir_sectors;

//         let data_sectors = total_sectors
//             - (bpb.reserved_sector_count as u32
//                 + (bpb.table_count as u32 * sectors_per_fat)
//                 + root_dir_sectors as u32);

//         let total_clusters = data_sectors / bpb.sectors_per_cluster as u32;

//         let fat_info = if bpb.bytes_per_sector == 0 {
//             FatInfo::ExFAT
//         } else if total_clusters < 4085 {
//             FatInfo::Fat12 { bpb, ebpb: ebpb16 }
//         } else if total_clusters < 65525 {
//             FatInfo::Fat16 { bpb, ebpb: ebpb16 }
//         } else {
//             FatInfo::Fat32 { bpb, ebpb: ebpb32 }
//         };

//         let bpb = fat_info.get_bpb().unwrap();

//         // Read root dir
//         if fat_info.is_fat12() || fat_info.is_fat16() {
//             let first_root_dir_sector = first_data_sector - root_dir_sectors;
//         } else {
//             let root_cluster = match fat_info {
//                 FatInfo::Fat32 { ref ebpb, .. } => ebpb.root_cluster,
//                 FatInfo::ExFAT => todo!(),
//                 _ => unreachable!(),
//             };

//             let first_sector_of_cluster = ((root_cluster as u64 - 2)
//                 * bpb.sectors_per_cluster as u64)
//                 + first_data_sector as u64;

//             println!("{first_sector_of_cluster}");

//             let mut buf = [0u8; 512];
//             disk.seek(io::SeekFrom::Start(first_sector_of_cluster * bpb.bytes_per_sector as u64))
//                 .unwrap();
//             disk.read(&mut buf).unwrap();

//             println!("{:?}", buf.hex_dump())
//         }

//         Self { disk, fat_info }
//     }
// }

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
#[repr(packed)]
pub struct Fat16EBPB {
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
#[repr(packed)]
pub struct Fat32EBPB {
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
