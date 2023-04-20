use crate::definitions::{MDM_DevDetail_Ext01, Win32_ComputerSystemProduct};

pub use wmi::COMLibrary;
use wmi::WMIConnection;

pub mod definitions;

/// Get unique product description, see [`Win32_ComputerSystemProduct`](crate::Win32_ComputerSystemProduct)
/// for details.
///
/// Returns `Some(Vec<Win32_ComputerSystemProduct>)` if the query is successful otherwise None
pub fn product() -> Option<Vec<Win32_ComputerSystemProduct>> {
    let com_con = unsafe { COMLibrary::assume_initialized() };
    if let Ok(wmi_con) = WMIConnection::new(com_con) {
        if let Ok(product) = wmi_con.query::<Win32_ComputerSystemProduct>() {
            Some(product)
        } else {
            None
        }
    } else {
        None
    }
}

/// Windows 10 system can also be identified by a so-called 4K-Hash, a special hash string that is
/// 4000 bytes in size. It is just one piece of information required to register Windows machines
/// yet it may be interesting by itself to uniquely identify Windows operating systems for other
/// purposes, too. See , see [`MDM_DevDetail_Ext01`](crate::MDM_DevDetail_Ext01)
/// for details.
///
/// <https://learn.microsoft.com/en-us/windows/win32/dmwmibridgeprov/mdm-devdetail-ext01>
pub fn mdm_dev_detail_ext01() -> Option<Vec<MDM_DevDetail_Ext01>> {
    let com_con = unsafe { COMLibrary::assume_initialized() };
    if let Ok(wmi_con) = WMIConnection::with_namespace_path("root/cimv2/mdm/dmmap", com_con) {
        if let Ok(product) = wmi_con.query::<MDM_DevDetail_Ext01>() {
            Some(product)
        } else {
            None
        }
    } else {
        None
    }
}
