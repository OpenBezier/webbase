mod errors;
mod linux;
mod macos;
mod windows;

use errors::HWIDError;
#[cfg(target_os = "linux")]
use linux::{get_disk_id, get_hwid, get_mac_address};
#[cfg(target_os = "macos")]
use macos::{get_disk_id, get_hwid, get_mac_address};
#[cfg(target_os = "windows")]
use windows::{get_disk_id, get_hwid, get_mac_address};

#[derive(PartialEq, Eq, Hash)]
pub enum HWIDComponent {
    /// System UUID
    SystemID,
    /// Number of CPU Cores
    CPUCores,
    /// Name of the OS
    OSName,
    /// Current Username
    Username,
    /// Host machine name
    MachineName,
    /// Mac Address
    MacAddress,
    /// CPU Vendor ID
    CPUID,
    /// UUID of the root disk
    DriveSerial,
}

impl HWIDComponent {
    pub fn to_string(&self) -> Result<String, HWIDError> {
        use HWIDComponent::*;
        return match self {
            SystemID => {
                let sysid = get_hwid()?;
                Ok(sysid.trim().to_string())
            }
            CPUCores => {
                let sys = sysinfo::System::new_all();
                let cores = sys.physical_core_count().unwrap_or(2);
                Ok(cores.to_string())
            }
            OSName => {
                let name = sysinfo::System::long_os_version()
                    .ok_or(HWIDError::new("OSName", "Could not retrieve OS Name"))?;
                Ok(name)
            }
            Username => Ok(whoami::username()),
            MachineName => {
                let name = sysinfo::System::host_name()
                    .ok_or(HWIDError::new("HostName", "Could not retrieve Host Name"))?;
                Ok(name)
            }
            MacAddress => {
                let mac = get_mac_address()?;
                Ok(mac.trim().to_string())
            }
            CPUID => {
                use sysinfo::{CpuRefreshKind, RefreshKind, System};
                let s = System::new_with_specifics(
                    RefreshKind::new().with_cpu(CpuRefreshKind::everything()),
                );
                let mut vendor_id = String::default();
                for cpu in s.cpus() {
                    vendor_id = cpu.vendor_id().to_string();
                }
                Ok(vendor_id)
            }
            DriveSerial => get_disk_id(),
        };
    }
}
