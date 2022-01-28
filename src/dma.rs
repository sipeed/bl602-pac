#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA_IntStatus."]
    pub dma_int_status: crate::Reg<dma_int_status::DMA_INTSTATUS_SPEC>,
    #[doc = "0x04 - DMA_IntTCStatus."]
    pub dma_int_tcstatus: crate::Reg<dma_int_tcstatus::DMA_INTTCSTATUS_SPEC>,
    #[doc = "0x08 - DMA_IntTCClear."]
    pub dma_int_tcclear: crate::Reg<dma_int_tcclear::DMA_INTTCCLEAR_SPEC>,
    #[doc = "0x0c - DMA_IntErrorStatus."]
    pub dma_int_error_status: crate::Reg<dma_int_error_status::DMA_INTERRORSTATUS_SPEC>,
    #[doc = "0x10 - DMA_IntErrClr."]
    pub dma_int_err_clr: crate::Reg<dma_int_err_clr::DMA_INTERRCLR_SPEC>,
    #[doc = "0x14 - DMA_RawIntTCStatus."]
    pub dma_raw_int_tcstatus: crate::Reg<dma_raw_int_tcstatus::DMA_RAWINTTCSTATUS_SPEC>,
    #[doc = "0x18 - DMA_RawIntErrorStatus."]
    pub dma_raw_int_error_status: crate::Reg<dma_raw_int_error_status::DMA_RAWINTERRORSTATUS_SPEC>,
    #[doc = "0x1c - DMA_EnbldChns."]
    pub dma_enbld_chns: crate::Reg<dma_enbld_chns::DMA_ENBLDCHNS_SPEC>,
    #[doc = "0x20 - DMA_SoftBReq."]
    pub dma_soft_breq: crate::Reg<dma_soft_breq::DMA_SOFTBREQ_SPEC>,
    #[doc = "0x24 - DMA_SoftSReq."]
    pub dma_soft_sreq: crate::Reg<dma_soft_sreq::DMA_SOFTSREQ_SPEC>,
    #[doc = "0x28 - DMA_SoftLBReq."]
    pub dma_soft_lbreq: crate::Reg<dma_soft_lbreq::DMA_SOFTLBREQ_SPEC>,
    #[doc = "0x2c - DMA_SoftLSReq."]
    pub dma_soft_lsreq: crate::Reg<dma_soft_lsreq::DMA_SOFTLSREQ_SPEC>,
    #[doc = "0x30 - DMA_Top_Config."]
    pub dma_top_config: crate::Reg<dma_top_config::DMA_TOP_CONFIG_SPEC>,
    #[doc = "0x34 - DMA_Sync."]
    pub dma_sync: crate::Reg<dma_sync::DMA_SYNC_SPEC>,
    _reserved14: [u8; 0xc8],
    #[doc = "0x100 - DMA_C0SrcAddr."]
    pub dma_c0src_addr: crate::Reg<dma_c0src_addr::DMA_C0SRCADDR_SPEC>,
    #[doc = "0x104 - DMA_C0DstAddr."]
    pub dma_c0dst_addr: crate::Reg<dma_c0dst_addr::DMA_C0DSTADDR_SPEC>,
    #[doc = "0x108 - DMA_C0LLI."]
    pub dma_c0lli: crate::Reg<dma_c0lli::DMA_C0LLI_SPEC>,
    #[doc = "0x10c - DMA_C0Control."]
    pub dma_c0control: crate::Reg<dma_c0control::DMA_C0CONTROL_SPEC>,
    #[doc = "0x110 - DMA_C0Config."]
    pub dma_c0config: crate::Reg<dma_c0config::DMA_C0CONFIG_SPEC>,
    _reserved19: [u8; 0xec],
    #[doc = "0x200 - DMA_C1SrcAddr."]
    pub dma_c1src_addr: crate::Reg<dma_c1src_addr::DMA_C1SRCADDR_SPEC>,
    #[doc = "0x204 - DMA_C1DstAddr."]
    pub dma_c1dst_addr: crate::Reg<dma_c1dst_addr::DMA_C1DSTADDR_SPEC>,
    #[doc = "0x208 - DMA_C1LLI."]
    pub dma_c1lli: crate::Reg<dma_c1lli::DMA_C1LLI_SPEC>,
    #[doc = "0x20c - DMA_C1Control."]
    pub dma_c1control: crate::Reg<dma_c1control::DMA_C1CONTROL_SPEC>,
    #[doc = "0x210 - DMA_C1Config."]
    pub dma_c1config: crate::Reg<dma_c1config::DMA_C1CONFIG_SPEC>,
    _reserved24: [u8; 0xec],
    #[doc = "0x300 - DMA_C2SrcAddr."]
    pub dma_c2src_addr: crate::Reg<dma_c2src_addr::DMA_C2SRCADDR_SPEC>,
    #[doc = "0x304 - DMA_C2DstAddr."]
    pub dma_c2dst_addr: crate::Reg<dma_c2dst_addr::DMA_C2DSTADDR_SPEC>,
    #[doc = "0x308 - DMA_C2LLI."]
    pub dma_c2lli: crate::Reg<dma_c2lli::DMA_C2LLI_SPEC>,
    #[doc = "0x30c - DMA_C2Control."]
    pub dma_c2control: crate::Reg<dma_c2control::DMA_C2CONTROL_SPEC>,
    #[doc = "0x310 - DMA_C2Config."]
    pub dma_c2config: crate::Reg<dma_c2config::DMA_C2CONFIG_SPEC>,
    _reserved29: [u8; 0xec],
    #[doc = "0x400 - DMA_C3SrcAddr."]
    pub dma_c3src_addr: crate::Reg<dma_c3src_addr::DMA_C3SRCADDR_SPEC>,
    #[doc = "0x404 - DMA_C3DstAddr."]
    pub dma_c3dst_addr: crate::Reg<dma_c3dst_addr::DMA_C3DSTADDR_SPEC>,
    #[doc = "0x408 - DMA_C3LLI."]
    pub dma_c3lli: crate::Reg<dma_c3lli::DMA_C3LLI_SPEC>,
    #[doc = "0x40c - DMA_C3Control."]
    pub dma_c3control: crate::Reg<dma_c3control::DMA_C3CONTROL_SPEC>,
    #[doc = "0x410 - DMA_C3Config."]
    pub dma_c3config: crate::Reg<dma_c3config::DMA_C3CONFIG_SPEC>,
}
#[doc = "DMA_IntStatus register accessor: an alias for `Reg<DMA_INTSTATUS_SPEC>`"]
pub type DMA_INTSTATUS = crate::Reg<dma_int_status::DMA_INTSTATUS_SPEC>;
#[doc = "DMA_IntStatus."]
pub mod dma_int_status;
#[doc = "DMA_IntTCStatus register accessor: an alias for `Reg<DMA_INTTCSTATUS_SPEC>`"]
pub type DMA_INTTCSTATUS = crate::Reg<dma_int_tcstatus::DMA_INTTCSTATUS_SPEC>;
#[doc = "DMA_IntTCStatus."]
pub mod dma_int_tcstatus;
#[doc = "DMA_IntTCClear register accessor: an alias for `Reg<DMA_INTTCCLEAR_SPEC>`"]
pub type DMA_INTTCCLEAR = crate::Reg<dma_int_tcclear::DMA_INTTCCLEAR_SPEC>;
#[doc = "DMA_IntTCClear."]
pub mod dma_int_tcclear;
#[doc = "DMA_IntErrorStatus register accessor: an alias for `Reg<DMA_INTERRORSTATUS_SPEC>`"]
pub type DMA_INTERRORSTATUS = crate::Reg<dma_int_error_status::DMA_INTERRORSTATUS_SPEC>;
#[doc = "DMA_IntErrorStatus."]
pub mod dma_int_error_status;
#[doc = "DMA_IntErrClr register accessor: an alias for `Reg<DMA_INTERRCLR_SPEC>`"]
pub type DMA_INTERRCLR = crate::Reg<dma_int_err_clr::DMA_INTERRCLR_SPEC>;
#[doc = "DMA_IntErrClr."]
pub mod dma_int_err_clr;
#[doc = "DMA_RawIntTCStatus register accessor: an alias for `Reg<DMA_RAWINTTCSTATUS_SPEC>`"]
pub type DMA_RAWINTTCSTATUS = crate::Reg<dma_raw_int_tcstatus::DMA_RAWINTTCSTATUS_SPEC>;
#[doc = "DMA_RawIntTCStatus."]
pub mod dma_raw_int_tcstatus;
#[doc = "DMA_RawIntErrorStatus register accessor: an alias for `Reg<DMA_RAWINTERRORSTATUS_SPEC>`"]
pub type DMA_RAWINTERRORSTATUS = crate::Reg<dma_raw_int_error_status::DMA_RAWINTERRORSTATUS_SPEC>;
#[doc = "DMA_RawIntErrorStatus."]
pub mod dma_raw_int_error_status;
#[doc = "DMA_EnbldChns register accessor: an alias for `Reg<DMA_ENBLDCHNS_SPEC>`"]
pub type DMA_ENBLDCHNS = crate::Reg<dma_enbld_chns::DMA_ENBLDCHNS_SPEC>;
#[doc = "DMA_EnbldChns."]
pub mod dma_enbld_chns;
#[doc = "DMA_SoftBReq register accessor: an alias for `Reg<DMA_SOFTBREQ_SPEC>`"]
pub type DMA_SOFTBREQ = crate::Reg<dma_soft_breq::DMA_SOFTBREQ_SPEC>;
#[doc = "DMA_SoftBReq."]
pub mod dma_soft_breq;
#[doc = "DMA_SoftSReq register accessor: an alias for `Reg<DMA_SOFTSREQ_SPEC>`"]
pub type DMA_SOFTSREQ = crate::Reg<dma_soft_sreq::DMA_SOFTSREQ_SPEC>;
#[doc = "DMA_SoftSReq."]
pub mod dma_soft_sreq;
#[doc = "DMA_SoftLBReq register accessor: an alias for `Reg<DMA_SOFTLBREQ_SPEC>`"]
pub type DMA_SOFTLBREQ = crate::Reg<dma_soft_lbreq::DMA_SOFTLBREQ_SPEC>;
#[doc = "DMA_SoftLBReq."]
pub mod dma_soft_lbreq;
#[doc = "DMA_SoftLSReq register accessor: an alias for `Reg<DMA_SOFTLSREQ_SPEC>`"]
pub type DMA_SOFTLSREQ = crate::Reg<dma_soft_lsreq::DMA_SOFTLSREQ_SPEC>;
#[doc = "DMA_SoftLSReq."]
pub mod dma_soft_lsreq;
#[doc = "DMA_Top_Config register accessor: an alias for `Reg<DMA_TOP_CONFIG_SPEC>`"]
pub type DMA_TOP_CONFIG = crate::Reg<dma_top_config::DMA_TOP_CONFIG_SPEC>;
#[doc = "DMA_Top_Config."]
pub mod dma_top_config;
#[doc = "DMA_Sync register accessor: an alias for `Reg<DMA_SYNC_SPEC>`"]
pub type DMA_SYNC = crate::Reg<dma_sync::DMA_SYNC_SPEC>;
#[doc = "DMA_Sync."]
pub mod dma_sync;
#[doc = "DMA_C0SrcAddr register accessor: an alias for `Reg<DMA_C0SRCADDR_SPEC>`"]
pub type DMA_C0SRCADDR = crate::Reg<dma_c0src_addr::DMA_C0SRCADDR_SPEC>;
#[doc = "DMA_C0SrcAddr."]
pub mod dma_c0src_addr;
#[doc = "DMA_C0DstAddr register accessor: an alias for `Reg<DMA_C0DSTADDR_SPEC>`"]
pub type DMA_C0DSTADDR = crate::Reg<dma_c0dst_addr::DMA_C0DSTADDR_SPEC>;
#[doc = "DMA_C0DstAddr."]
pub mod dma_c0dst_addr;
#[doc = "DMA_C0LLI register accessor: an alias for `Reg<DMA_C0LLI_SPEC>`"]
pub type DMA_C0LLI = crate::Reg<dma_c0lli::DMA_C0LLI_SPEC>;
#[doc = "DMA_C0LLI."]
pub mod dma_c0lli;
#[doc = "DMA_C0Control register accessor: an alias for `Reg<DMA_C0CONTROL_SPEC>`"]
pub type DMA_C0CONTROL = crate::Reg<dma_c0control::DMA_C0CONTROL_SPEC>;
#[doc = "DMA_C0Control."]
pub mod dma_c0control;
#[doc = "DMA_C0Config register accessor: an alias for `Reg<DMA_C0CONFIG_SPEC>`"]
pub type DMA_C0CONFIG = crate::Reg<dma_c0config::DMA_C0CONFIG_SPEC>;
#[doc = "DMA_C0Config."]
pub mod dma_c0config;
#[doc = "DMA_C1SrcAddr register accessor: an alias for `Reg<DMA_C1SRCADDR_SPEC>`"]
pub type DMA_C1SRCADDR = crate::Reg<dma_c1src_addr::DMA_C1SRCADDR_SPEC>;
#[doc = "DMA_C1SrcAddr."]
pub mod dma_c1src_addr;
#[doc = "DMA_C1DstAddr register accessor: an alias for `Reg<DMA_C1DSTADDR_SPEC>`"]
pub type DMA_C1DSTADDR = crate::Reg<dma_c1dst_addr::DMA_C1DSTADDR_SPEC>;
#[doc = "DMA_C1DstAddr."]
pub mod dma_c1dst_addr;
#[doc = "DMA_C1LLI register accessor: an alias for `Reg<DMA_C1LLI_SPEC>`"]
pub type DMA_C1LLI = crate::Reg<dma_c1lli::DMA_C1LLI_SPEC>;
#[doc = "DMA_C1LLI."]
pub mod dma_c1lli;
#[doc = "DMA_C1Control register accessor: an alias for `Reg<DMA_C1CONTROL_SPEC>`"]
pub type DMA_C1CONTROL = crate::Reg<dma_c1control::DMA_C1CONTROL_SPEC>;
#[doc = "DMA_C1Control."]
pub mod dma_c1control;
#[doc = "DMA_C1Config register accessor: an alias for `Reg<DMA_C1CONFIG_SPEC>`"]
pub type DMA_C1CONFIG = crate::Reg<dma_c1config::DMA_C1CONFIG_SPEC>;
#[doc = "DMA_C1Config."]
pub mod dma_c1config;
#[doc = "DMA_C2SrcAddr register accessor: an alias for `Reg<DMA_C2SRCADDR_SPEC>`"]
pub type DMA_C2SRCADDR = crate::Reg<dma_c2src_addr::DMA_C2SRCADDR_SPEC>;
#[doc = "DMA_C2SrcAddr."]
pub mod dma_c2src_addr;
#[doc = "DMA_C2DstAddr register accessor: an alias for `Reg<DMA_C2DSTADDR_SPEC>`"]
pub type DMA_C2DSTADDR = crate::Reg<dma_c2dst_addr::DMA_C2DSTADDR_SPEC>;
#[doc = "DMA_C2DstAddr."]
pub mod dma_c2dst_addr;
#[doc = "DMA_C2LLI register accessor: an alias for `Reg<DMA_C2LLI_SPEC>`"]
pub type DMA_C2LLI = crate::Reg<dma_c2lli::DMA_C2LLI_SPEC>;
#[doc = "DMA_C2LLI."]
pub mod dma_c2lli;
#[doc = "DMA_C2Control register accessor: an alias for `Reg<DMA_C2CONTROL_SPEC>`"]
pub type DMA_C2CONTROL = crate::Reg<dma_c2control::DMA_C2CONTROL_SPEC>;
#[doc = "DMA_C2Control."]
pub mod dma_c2control;
#[doc = "DMA_C2Config register accessor: an alias for `Reg<DMA_C2CONFIG_SPEC>`"]
pub type DMA_C2CONFIG = crate::Reg<dma_c2config::DMA_C2CONFIG_SPEC>;
#[doc = "DMA_C2Config."]
pub mod dma_c2config;
#[doc = "DMA_C3SrcAddr register accessor: an alias for `Reg<DMA_C3SRCADDR_SPEC>`"]
pub type DMA_C3SRCADDR = crate::Reg<dma_c3src_addr::DMA_C3SRCADDR_SPEC>;
#[doc = "DMA_C3SrcAddr."]
pub mod dma_c3src_addr;
#[doc = "DMA_C3DstAddr register accessor: an alias for `Reg<DMA_C3DSTADDR_SPEC>`"]
pub type DMA_C3DSTADDR = crate::Reg<dma_c3dst_addr::DMA_C3DSTADDR_SPEC>;
#[doc = "DMA_C3DstAddr."]
pub mod dma_c3dst_addr;
#[doc = "DMA_C3LLI register accessor: an alias for `Reg<DMA_C3LLI_SPEC>`"]
pub type DMA_C3LLI = crate::Reg<dma_c3lli::DMA_C3LLI_SPEC>;
#[doc = "DMA_C3LLI."]
pub mod dma_c3lli;
#[doc = "DMA_C3Control register accessor: an alias for `Reg<DMA_C3CONTROL_SPEC>`"]
pub type DMA_C3CONTROL = crate::Reg<dma_c3control::DMA_C3CONTROL_SPEC>;
#[doc = "DMA_C3Control."]
pub mod dma_c3control;
#[doc = "DMA_C3Config register accessor: an alias for `Reg<DMA_C3CONFIG_SPEC>`"]
pub type DMA_C3CONFIG = crate::Reg<dma_c3config::DMA_C3CONFIG_SPEC>;
#[doc = "DMA_C3Config."]
pub mod dma_c3config;
