#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA_IntStatus."]
    pub dma_int_status: DMA_INT_STATUS,
    #[doc = "0x04 - DMA_IntTCStatus."]
    pub dma_int_tcstatus: DMA_INT_TCSTATUS,
    #[doc = "0x08 - DMA_IntTCClear."]
    pub dma_int_tcclear: DMA_INT_TCCLEAR,
    #[doc = "0x0c - DMA_IntErrorStatus."]
    pub dma_int_error_status: DMA_INT_ERROR_STATUS,
    #[doc = "0x10 - DMA_IntErrClr."]
    pub dma_int_err_clr: DMA_INT_ERR_CLR,
    #[doc = "0x14 - DMA_RawIntTCStatus."]
    pub dma_raw_int_tcstatus: DMA_RAW_INT_TCSTATUS,
    #[doc = "0x18 - DMA_RawIntErrorStatus."]
    pub dma_raw_int_error_status: DMA_RAW_INT_ERROR_STATUS,
    #[doc = "0x1c - DMA_EnbldChns."]
    pub dma_enbld_chns: DMA_ENBLD_CHNS,
    #[doc = "0x20 - DMA_SoftBReq."]
    pub dma_soft_breq: DMA_SOFT_BREQ,
    #[doc = "0x24 - DMA_SoftSReq."]
    pub dma_soft_sreq: DMA_SOFT_SREQ,
    #[doc = "0x28 - DMA_SoftLBReq."]
    pub dma_soft_lbreq: DMA_SOFT_LBREQ,
    #[doc = "0x2c - DMA_SoftLSReq."]
    pub dma_soft_lsreq: DMA_SOFT_LSREQ,
    #[doc = "0x30 - DMA_Top_Config."]
    pub dma_top_config: DMA_TOP_CONFIG,
    #[doc = "0x34 - DMA_Sync."]
    pub dma_sync: DMA_SYNC,
    _reserved14: [u8; 0xc8],
    #[doc = "0x100..0x114 - Cluster CH%s, containing SrcAddr, DstAddr, LLI, Control, Config"]
    pub ch0: CH,
    _reserved15: [u8; 0xec],
    #[doc = "0x200..0x214 - Cluster CH%s, containing SrcAddr, DstAddr, LLI, Control, Config"]
    pub ch1: CH,
    _reserved16: [u8; 0xec],
    #[doc = "0x300..0x314 - Cluster CH%s, containing SrcAddr, DstAddr, LLI, Control, Config"]
    pub ch2: CH,
    _reserved17: [u8; 0xec],
    #[doc = "0x400..0x414 - Cluster CH%s, containing SrcAddr, DstAddr, LLI, Control, Config"]
    pub ch3: CH,
}
#[doc = "DMA_IntStatus (rw) register accessor: an alias for `Reg<DMA_INT_STATUS_SPEC>`"]
pub type DMA_INT_STATUS = crate::Reg<dma_int_status::DMA_INT_STATUS_SPEC>;
#[doc = "DMA_IntStatus."]
pub mod dma_int_status;
#[doc = "DMA_IntTCStatus (rw) register accessor: an alias for `Reg<DMA_INT_TCSTATUS_SPEC>`"]
pub type DMA_INT_TCSTATUS = crate::Reg<dma_int_tcstatus::DMA_INT_TCSTATUS_SPEC>;
#[doc = "DMA_IntTCStatus."]
pub mod dma_int_tcstatus;
#[doc = "DMA_IntTCClear (rw) register accessor: an alias for `Reg<DMA_INT_TCCLEAR_SPEC>`"]
pub type DMA_INT_TCCLEAR = crate::Reg<dma_int_tcclear::DMA_INT_TCCLEAR_SPEC>;
#[doc = "DMA_IntTCClear."]
pub mod dma_int_tcclear;
#[doc = "DMA_IntErrorStatus (rw) register accessor: an alias for `Reg<DMA_INT_ERROR_STATUS_SPEC>`"]
pub type DMA_INT_ERROR_STATUS = crate::Reg<dma_int_error_status::DMA_INT_ERROR_STATUS_SPEC>;
#[doc = "DMA_IntErrorStatus."]
pub mod dma_int_error_status;
#[doc = "DMA_IntErrClr (rw) register accessor: an alias for `Reg<DMA_INT_ERR_CLR_SPEC>`"]
pub type DMA_INT_ERR_CLR = crate::Reg<dma_int_err_clr::DMA_INT_ERR_CLR_SPEC>;
#[doc = "DMA_IntErrClr."]
pub mod dma_int_err_clr;
#[doc = "DMA_RawIntTCStatus (rw) register accessor: an alias for `Reg<DMA_RAW_INT_TCSTATUS_SPEC>`"]
pub type DMA_RAW_INT_TCSTATUS = crate::Reg<dma_raw_int_tcstatus::DMA_RAW_INT_TCSTATUS_SPEC>;
#[doc = "DMA_RawIntTCStatus."]
pub mod dma_raw_int_tcstatus;
#[doc = "DMA_RawIntErrorStatus (rw) register accessor: an alias for `Reg<DMA_RAW_INT_ERROR_STATUS_SPEC>`"]
pub type DMA_RAW_INT_ERROR_STATUS =
    crate::Reg<dma_raw_int_error_status::DMA_RAW_INT_ERROR_STATUS_SPEC>;
