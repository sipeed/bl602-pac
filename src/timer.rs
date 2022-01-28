#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TCCR."]
    pub tccr: crate::Reg<tccr::TCCR_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - TMR2_0."]
    pub tmr2_0: crate::Reg<tmr2_0::TMR2_0_SPEC>,
    #[doc = "0x14 - TMR2_1."]
    pub tmr2_1: crate::Reg<tmr2_1::TMR2_1_SPEC>,
    #[doc = "0x18 - TMR2_2."]
    pub tmr2_2: crate::Reg<tmr2_2::TMR2_2_SPEC>,
    #[doc = "0x1c - TMR3_0."]
    pub tmr3_0: crate::Reg<tmr3_0::TMR3_0_SPEC>,
    #[doc = "0x20 - TMR3_1."]
    pub tmr3_1: crate::Reg<tmr3_1::TMR3_1_SPEC>,
    #[doc = "0x24 - TMR3_2."]
    pub tmr3_2: crate::Reg<tmr3_2::TMR3_2_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x2c - TCR2."]
    pub tcr2: crate::Reg<tcr2::TCR2_SPEC>,
    #[doc = "0x30 - TCR3."]
    pub tcr3: crate::Reg<tcr3::TCR3_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x38 - TMSR2."]
    pub tmsr2: crate::Reg<tmsr2::TMSR2_SPEC>,
    #[doc = "0x3c - TMSR3."]
    pub tmsr3: crate::Reg<tmsr3::TMSR3_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x44 - TIER2."]
    pub tier2: crate::Reg<tier2::TIER2_SPEC>,
    #[doc = "0x48 - TIER3."]
    pub tier3: crate::Reg<tier3::TIER3_SPEC>,
    _reserved13: [u8; 0x04],
    #[doc = "0x50 - TPLVR2."]
    pub tplvr2: crate::Reg<tplvr2::TPLVR2_SPEC>,
    #[doc = "0x54 - TPLVR3."]
    pub tplvr3: crate::Reg<tplvr3::TPLVR3_SPEC>,
    _reserved15: [u8; 0x04],
    #[doc = "0x5c - TPLCR2."]
    pub tplcr2: crate::Reg<tplcr2::TPLCR2_SPEC>,
    #[doc = "0x60 - TPLCR3."]
    pub tplcr3: crate::Reg<tplcr3::TPLCR3_SPEC>,
    #[doc = "0x64 - WMER."]
    pub wmer: crate::Reg<wmer::WMER_SPEC>,
    #[doc = "0x68 - WMR."]
    pub wmr: crate::Reg<wmr::WMR_SPEC>,
    #[doc = "0x6c - WVR."]
    pub wvr: crate::Reg<wvr::WVR_SPEC>,
    #[doc = "0x70 - WSR."]
    pub wsr: crate::Reg<wsr::WSR_SPEC>,
    _reserved21: [u8; 0x04],
    #[doc = "0x78 - TICR2."]
    pub ticr2: crate::Reg<ticr2::TICR2_SPEC>,
    #[doc = "0x7c - TICR3."]
    pub ticr3: crate::Reg<ticr3::TICR3_SPEC>,
    #[doc = "0x80 - WICR."]
    pub wicr: crate::Reg<wicr::WICR_SPEC>,
    #[doc = "0x84 - TCER."]
    pub tcer: crate::Reg<tcer::TCER_SPEC>,
    #[doc = "0x88 - TCMR."]
    pub tcmr: crate::Reg<tcmr::TCMR_SPEC>,
    _reserved26: [u8; 0x04],
    #[doc = "0x90 - TILR2."]
    pub tilr2: crate::Reg<tilr2::TILR2_SPEC>,
    #[doc = "0x94 - TILR3."]
    pub tilr3: crate::Reg<tilr3::TILR3_SPEC>,
    #[doc = "0x98 - WCR."]
    pub wcr: crate::Reg<wcr::WCR_SPEC>,
    #[doc = "0x9c - WFAR."]
    pub wfar: crate::Reg<wfar::WFAR_SPEC>,
    #[doc = "0xa0 - WSAR."]
    pub wsar: crate::Reg<wsar::WSAR_SPEC>,
    _reserved31: [u8; 0x04],
    #[doc = "0xa8 - TCVWR2."]
    pub tcvwr2: crate::Reg<tcvwr2::TCVWR2_SPEC>,
    #[doc = "0xac - TCVWR3."]
    pub tcvwr3: crate::Reg<tcvwr3::TCVWR3_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0xb4 - TCVSYN2."]
    pub tcvsyn2: crate::Reg<tcvsyn2::TCVSYN2_SPEC>,
    #[doc = "0xb8 - TCVSYN3."]
    pub tcvsyn3: crate::Reg<tcvsyn3::TCVSYN3_SPEC>,
    #[doc = "0xbc - TCDR."]
    pub tcdr: crate::Reg<tcdr::TCDR_SPEC>,
}
#[doc = "TCCR register accessor: an alias for `Reg<TCCR_SPEC>`"]
pub type TCCR = crate::Reg<tccr::TCCR_SPEC>;
#[doc = "TCCR."]
pub mod tccr;
#[doc = "TMR2_0 register accessor: an alias for `Reg<TMR2_0_SPEC>`"]
pub type TMR2_0 = crate::Reg<tmr2_0::TMR2_0_SPEC>;
#[doc = "TMR2_0."]
pub mod tmr2_0;
#[doc = "TMR2_1 register accessor: an alias for `Reg<TMR2_1_SPEC>`"]
pub type TMR2_1 = crate::Reg<tmr2_1::TMR2_1_SPEC>;
#[doc = "TMR2_1."]
pub mod tmr2_1;
#[doc = "TMR2_2 register accessor: an alias for `Reg<TMR2_2_SPEC>`"]
pub type TMR2_2 = crate::Reg<tmr2_2::TMR2_2_SPEC>;
#[doc = "TMR2_2."]
pub mod tmr2_2;
#[doc = "TMR3_0 register accessor: an alias for `Reg<TMR3_0_SPEC>`"]
pub type TMR3_0 = crate::Reg<tmr3_0::TMR3_0_SPEC>;
#[doc = "TMR3_0."]
pub mod tmr3_0;
#[doc = "TMR3_1 register accessor: an alias for `Reg<TMR3_1_SPEC>`"]
pub type TMR3_1 = crate::Reg<tmr3_1::TMR3_1_SPEC>;
#[doc = "TMR3_1."]
pub mod tmr3_1;
#[doc = "TMR3_2 register accessor: an alias for `Reg<TMR3_2_SPEC>`"]
pub type TMR3_2 = crate::Reg<tmr3_2::TMR3_2_SPEC>;
#[doc = "TMR3_2."]
pub mod tmr3_2;
#[doc = "TCR2 register accessor: an alias for `Reg<TCR2_SPEC>`"]
pub type TCR2 = crate::Reg<tcr2::TCR2_SPEC>;
#[doc = "TCR2."]
pub mod tcr2;
#[doc = "TCR3 register accessor: an alias for `Reg<TCR3_SPEC>`"]
pub type TCR3 = crate::Reg<tcr3::TCR3_SPEC>;
#[doc = "TCR3."]
pub mod tcr3;
#[doc = "TMSR2 register accessor: an alias for `Reg<TMSR2_SPEC>`"]
pub type TMSR2 = crate::Reg<tmsr2::TMSR2_SPEC>;
#[doc = "TMSR2."]
pub mod tmsr2;
#[doc = "TMSR3 register accessor: an alias for `Reg<TMSR3_SPEC>`"]
pub type TMSR3 = crate::Reg<tmsr3::TMSR3_SPEC>;
#[doc = "TMSR3."]
pub mod tmsr3;
#[doc = "TIER2 register accessor: an alias for `Reg<TIER2_SPEC>`"]
pub type TIER2 = crate::Reg<tier2::TIER2_SPEC>;
#[doc = "TIER2."]
pub mod tier2;
#[doc = "TIER3 register accessor: an alias for `Reg<TIER3_SPEC>`"]
pub type TIER3 = crate::Reg<tier3::TIER3_SPEC>;
#[doc = "TIER3."]
pub mod tier3;
#[doc = "TPLVR2 register accessor: an alias for `Reg<TPLVR2_SPEC>`"]
pub type TPLVR2 = crate::Reg<tplvr2::TPLVR2_SPEC>;
#[doc = "TPLVR2."]
pub mod tplvr2;
#[doc = "TPLVR3 register accessor: an alias for `Reg<TPLVR3_SPEC>`"]
pub type TPLVR3 = crate::Reg<tplvr3::TPLVR3_SPEC>;
#[doc = "TPLVR3."]
pub mod tplvr3;
#[doc = "TPLCR2 register accessor: an alias for `Reg<TPLCR2_SPEC>`"]
pub type TPLCR2 = crate::Reg<tplcr2::TPLCR2_SPEC>;
#[doc = "TPLCR2."]
pub mod tplcr2;
#[doc = "TPLCR3 register accessor: an alias for `Reg<TPLCR3_SPEC>`"]
pub type TPLCR3 = crate::Reg<tplcr3::TPLCR3_SPEC>;
#[doc = "TPLCR3."]
pub mod tplcr3;
#[doc = "WMER register accessor: an alias for `Reg<WMER_SPEC>`"]
pub type WMER = crate::Reg<wmer::WMER_SPEC>;
#[doc = "WMER."]
pub mod wmer;
#[doc = "WMR register accessor: an alias for `Reg<WMR_SPEC>`"]
pub type WMR = crate::Reg<wmr::WMR_SPEC>;
#[doc = "WMR."]
pub mod wmr;
#[doc = "WVR register accessor: an alias for `Reg<WVR_SPEC>`"]
pub type WVR = crate::Reg<wvr::WVR_SPEC>;
#[doc = "WVR."]
pub mod wvr;
#[doc = "WSR register accessor: an alias for `Reg<WSR_SPEC>`"]
pub type WSR = crate::Reg<wsr::WSR_SPEC>;
#[doc = "WSR."]
pub mod wsr;
#[doc = "TICR2 register accessor: an alias for `Reg<TICR2_SPEC>`"]
pub type TICR2 = crate::Reg<ticr2::TICR2_SPEC>;
#[doc = "TICR2."]
pub mod ticr2;
#[doc = "TICR3 register accessor: an alias for `Reg<TICR3_SPEC>`"]
pub type TICR3 = crate::Reg<ticr3::TICR3_SPEC>;
#[doc = "TICR3."]
pub mod ticr3;
#[doc = "WICR register accessor: an alias for `Reg<WICR_SPEC>`"]
pub type WICR = crate::Reg<wicr::WICR_SPEC>;
#[doc = "WICR."]
pub mod wicr;
#[doc = "TCER register accessor: an alias for `Reg<TCER_SPEC>`"]
pub type TCER = crate::Reg<tcer::TCER_SPEC>;
#[doc = "TCER."]
pub mod tcer;
#[doc = "TCMR register accessor: an alias for `Reg<TCMR_SPEC>`"]
pub type TCMR = crate::Reg<tcmr::TCMR_SPEC>;
#[doc = "TCMR."]
pub mod tcmr;
#[doc = "TILR2 register accessor: an alias for `Reg<TILR2_SPEC>`"]
pub type TILR2 = crate::Reg<tilr2::TILR2_SPEC>;
#[doc = "TILR2."]
pub mod tilr2;
#[doc = "TILR3 register accessor: an alias for `Reg<TILR3_SPEC>`"]
pub type TILR3 = crate::Reg<tilr3::TILR3_SPEC>;
#[doc = "TILR3."]
pub mod tilr3;
#[doc = "WCR register accessor: an alias for `Reg<WCR_SPEC>`"]
pub type WCR = crate::Reg<wcr::WCR_SPEC>;
#[doc = "WCR."]
pub mod wcr;
#[doc = "WFAR register accessor: an alias for `Reg<WFAR_SPEC>`"]
pub type WFAR = crate::Reg<wfar::WFAR_SPEC>;
#[doc = "WFAR."]
pub mod wfar;
#[doc = "WSAR register accessor: an alias for `Reg<WSAR_SPEC>`"]
pub type WSAR = crate::Reg<wsar::WSAR_SPEC>;
#[doc = "WSAR."]
pub mod wsar;
#[doc = "TCVWR2 register accessor: an alias for `Reg<TCVWR2_SPEC>`"]
pub type TCVWR2 = crate::Reg<tcvwr2::TCVWR2_SPEC>;
#[doc = "TCVWR2."]
pub mod tcvwr2;
#[doc = "TCVWR3 register accessor: an alias for `Reg<TCVWR3_SPEC>`"]
pub type TCVWR3 = crate::Reg<tcvwr3::TCVWR3_SPEC>;
#[doc = "TCVWR3."]
pub mod tcvwr3;
#[doc = "TCVSYN2 register accessor: an alias for `Reg<TCVSYN2_SPEC>`"]
pub type TCVSYN2 = crate::Reg<tcvsyn2::TCVSYN2_SPEC>;
#[doc = "TCVSYN2."]
pub mod tcvsyn2;
#[doc = "TCVSYN3 register accessor: an alias for `Reg<TCVSYN3_SPEC>`"]
pub type TCVSYN3 = crate::Reg<tcvsyn3::TCVSYN3_SPEC>;
#[doc = "TCVSYN3."]
pub mod tcvsyn3;
#[doc = "TCDR register accessor: an alias for `Reg<TCDR_SPEC>`"]
pub type TCDR = crate::Reg<tcdr::TCDR_SPEC>;
#[doc = "TCDR."]
pub mod tcdr;
