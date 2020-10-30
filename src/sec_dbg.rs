#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - sd_chip_id_low."]
    pub sd_chip_id_low: crate::Reg<sd_chip_id_low::SD_CHIP_ID_LOW_SPEC>,
    #[doc = "0x04 - sd_chip_id_high."]
    pub sd_chip_id_high: crate::Reg<sd_chip_id_high::SD_CHIP_ID_HIGH_SPEC>,
    #[doc = "0x08 - sd_wifi_mac_low."]
    pub sd_wifi_mac_low: crate::Reg<sd_wifi_mac_low::SD_WIFI_MAC_LOW_SPEC>,
    #[doc = "0x0c - sd_wifi_mac_high."]
    pub sd_wifi_mac_high: crate::Reg<sd_wifi_mac_high::SD_WIFI_MAC_HIGH_SPEC>,
    #[doc = "0x10 - sd_dbg_pwd_low."]
    pub sd_dbg_pwd_low: crate::Reg<sd_dbg_pwd_low::SD_DBG_PWD_LOW_SPEC>,
    #[doc = "0x14 - sd_dbg_pwd_high."]
    pub sd_dbg_pwd_high: crate::Reg<sd_dbg_pwd_high::SD_DBG_PWD_HIGH_SPEC>,
    #[doc = "0x18 - sd_status."]
    pub sd_status: crate::Reg<sd_status::SD_STATUS_SPEC>,
    #[doc = "0x1c - sd_dbg_reserved."]
    pub sd_dbg_reserved: crate::Reg<sd_dbg_reserved::SD_DBG_RESERVED_SPEC>,
}
#[doc = "sd_chip_id_low register accessor: an alias for `Reg<SD_CHIP_ID_LOW_SPEC>`"]
pub type SD_CHIP_ID_LOW = crate::Reg<sd_chip_id_low::SD_CHIP_ID_LOW_SPEC>;
#[doc = "sd_chip_id_low."]
pub mod sd_chip_id_low;
#[doc = "sd_chip_id_high register accessor: an alias for `Reg<SD_CHIP_ID_HIGH_SPEC>`"]
pub type SD_CHIP_ID_HIGH = crate::Reg<sd_chip_id_high::SD_CHIP_ID_HIGH_SPEC>;
#[doc = "sd_chip_id_high."]
pub mod sd_chip_id_high;
#[doc = "sd_wifi_mac_low register accessor: an alias for `Reg<SD_WIFI_MAC_LOW_SPEC>`"]
pub type SD_WIFI_MAC_LOW = crate::Reg<sd_wifi_mac_low::SD_WIFI_MAC_LOW_SPEC>;
#[doc = "sd_wifi_mac_low."]
pub mod sd_wifi_mac_low;
#[doc = "sd_wifi_mac_high register accessor: an alias for `Reg<SD_WIFI_MAC_HIGH_SPEC>`"]
pub type SD_WIFI_MAC_HIGH = crate::Reg<sd_wifi_mac_high::SD_WIFI_MAC_HIGH_SPEC>;
#[doc = "sd_wifi_mac_high."]
pub mod sd_wifi_mac_high;
#[doc = "sd_dbg_pwd_low register accessor: an alias for `Reg<SD_DBG_PWD_LOW_SPEC>`"]
pub type SD_DBG_PWD_LOW = crate::Reg<sd_dbg_pwd_low::SD_DBG_PWD_LOW_SPEC>;
#[doc = "sd_dbg_pwd_low."]
pub mod sd_dbg_pwd_low;
#[doc = "sd_dbg_pwd_high register accessor: an alias for `Reg<SD_DBG_PWD_HIGH_SPEC>`"]
pub type SD_DBG_PWD_HIGH = crate::Reg<sd_dbg_pwd_high::SD_DBG_PWD_HIGH_SPEC>;
#[doc = "sd_dbg_pwd_high."]
pub mod sd_dbg_pwd_high;
#[doc = "sd_status register accessor: an alias for `Reg<SD_STATUS_SPEC>`"]
pub type SD_STATUS = crate::Reg<sd_status::SD_STATUS_SPEC>;
#[doc = "sd_status."]
pub mod sd_status;
#[doc = "sd_dbg_reserved register accessor: an alias for `Reg<SD_DBG_RESERVED_SPEC>`"]
pub type SD_DBG_RESERVED = crate::Reg<sd_dbg_reserved::SD_DBG_RESERVED_SPEC>;
#[doc = "sd_dbg_reserved."]
pub mod sd_dbg_reserved;
