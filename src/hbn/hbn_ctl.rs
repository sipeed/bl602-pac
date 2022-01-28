#[doc = "Register `HBN_CTL` reader"]
pub struct R(crate::R<HBN_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HBN_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HBN_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_CTL` writer"]
pub struct W(crate::W<HBN_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_CTL_SPEC>;
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
impl From<crate::W<HBN_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HBN_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hbn_state` reader - "]
pub struct HBN_STATE_R(crate::FieldReader<u8, u8>);
impl HBN_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBN_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram_slp` reader - "]
pub struct SRAM_SLP_R(crate::FieldReader<bool, bool>);
impl SRAM_SLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram_slp_option` reader - "]
pub struct SRAM_SLP_OPTION_R(crate::FieldReader<bool, bool>);
impl SRAM_SLP_OPTION_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_SLP_OPTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_SLP_OPTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram_slp_option` writer - "]
pub struct SRAM_SLP_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_SLP_OPTION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `pwr_on_option` reader - "]
pub struct PWR_ON_OPTION_R(crate::FieldReader<bool, bool>);
impl PWR_ON_OPTION_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWR_ON_OPTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_ON_OPTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_on_option` writer - "]
pub struct PWR_ON_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_ON_OPTION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `rtc_dly_option` reader - "]
pub struct RTC_DLY_OPTION_R(crate::FieldReader<bool, bool>);
impl RTC_DLY_OPTION_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTC_DLY_OPTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_DLY_OPTION_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtc_dly_option` writer - "]
pub struct RTC_DLY_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_DLY_OPTION_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `pu_dcdc18_aon` reader - "]
pub struct PU_DCDC18_AON_R(crate::FieldReader<bool, bool>);
impl PU_DCDC18_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_DCDC18_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_DCDC18_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_dcdc18_aon` writer - "]
pub struct PU_DCDC18_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_DCDC18_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `hbn_ldo11_aon_vout_sel` reader - "]
pub struct HBN_LDO11_AON_VOUT_SEL_R(crate::FieldReader<u8, u8>);
impl HBN_LDO11_AON_VOUT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBN_LDO11_AON_VOUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_LDO11_AON_VOUT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_ldo11_aon_vout_sel` writer - "]
pub struct HBN_LDO11_AON_VOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_LDO11_AON_VOUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 19)) | ((value as u32 & 0x0f) << 19);
        self.w
    }
}
#[doc = "Field `hbn_ldo11_rt_vout_sel` reader - "]
pub struct HBN_LDO11_RT_VOUT_SEL_R(crate::FieldReader<u8, u8>);
impl HBN_LDO11_RT_VOUT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBN_LDO11_RT_VOUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_LDO11_RT_VOUT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_ldo11_rt_vout_sel` writer - "]
pub struct HBN_LDO11_RT_VOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_LDO11_RT_VOUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 15)) | ((value as u32 & 0x0f) << 15);
        self.w
    }
}
#[doc = "Field `hbn_dis_pwr_off_ldo11_rt` reader - "]
pub struct HBN_DIS_PWR_OFF_LDO11_RT_R(crate::FieldReader<bool, bool>);
impl HBN_DIS_PWR_OFF_LDO11_RT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HBN_DIS_PWR_OFF_LDO11_RT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_DIS_PWR_OFF_LDO11_RT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_dis_pwr_off_ldo11_rt` writer - "]
pub struct HBN_DIS_PWR_OFF_LDO11_RT_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_DIS_PWR_OFF_LDO11_RT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `hbn_dis_pwr_off_ldo11` reader - "]
pub struct HBN_DIS_PWR_OFF_LDO11_R(crate::FieldReader<bool, bool>);
impl HBN_DIS_PWR_OFF_LDO11_R {
    pub(crate) fn new(bits: bool) -> Self {
        HBN_DIS_PWR_OFF_LDO11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_DIS_PWR_OFF_LDO11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_dis_pwr_off_ldo11` writer - "]
