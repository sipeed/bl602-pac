#[doc = "Register `aon_common` reader"]
pub struct R(crate::R<AON_COMMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AON_COMMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AON_COMMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AON_COMMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `aon_common` writer"]
pub struct W(crate::W<AON_COMMON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AON_COMMON_SPEC>;
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
impl From<crate::W<AON_COMMON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AON_COMMON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ten_cip_misc_aon` reader - "]
pub struct TEN_CIP_MISC_AON_R(crate::FieldReader<bool, bool>);
impl TEN_CIP_MISC_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_CIP_MISC_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_CIP_MISC_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_cip_misc_aon` writer - "]
pub struct TEN_CIP_MISC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_CIP_MISC_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `ten_mbg_aon` reader - "]
pub struct TEN_MBG_AON_R(crate::FieldReader<bool, bool>);
impl TEN_MBG_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_MBG_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_MBG_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_mbg_aon` writer - "]
pub struct TEN_MBG_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_MBG_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `dten_xtal_aon` reader - "]
pub struct DTEN_XTAL_AON_R(crate::FieldReader<bool, bool>);
impl DTEN_XTAL_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_XTAL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_XTAL_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_xtal_aon` writer - "]
pub struct DTEN_XTAL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_XTAL_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `ten_xtal_aon` reader - "]
pub struct TEN_XTAL_AON_R(crate::FieldReader<bool, bool>);
impl TEN_XTAL_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_XTAL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_XTAL_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_xtal_aon` writer - "]
pub struct TEN_XTAL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_XTAL_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ten_ldo15rf_aon` reader - "]
pub struct TEN_LDO15RF_AON_R(crate::FieldReader<bool, bool>);
impl TEN_LDO15RF_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_LDO15RF_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_LDO15RF_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_ldo15rf_aon` writer - "]
pub struct TEN_LDO15RF_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_LDO15RF_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ten_bg_sys_aon` reader - "]
pub struct TEN_BG_SYS_AON_R(crate::FieldReader<bool, bool>);
impl TEN_BG_SYS_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_BG_SYS_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_BG_SYS_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_bg_sys_aon` writer - "]
pub struct TEN_BG_SYS_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_BG_SYS_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `ten_dcdc18_1_aon` reader - "]
pub struct TEN_DCDC18_1_AON_R(crate::FieldReader<bool, bool>);
impl TEN_DCDC18_1_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_DCDC18_1_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_DCDC18_1_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_dcdc18_1_aon` writer - "]
pub struct TEN_DCDC18_1_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_DCDC18_1_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `ten_dcdc18_0_aon` reader - "]
pub struct TEN_DCDC18_0_AON_R(crate::FieldReader<bool, bool>);
impl TEN_DCDC18_0_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_DCDC18_0_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_DCDC18_0_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_dcdc18_0_aon` writer - "]
pub struct TEN_DCDC18_0_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_DCDC18_0_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `ten_ldo11soc_aon` reader - "]
pub struct TEN_LDO11SOC_AON_R(crate::FieldReader<bool, bool>);
impl TEN_LDO11SOC_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_LDO11SOC_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_LDO11SOC_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_ldo11soc_aon` writer - "]
pub struct TEN_LDO11SOC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_LDO11SOC_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ten_vddcore_aon` reader - "]
pub struct TEN_VDDCORE_AON_R(crate::FieldReader<bool, bool>);
impl TEN_VDDCORE_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_VDDCORE_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_VDDCORE_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_vddcore_aon` writer - "]
pub struct TEN_VDDCORE_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_VDDCORE_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `ten_xtal32k` reader - "]
pub struct TEN_XTAL32K_R(crate::FieldReader<bool, bool>);
impl TEN_XTAL32K_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_XTAL32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_XTAL32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_xtal32k` writer - "]
pub struct TEN_XTAL32K_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_XTAL32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `dten_xtal32k` reader - "]
pub struct DTEN_XTAL32K_R(crate::FieldReader<bool, bool>);
impl DTEN_XTAL32K_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_XTAL32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_XTAL32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dten_xtal32k` writer - "]
pub struct DTEN_XTAL32K_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_XTAL32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ten_aon` reader - "]
pub struct TEN_AON_R(crate::FieldReader<bool, bool>);
impl TEN_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_aon` writer - "]
pub struct TEN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `tmux_aon` reader - "]
pub struct TMUX_AON_R(crate::FieldReader<u8, u8>);
impl TMUX_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMUX_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMUX_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmux_aon` writer - "]
pub struct TMUX_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> TMUX_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_cip_misc_aon(&self) -> TEN_CIP_MISC_AON_R {
        TEN_CIP_MISC_AON_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_mbg_aon(&self) -> TEN_MBG_AON_R {
        TEN_MBG_AON_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dten_xtal_aon(&self) -> DTEN_XTAL_AON_R {
        DTEN_XTAL_AON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_xtal_aon(&self) -> TEN_XTAL_AON_R {
        TEN_XTAL_AON_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_ldo15rf_aon(&self) -> TEN_LDO15RF_AON_R {
        TEN_LDO15RF_AON_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_bg_sys_aon(&self) -> TEN_BG_SYS_AON_R {
        TEN_BG_SYS_AON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ten_dcdc18_1_aon(&self) -> TEN_DCDC18_1_AON_R {
        TEN_DCDC18_1_AON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ten_dcdc18_0_aon(&self) -> TEN_DCDC18_0_AON_R {
        TEN_DCDC18_0_AON_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_ldo11soc_aon(&self) -> TEN_LDO11SOC_AON_R {
        TEN_LDO11SOC_AON_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_vddcore_aon(&self) -> TEN_VDDCORE_AON_R {
        TEN_VDDCORE_AON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_xtal32k(&self) -> TEN_XTAL32K_R {
        TEN_XTAL32K_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_xtal32k(&self) -> DTEN_XTAL32K_R {
        DTEN_XTAL32K_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ten_aon(&self) -> TEN_AON_R {
        TEN_AON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux_aon(&self) -> TMUX_AON_R {
        TMUX_AON_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_cip_misc_aon(&mut self) -> TEN_CIP_MISC_AON_W {
        TEN_CIP_MISC_AON_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_mbg_aon(&mut self) -> TEN_MBG_AON_W {
        TEN_MBG_AON_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn dten_xtal_aon(&mut self) -> DTEN_XTAL_AON_W {
        DTEN_XTAL_AON_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_xtal_aon(&mut self) -> TEN_XTAL_AON_W {
        TEN_XTAL_AON_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_ldo15rf_aon(&mut self) -> TEN_LDO15RF_AON_W {
        TEN_LDO15RF_AON_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_bg_sys_aon(&mut self) -> TEN_BG_SYS_AON_W {
        TEN_BG_SYS_AON_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ten_dcdc18_1_aon(&mut self) -> TEN_DCDC18_1_AON_W {
        TEN_DCDC18_1_AON_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ten_dcdc18_0_aon(&mut self) -> TEN_DCDC18_0_AON_W {
        TEN_DCDC18_0_AON_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_ldo11soc_aon(&mut self) -> TEN_LDO11SOC_AON_W {
        TEN_LDO11SOC_AON_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_vddcore_aon(&mut self) -> TEN_VDDCORE_AON_W {
        TEN_VDDCORE_AON_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ten_xtal32k(&mut self) -> TEN_XTAL32K_W {
        TEN_XTAL32K_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dten_xtal32k(&mut self) -> DTEN_XTAL32K_W {
        DTEN_XTAL32K_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ten_aon(&mut self) -> TEN_AON_W {
        TEN_AON_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux_aon(&mut self) -> TMUX_AON_W {
        TMUX_AON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "aon_common.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aon_common](index.html) module"]
pub struct AON_COMMON_SPEC;
impl crate::RegisterSpec for AON_COMMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aon_common::R](R) reader structure"]
impl crate::Readable for AON_COMMON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aon_common::W](W) writer structure"]
impl crate::Writable for AON_COMMON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets aon_common to value 0"]
impl crate::Resettable for AON_COMMON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
