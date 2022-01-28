#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - HBN_CTL."]
    pub hbn_ctl: crate::Reg<hbn_ctl::HBN_CTL_SPEC>,
    #[doc = "0x04 - HBN_TIME_L."]
    pub hbn_time_l: crate::Reg<hbn_time_l::HBN_TIME_L_SPEC>,
    #[doc = "0x08 - HBN_TIME_H."]
    pub hbn_time_h: crate::Reg<hbn_time_h::HBN_TIME_H_SPEC>,
    #[doc = "0x0c - RTC_TIME_L."]
    pub rtc_time_l: crate::Reg<rtc_time_l::RTC_TIME_L_SPEC>,
    #[doc = "0x10 - RTC_TIME_H."]
    pub rtc_time_h: crate::Reg<rtc_time_h::RTC_TIME_H_SPEC>,
    #[doc = "0x14 - HBN_IRQ_MODE."]
    pub hbn_irq_mode: crate::Reg<hbn_irq_mode::HBN_IRQ_MODE_SPEC>,
    #[doc = "0x18 - HBN_IRQ_STAT."]
    pub hbn_irq_stat: crate::Reg<hbn_irq_stat::HBN_IRQ_STAT_SPEC>,
    #[doc = "0x1c - HBN_IRQ_CLR."]
    pub hbn_irq_clr: crate::Reg<hbn_irq_clr::HBN_IRQ_CLR_SPEC>,
    #[doc = "0x20 - HBN_PIR_CFG."]
    pub hbn_pir_cfg: crate::Reg<hbn_pir_cfg::HBN_PIR_CFG_SPEC>,
    #[doc = "0x24 - HBN_PIR_VTH."]
    pub hbn_pir_vth: crate::Reg<hbn_pir_vth::HBN_PIR_VTH_SPEC>,
    #[doc = "0x28 - HBN_PIR_INTERVAL."]
    pub hbn_pir_interval: crate::Reg<hbn_pir_interval::HBN_PIR_INTERVAL_SPEC>,
    #[doc = "0x2c - HBN_BOR_CFG."]
    pub hbn_bor_cfg: crate::Reg<hbn_bor_cfg::HBN_BOR_CFG_SPEC>,
    #[doc = "0x30 - HBN_GLB."]
    pub hbn_glb: crate::Reg<hbn_glb::HBN_GLB_SPEC>,
    #[doc = "0x34 - HBN_SRAM."]
    pub hbn_sram: crate::Reg<hbn_sram::HBN_SRAM_SPEC>,
    _reserved14: [u8; 0xc8],
    #[doc = "0x100 - HBN_RSV0."]
    pub hbn_rsv0: crate::Reg<hbn_rsv0::HBN_RSV0_SPEC>,
    #[doc = "0x104 - HBN_RSV1."]
    pub hbn_rsv1: crate::Reg<hbn_rsv1::HBN_RSV1_SPEC>,
    #[doc = "0x108 - HBN_RSV2."]
    pub hbn_rsv2: crate::Reg<hbn_rsv2::HBN_RSV2_SPEC>,
    #[doc = "0x10c - HBN_RSV3."]
    pub hbn_rsv3: crate::Reg<hbn_rsv3::HBN_RSV3_SPEC>,
    _reserved18: [u8; 0xf0],
    #[doc = "0x200 - rc32k_ctrl0."]
    pub rc32k_ctrl0: crate::Reg<rc32k_ctrl0::RC32K_CTRL0_SPEC>,
    #[doc = "0x204 - xtal32k."]
    pub xtal32k: crate::Reg<xtal32k::XTAL32K_SPEC>,
}
#[doc = "HBN_CTL register accessor: an alias for `Reg<HBN_CTL_SPEC>`"]
pub type HBN_CTL = crate::Reg<hbn_ctl::HBN_CTL_SPEC>;
#[doc = "HBN_CTL."]
pub mod hbn_ctl;
#[doc = "HBN_TIME_L register accessor: an alias for `Reg<HBN_TIME_L_SPEC>`"]
pub type HBN_TIME_L = crate::Reg<hbn_time_l::HBN_TIME_L_SPEC>;
#[doc = "HBN_TIME_L."]
pub mod hbn_time_l;
#[doc = "HBN_TIME_H register accessor: an alias for `Reg<HBN_TIME_H_SPEC>`"]
pub type HBN_TIME_H = crate::Reg<hbn_time_h::HBN_TIME_H_SPEC>;
#[doc = "HBN_TIME_H."]
pub mod hbn_time_h;
#[doc = "RTC_TIME_L register accessor: an alias for `Reg<RTC_TIME_L_SPEC>`"]
pub type RTC_TIME_L = crate::Reg<rtc_time_l::RTC_TIME_L_SPEC>;
#[doc = "RTC_TIME_L."]
pub mod rtc_time_l;
#[doc = "RTC_TIME_H register accessor: an alias for `Reg<RTC_TIME_H_SPEC>`"]
pub type RTC_TIME_H = crate::Reg<rtc_time_h::RTC_TIME_H_SPEC>;
#[doc = "RTC_TIME_H."]
pub mod rtc_time_h;
#[doc = "HBN_IRQ_MODE register accessor: an alias for `Reg<HBN_IRQ_MODE_SPEC>`"]
pub type HBN_IRQ_MODE = crate::Reg<hbn_irq_mode::HBN_IRQ_MODE_SPEC>;
#[doc = "HBN_IRQ_MODE."]
pub mod hbn_irq_mode;
#[doc = "HBN_IRQ_STAT register accessor: an alias for `Reg<HBN_IRQ_STAT_SPEC>`"]
pub type HBN_IRQ_STAT = crate::Reg<hbn_irq_stat::HBN_IRQ_STAT_SPEC>;
#[doc = "HBN_IRQ_STAT."]
pub mod hbn_irq_stat;
#[doc = "HBN_IRQ_CLR register accessor: an alias for `Reg<HBN_IRQ_CLR_SPEC>`"]
pub type HBN_IRQ_CLR = crate::Reg<hbn_irq_clr::HBN_IRQ_CLR_SPEC>;
#[doc = "HBN_IRQ_CLR."]
pub mod hbn_irq_clr;
#[doc = "HBN_PIR_CFG register accessor: an alias for `Reg<HBN_PIR_CFG_SPEC>`"]
pub type HBN_PIR_CFG = crate::Reg<hbn_pir_cfg::HBN_PIR_CFG_SPEC>;
#[doc = "HBN_PIR_CFG."]
pub mod hbn_pir_cfg;
#[doc = "HBN_PIR_VTH register accessor: an alias for `Reg<HBN_PIR_VTH_SPEC>`"]
pub type HBN_PIR_VTH = crate::Reg<hbn_pir_vth::HBN_PIR_VTH_SPEC>;
#[doc = "HBN_PIR_VTH."]
pub mod hbn_pir_vth;
#[doc = "HBN_PIR_INTERVAL register accessor: an alias for `Reg<HBN_PIR_INTERVAL_SPEC>`"]
pub type HBN_PIR_INTERVAL = crate::Reg<hbn_pir_interval::HBN_PIR_INTERVAL_SPEC>;
#[doc = "HBN_PIR_INTERVAL."]
pub mod hbn_pir_interval;
#[doc = "HBN_BOR_CFG register accessor: an alias for `Reg<HBN_BOR_CFG_SPEC>`"]
pub type HBN_BOR_CFG = crate::Reg<hbn_bor_cfg::HBN_BOR_CFG_SPEC>;
#[doc = "HBN_BOR_CFG."]
pub mod hbn_bor_cfg;
#[doc = "HBN_GLB register accessor: an alias for `Reg<HBN_GLB_SPEC>`"]
pub type HBN_GLB = crate::Reg<hbn_glb::HBN_GLB_SPEC>;
#[doc = "HBN_GLB."]
pub mod hbn_glb;
#[doc = "HBN_SRAM register accessor: an alias for `Reg<HBN_SRAM_SPEC>`"]
pub type HBN_SRAM = crate::Reg<hbn_sram::HBN_SRAM_SPEC>;
#[doc = "HBN_SRAM."]
pub mod hbn_sram;
#[doc = "HBN_RSV0 register accessor: an alias for `Reg<HBN_RSV0_SPEC>`"]
pub type HBN_RSV0 = crate::Reg<hbn_rsv0::HBN_RSV0_SPEC>;
#[doc = "HBN_RSV0."]
pub mod hbn_rsv0;
#[doc = "HBN_RSV1 register accessor: an alias for `Reg<HBN_RSV1_SPEC>`"]
pub type HBN_RSV1 = crate::Reg<hbn_rsv1::HBN_RSV1_SPEC>;
#[doc = "HBN_RSV1."]
pub mod hbn_rsv1;
#[doc = "HBN_RSV2 register accessor: an alias for `Reg<HBN_RSV2_SPEC>`"]
pub type HBN_RSV2 = crate::Reg<hbn_rsv2::HBN_RSV2_SPEC>;
#[doc = "HBN_RSV2."]
pub mod hbn_rsv2;
#[doc = "HBN_RSV3 register accessor: an alias for `Reg<HBN_RSV3_SPEC>`"]
pub type HBN_RSV3 = crate::Reg<hbn_rsv3::HBN_RSV3_SPEC>;
#[doc = "HBN_RSV3."]
pub mod hbn_rsv3;
#[doc = "rc32k_ctrl0 register accessor: an alias for `Reg<RC32K_CTRL0_SPEC>`"]
pub type RC32K_CTRL0 = crate::Reg<rc32k_ctrl0::RC32K_CTRL0_SPEC>;
#[doc = "rc32k_ctrl0."]
pub mod rc32k_ctrl0;
#[doc = "xtal32k register accessor: an alias for `Reg<XTAL32K_SPEC>`"]
pub type XTAL32K = crate::Reg<xtal32k::XTAL32K_SPEC>;
#[doc = "xtal32k."]
pub mod xtal32k;
