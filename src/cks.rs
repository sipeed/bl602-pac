#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - cks_config."]
    pub cks_config: crate::Reg<cks_config::CKS_CONFIG_SPEC>,
    #[doc = "0x04 - data_in."]
    pub data_in: crate::Reg<data_in::DATA_IN_SPEC>,
    #[doc = "0x08 - cks_out."]
    pub cks_out: crate::Reg<cks_out::CKS_OUT_SPEC>,
}
#[doc = "cks_config register accessor: an alias for `Reg<CKS_CONFIG_SPEC>`"]
pub type CKS_CONFIG = crate::Reg<cks_config::CKS_CONFIG_SPEC>;
#[doc = "cks_config."]
pub mod cks_config;
#[doc = "data_in register accessor: an alias for `Reg<DATA_IN_SPEC>`"]
pub type DATA_IN = crate::Reg<data_in::DATA_IN_SPEC>;
#[doc = "data_in."]
pub mod data_in;
#[doc = "cks_out register accessor: an alias for `Reg<CKS_OUT_SPEC>`"]
pub type CKS_OUT = crate::Reg<cks_out::CKS_OUT_SPEC>;
#[doc = "cks_out."]
pub mod cks_out;
