#[doc = "Register `tzc_glb_ctrl_0` reader"]
pub struct R(crate::R<TZC_GLB_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_GLB_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TZC_GLB_CTRL_0_SPEC>> for R {
    fn from(reader: crate::R<TZC_GLB_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tzc_glb_clk_lock` reader - "]
pub struct TZC_GLB_CLK_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_CLK_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_CLK_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_CLK_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_mbist_lock` reader - "]
pub struct TZC_GLB_MBIST_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_MBIST_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_MBIST_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_MBIST_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_dbg_lock` reader - "]
pub struct TZC_GLB_DBG_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_DBG_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_DBG_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_DBG_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_bmx_lock` reader - "]
pub struct TZC_GLB_BMX_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_BMX_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_BMX_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_BMX_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_l2c_lock` reader - "]
pub struct TZC_GLB_L2C_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_L2C_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_L2C_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_L2C_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_sram_lock` reader - "]
pub struct TZC_GLB_SRAM_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_SRAM_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_SRAM_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_SRAM_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_misc_lock` reader - "]
pub struct TZC_GLB_MISC_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_MISC_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_MISC_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_MISC_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_ctrl_ungated_ap_lock` reader - "]
pub struct TZC_GLB_CTRL_UNGATED_AP_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_CTRL_UNGATED_AP_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_CTRL_UNGATED_AP_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_CTRL_UNGATED_AP_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_ctrl_sys_reset_lock` reader - "]
pub struct TZC_GLB_CTRL_SYS_RESET_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_CTRL_SYS_RESET_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_CTRL_SYS_RESET_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_CTRL_SYS_RESET_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_ctrl_cpu_reset_lock` reader - "]
pub struct TZC_GLB_CTRL_CPU_RESET_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_CTRL_CPU_RESET_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_CTRL_CPU_RESET_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_CTRL_CPU_RESET_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_ctrl_pwron_rst_lock` reader - "]
pub struct TZC_GLB_CTRL_PWRON_RST_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_CTRL_PWRON_RST_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_CTRL_PWRON_RST_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_CTRL_PWRON_RST_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_swrst_s30_lock` reader - "]
pub struct TZC_GLB_SWRST_S30_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_SWRST_S30_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_SWRST_S30_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_SWRST_S30_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_swrst_s01_lock` reader - "]
pub struct TZC_GLB_SWRST_S01_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_SWRST_S01_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_SWRST_S01_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_SWRST_S01_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_swrst_s00_lock` reader - "]
pub struct TZC_GLB_SWRST_S00_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_SWRST_S00_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_SWRST_S00_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_SWRST_S00_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_clk_lock(&self) -> TZC_GLB_CLK_LOCK_R {
        TZC_GLB_CLK_LOCK_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_mbist_lock(&self) -> TZC_GLB_MBIST_LOCK_R {
        TZC_GLB_MBIST_LOCK_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_dbg_lock(&self) -> TZC_GLB_DBG_LOCK_R {
        TZC_GLB_DBG_LOCK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_bmx_lock(&self) -> TZC_GLB_BMX_LOCK_R {
        TZC_GLB_BMX_LOCK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_l2c_lock(&self) -> TZC_GLB_L2C_LOCK_R {
        TZC_GLB_L2C_LOCK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_sram_lock(&self) -> TZC_GLB_SRAM_LOCK_R {
        TZC_GLB_SRAM_LOCK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_misc_lock(&self) -> TZC_GLB_MISC_LOCK_R {
        TZC_GLB_MISC_LOCK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_ungated_ap_lock(&self) -> TZC_GLB_CTRL_UNGATED_AP_LOCK_R {
        TZC_GLB_CTRL_UNGATED_AP_LOCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_sys_reset_lock(&self) -> TZC_GLB_CTRL_SYS_RESET_LOCK_R {
        TZC_GLB_CTRL_SYS_RESET_LOCK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_cpu_reset_lock(&self) -> TZC_GLB_CTRL_CPU_RESET_LOCK_R {
        TZC_GLB_CTRL_CPU_RESET_LOCK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_pwron_rst_lock(&self) -> TZC_GLB_CTRL_PWRON_RST_LOCK_R {
        TZC_GLB_CTRL_PWRON_RST_LOCK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s30_lock(&self) -> TZC_GLB_SWRST_S30_LOCK_R {
        TZC_GLB_SWRST_S30_LOCK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s01_lock(&self) -> TZC_GLB_SWRST_S01_LOCK_R {
        TZC_GLB_SWRST_S01_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s00_lock(&self) -> TZC_GLB_SWRST_S00_LOCK_R {
        TZC_GLB_SWRST_S00_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "tzc_glb_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_0](index.html) module"]
pub struct TZC_GLB_CTRL_0_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_glb_ctrl_0::R](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_0 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
