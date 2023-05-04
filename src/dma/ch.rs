#[doc = r"Register block"]
#[repr(C)]
pub struct CH {
    #[doc = "0x00 - DMA channel source address"]
    pub ch_src_addr: CH_SRC_ADDR,
    #[doc = "0x04 - DMA channel destination address"]
    pub ch_dst_addr: CH_DST_ADDR,
    #[doc = "0x08 - DMA channel linked list item"]
    pub ch_lli: CH_LLI,
    #[doc = "0x0c - DMA channel control"]
    pub ch_control: CH_CONTROL,
    #[doc = "0x10 - DMA channel config"]
    pub ch_config: CH_CONFIG,
}
#[doc = "CH_SRC_ADDR (rw) register accessor: an alias for `Reg<CH_SRC_ADDR_SPEC>`"]
pub type CH_SRC_ADDR = crate::Reg<ch_src_addr::CH_SRC_ADDR_SPEC>;
#[doc = "DMA channel source address"]
pub mod ch_src_addr;
#[doc = "CH_DST_ADDR (rw) register accessor: an alias for `Reg<CH_DST_ADDR_SPEC>`"]
pub type CH_DST_ADDR = crate::Reg<ch_dst_addr::CH_DST_ADDR_SPEC>;
#[doc = "DMA channel destination address"]
pub mod ch_dst_addr;
#[doc = "CH_LLI (rw) register accessor: an alias for `Reg<CH_LLI_SPEC>`"]
pub type CH_LLI = crate::Reg<ch_lli::CH_LLI_SPEC>;
#[doc = "DMA channel linked list item"]
pub mod ch_lli;
#[doc = "CH_CONTROL (rw) register accessor: an alias for `Reg<CH_CONTROL_SPEC>`"]
pub type CH_CONTROL = crate::Reg<ch_control::CH_CONTROL_SPEC>;
#[doc = "DMA channel control"]
pub mod ch_control;
#[doc = "CH_CONFIG (rw) register accessor: an alias for `Reg<CH_CONFIG_SPEC>`"]
pub type CH_CONFIG = crate::Reg<ch_config::CH_CONFIG_SPEC>;
#[doc = "DMA channel config"]
pub mod ch_config;