pub struct HBN_DIS_PWR_OFF_LDO11_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_DIS_PWR_OFF_LDO11_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `sw_rst` reader - "]
pub struct SW_RST_R(crate::FieldReader<bool, bool>);
impl SW_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SW_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SW_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sw_rst` writer - "]
pub struct SW_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_RST_W<'a> {
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
#[doc = "Field `pwrdn_hbn_rtc` reader - "]
pub struct PWRDN_HBN_RTC_R(crate::FieldReader<bool, bool>);
impl PWRDN_HBN_RTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRDN_HBN_RTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRDN_HBN_RTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwrdn_hbn_rtc` writer - "]
pub struct PWRDN_HBN_RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDN_HBN_RTC_W<'a> {
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
#[doc = "Field `pwrdn_hbn_core` reader - "]
pub struct PWRDN_HBN_CORE_R(crate::FieldReader<bool, bool>);
impl PWRDN_HBN_CORE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRDN_HBN_CORE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRDN_HBN_CORE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwrdn_hbn_core` writer - "]
pub struct PWRDN_HBN_CORE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDN_HBN_CORE_W<'a> {
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
#[doc = "Field `trap_mode` reader - "]
pub struct TRAP_MODE_R(crate::FieldReader<bool, bool>);
impl TRAP_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRAP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRAP_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_mode` writer - "]
pub struct HBN_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `rtc_ctl` reader - "]
pub struct RTC_CTL_R(crate::FieldReader<u8, u8>);
impl RTC_CTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTC_CTL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CTL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtc_ctl` writer - "]
pub struct RTC_CTL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CTL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn hbn_state(&self) -> HBN_STATE_R {
        HBN_STATE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sram_slp(&self) -> SRAM_SLP_R {
        SRAM_SLP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sram_slp_option(&self) -> SRAM_SLP_OPTION_R {
        SRAM_SLP_OPTION_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwr_on_option(&self) -> PWR_ON_OPTION_R {
        PWR_ON_OPTION_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_dly_option(&self) -> RTC_DLY_OPTION_R {
        RTC_DLY_OPTION_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_dcdc18_aon(&self) -> PU_DCDC18_AON_R {
        PU_DCDC18_AON_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn hbn_ldo11_aon_vout_sel(&self) -> HBN_LDO11_AON_VOUT_SEL_R {
        HBN_LDO11_AON_VOUT_SEL_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn hbn_ldo11_rt_vout_sel(&self) -> HBN_LDO11_RT_VOUT_SEL_R {
        HBN_LDO11_RT_VOUT_SEL_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11_rt(&self) -> HBN_DIS_PWR_OFF_LDO11_RT_R {
        HBN_DIS_PWR_OFF_LDO11_RT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11(&self) -> HBN_DIS_PWR_OFF_LDO11_R {
        HBN_DIS_PWR_OFF_LDO11_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sw_rst(&self) -> SW_RST_R {
        SW_RST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pwrdn_hbn_rtc(&self) -> PWRDN_HBN_RTC_R {
        PWRDN_HBN_RTC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwrdn_hbn_core(&self) -> PWRDN_HBN_CORE_R {
        PWRDN_HBN_CORE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trap_mode(&self) -> TRAP_MODE_R {
        TRAP_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rtc_ctl(&self) -> RTC_CTL_R {
        RTC_CTL_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sram_slp_option(&mut self) -> SRAM_SLP_OPTION_W {
        SRAM_SLP_OPTION_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pwr_on_option(&mut self) -> PWR_ON_OPTION_W {
        PWR_ON_OPTION_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rtc_dly_option(&mut self) -> RTC_DLY_OPTION_W {
        RTC_DLY_OPTION_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_dcdc18_aon(&mut self) -> PU_DCDC18_AON_W {
        PU_DCDC18_AON_W { w: self }
    }
    #[doc = "Bits 19:22"]
    #[inline(always)]
    pub fn hbn_ldo11_aon_vout_sel(&mut self) -> HBN_LDO11_AON_VOUT_SEL_W {
        HBN_LDO11_AON_VOUT_SEL_W { w: self }
    }
    #[doc = "Bits 15:18"]
    #[inline(always)]
    pub fn hbn_ldo11_rt_vout_sel(&mut self) -> HBN_LDO11_RT_VOUT_SEL_W {
        HBN_LDO11_RT_VOUT_SEL_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11_rt(&mut self) -> HBN_DIS_PWR_OFF_LDO11_RT_W {
        HBN_DIS_PWR_OFF_LDO11_RT_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hbn_dis_pwr_off_ldo11(&mut self) -> HBN_DIS_PWR_OFF_LDO11_W {
        HBN_DIS_PWR_OFF_LDO11_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sw_rst(&mut self) -> SW_RST_W {
        SW_RST_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pwrdn_hbn_rtc(&mut self) -> PWRDN_HBN_RTC_W {
        PWRDN_HBN_RTC_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwrdn_hbn_core(&mut self) -> PWRDN_HBN_CORE_W {
        PWRDN_HBN_CORE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn hbn_mode(&mut self) -> HBN_MODE_W {
        HBN_MODE_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rtc_ctl(&mut self) -> RTC_CTL_W {
        RTC_CTL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_CTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_ctl](index.html) module"]
pub struct HBN_CTL_SPEC;
impl crate::RegisterSpec for HBN_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_ctl::R](R) reader structure"]
impl crate::Readable for HBN_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_ctl::W](W) writer structure"]
impl crate::Writable for HBN_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_CTL to value 0x00d5_0000"]
impl crate::Resettable for HBN_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00d5_0000
    }
}
