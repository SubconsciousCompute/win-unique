use serde::{Deserialize, Serialize};

/// The `Win32_ComputerSystemProduct` WMI class represents a product. This includes software and
/// hardware used on this computer system.
///
/// <https://learn.microsoft.com/en-us/windows/win32/cimwin32prov/win32-computersystemproduct>
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct Win32_ComputerSystemProduct {
    /// Short textual description for the product.
    pub Caption: Option<String>,
    /// Textual description of the product.
    pub Description: Option<String>,
    /// Product identification, such as a serial number on software, a die number on a hardware
    /// chip, or (for noncommercial products) a project number.
    pub IdentifyingNumber: Option<String>,
    /// Commonly used product name.
    pub Name: Option<String>,
    /// Product's stock-keeping unit (SKU) information.
    pub SKUNumber: Option<String>,
    /// Name of the product's supplier, or the entity selling the product (the manufacturer,
    /// reseller, OEM, and so on).
    pub Vendor: Option<String>,
    /// Product version information.
    pub Version: Option<String>,
    /// Universally unique identifier (UUID) for this product. A UUID is a 128-bit identifier that
    /// is guaranteed to be different from other generated UUIDs. If a UUID is not available, a UUID of all zeros is used.
    ///
    /// This value comes from the UUID member of the System Information structure in the SMBIOS
    /// information.
    pub UUID: Option<String>,
}

/// The `mdm_dev_detail_ext01` class handles the management object which provides device-specific
/// parameters to the OMA DM server. These device parameters are not sent from the client to the
/// server automatically, but can be queried by servers using OMA DM commands.
///
/// <https://learn.microsoft.com/en-us/windows/win32/dmwmibridgeprov/mdm-devdetail-ext01>
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
pub struct MDM_DevDetail_Ext01 {
    /// Identifies the name of the parent node. For this class, the string is "Ext"
    pub InstanceID: Option<String>,
    /// Describes the full path to the parent node. For this class, the string is "./DevDetail/"
    pub ParentID: Option<String>,
    pub DeviceHardwareData: Option<String>,
    pub WLANMACAddress: Option<String>,
}
