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
#[doc = "Register `tzc_glb_ctrl_0` writer"]
pub struct W(crate::W<TZC_GLB_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_GLB_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<TZC_GLB_CTRL_0_SPEC>> for W {
    fn from(writer: crate::W<TZC_GLB_CTRL_0_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `tzc_glb_clk_lock` writer - "]
pub struct TZC_GLB_CLK_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_CLK_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
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
#[doc = "Field `tzc_glb_mbist_lock` writer - "]
pub struct TZC_GLB_MBIST_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_MBIST_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
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
#[doc = "Field `tzc_glb_dbg_lock` writer - "]
pub struct TZC_GLB_DBG_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_DBG_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
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
#[doc = "Field `tzc_glb_bmx_lock` writer - "]
pub struct TZC_GLB_BMX_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_BMX_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
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
#[doc = "Field `tzc_glb_l2c_lock` writer - "]
pub struct TZC_GLB_L2C_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_L2C_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
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
#[doc = "Field `tzc_glb_sram_lock` writer - "]
pub struct TZC_GLB_SRAM_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SRAM_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
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
#[doc = "Field `tzc_glb_misc_lock` writer - "]
pub struct TZC_GLB_MISC_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_MISC_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
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
#[doc = "Field `tzc_glb_ctrl_ungated_ap_lock` writer - "]
pub struct TZC_GLB_CTRL_UNGATED_AP_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_CTRL_UNGATED_AP_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
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
#[doc = "Field `tzc_glb_ctrl_sys_reset_lock` writer - "]
pub struct TZC_GLB_CTRL_SYS_RESET_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_CTRL_SYS_RESET_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
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
#[doc = "Field `tzc_glb_ctrl_cpu_reset_lock` writer - "]
pub struct TZC_GLB_CTRL_CPU_RESET_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_CTRL_CPU_RESET_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
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
#[doc = "Field `tzc_glb_ctrl_pwron_rst_lock` writer - "]
pub struct TZC_GLB_CTRL_PWRON_RST_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_CTRL_PWRON_RST_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
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
#[doc = "Field `tzc_glb_swrst_s30_lock` writer - "]
pub struct TZC_GLB_SWRST_S30_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S30_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
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
#[doc = "Field `tzc_glb_swrst_s01_lock` writer - "]
pub struct TZC_GLB_SWRST_S01_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S01_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
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
#[doc = "Field `tzc_glb_swrst_s00_lock` writer - "]
pub struct TZC_GLB_SWRST_S00_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_SWRST_S00_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
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
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn tzc_glb_clk_lock(&mut self) -> TZC_GLB_CLK_LOCK_W {
        TZC_GLB_CLK_LOCK_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn tzc_glb_mbist_lock(&mut self) -> TZC_GLB_MBIST_LOCK_W {
        TZC_GLB_MBIST_LOCK_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn tzc_glb_dbg_lock(&mut self) -> TZC_GLB_DBG_LOCK_W {
        TZC_GLB_DBG_LOCK_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_bmx_lock(&mut self) -> TZC_GLB_BMX_LOCK_W {
        TZC_GLB_BMX_LOCK_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_l2c_lock(&mut self) -> TZC_GLB_L2C_LOCK_W {
        TZC_GLB_L2C_LOCK_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_sram_lock(&mut self) -> TZC_GLB_SRAM_LOCK_W {
        TZC_GLB_SRAM_LOCK_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_misc_lock(&mut self) -> TZC_GLB_MISC_LOCK_W {
        TZC_GLB_MISC_LOCK_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_ungated_ap_lock(&mut self) -> TZC_GLB_CTRL_UNGATED_AP_LOCK_W {
        TZC_GLB_CTRL_UNGATED_AP_LOCK_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_sys_reset_lock(&mut self) -> TZC_GLB_CTRL_SYS_RESET_LOCK_W {
        TZC_GLB_CTRL_SYS_RESET_LOCK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_cpu_reset_lock(&mut self) -> TZC_GLB_CTRL_CPU_RESET_LOCK_W {
        TZC_GLB_CTRL_CPU_RESET_LOCK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_ctrl_pwron_rst_lock(&mut self) -> TZC_GLB_CTRL_PWRON_RST_LOCK_W {
        TZC_GLB_CTRL_PWRON_RST_LOCK_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s30_lock(&mut self) -> TZC_GLB_SWRST_S30_LOCK_W {
        TZC_GLB_SWRST_S30_LOCK_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s01_lock(&mut self) -> TZC_GLB_SWRST_S01_LOCK_W {
        TZC_GLB_SWRST_S01_LOCK_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_swrst_s00_lock(&mut self) -> TZC_GLB_SWRST_S00_LOCK_W {
        TZC_GLB_SWRST_S00_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_glb_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_0](index.html) module"]
pub struct TZC_GLB_CTRL_0_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_glb_ctrl_0::R](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_glb_ctrl_0::W](W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_0 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
