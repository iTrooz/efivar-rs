use anyhow::{Context, Ok};
use efivar::boot::{EFIHardDrive, EFIHardDriveType};
use std::path::PathBuf;
use win_partlist::types::PartitionExtra;
use std::fmt;

pub struct Partition {
    pub disk_id: usize,
    pub partition_id: usize,
}

impl fmt::Display for Partition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Disk {}, Partition {}", self.disk_id, self.partition_id)
    }
}

pub fn query_partition(disk_id: Option<String>, partition_id: String) -> anyhow::Result<Partition> {
    let disk_id = disk_id.context("Disk ID must be provided")?;

    Ok(Partition {
        disk_id: disk_id
            .parse::<usize>()
            .context(format!("Invalid disk ID: {disk_id}"))?,
        partition_id: partition_id
            .parse::<usize>()
            .context(format!("Invalid partition ID: {partition_id}"))?,
    })
}

pub fn retrieve_efi_partition_data(partition_arg: &Partition) -> anyhow::Result<EFIHardDrive> {
    let disks = win_partlist::list_disks().map_err(anyhow::Error::from_boxed)?;
    let disk = disks.get(partition_arg.disk_id).context("Disk not found")?;

    let partition_win = disk
        .partitions
        .get(partition_arg.partition_id as usize)
        .context("Partition not found")?;
    let partition_win_gpt = match partition_win.extra {
        PartitionExtra::Gpt(ref gpt) => gpt,
        _ => {
            anyhow::bail!(
                "Partition {} on disk {} is not a GPT partition",
                partition_arg.partition_id,
                partition_arg.disk_id
            );
        }
    };

    // Just to be sure
    assert!(partition_arg.partition_id == partition_win.partition_number as usize);

    Ok(EFIHardDrive {
        partition_number: partition_win.partition_number,
        partition_start: partition_win.starting_offset as u64,
        partition_size: partition_win.partition_length as u64,
        partition_sig: partition_win_gpt.partition_id,
        format: 0x02, // no idea
        sig_type: EFIHardDriveType::Gpt,
    })
}

pub fn get_mount_point(_name: &Partition) -> Option<PathBuf> {
    None
}