#[doc = "DMA_RawIntErrorStatus."]
pub mod dma_raw_int_error_status;
#[doc = "DMA_EnbldChns (rw) register accessor: an alias for `Reg<DMA_ENBLD_CHNS_SPEC>`"]
pub type DMA_ENBLD_CHNS = crate::Reg<dma_enbld_chns::DMA_ENBLD_CHNS_SPEC>;
#[doc = "DMA_EnbldChns."]
pub mod dma_enbld_chns;
#[doc = "DMA_SoftBReq (rw) register accessor: an alias for `Reg<DMA_SOFT_BREQ_SPEC>`"]
pub type DMA_SOFT_BREQ = crate::Reg<dma_soft_breq::DMA_SOFT_BREQ_SPEC>;
#[doc = "DMA_SoftBReq."]
pub mod dma_soft_breq;
#[doc = "DMA_SoftSReq (rw) register accessor: an alias for `Reg<DMA_SOFT_SREQ_SPEC>`"]
pub type DMA_SOFT_SREQ = crate::Reg<dma_soft_sreq::DMA_SOFT_SREQ_SPEC>;
#[doc = "DMA_SoftSReq."]
pub mod dma_soft_sreq;
#[doc = "DMA_SoftLBReq (rw) register accessor: an alias for `Reg<DMA_SOFT_LBREQ_SPEC>`"]
pub type DMA_SOFT_LBREQ = crate::Reg<dma_soft_lbreq::DMA_SOFT_LBREQ_SPEC>;
#[doc = "DMA_SoftLBReq."]
pub mod dma_soft_lbreq;
#[doc = "DMA_SoftLSReq (rw) register accessor: an alias for `Reg<DMA_SOFT_LSREQ_SPEC>`"]
pub type DMA_SOFT_LSREQ = crate::Reg<dma_soft_lsreq::DMA_SOFT_LSREQ_SPEC>;
#[doc = "DMA_SoftLSReq."]
pub mod dma_soft_lsreq;
#[doc = "DMA_Top_Config (rw) register accessor: an alias for `Reg<DMA_TOP_CONFIG_SPEC>`"]
pub type DMA_TOP_CONFIG = crate::Reg<dma_top_config::DMA_TOP_CONFIG_SPEC>;
#[doc = "DMA_Top_Config."]
pub mod dma_top_config;
#[doc = "DMA_Sync (rw) register accessor: an alias for `Reg<DMA_SYNC_SPEC>`"]
pub type DMA_SYNC = crate::Reg<dma_sync::DMA_SYNC_SPEC>;
#[doc = "DMA_Sync."]
pub mod dma_sync;
#[doc = "Cluster CH%s, containing SrcAddr, DstAddr, LLI, Control, Config"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster CH%s, containing SrcAddr, DstAddr, LLI, Control, Config"]
pub mod ch;
