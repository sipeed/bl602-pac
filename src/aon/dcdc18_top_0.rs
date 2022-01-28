#[doc = "Register `dcdc18_top_0` reader"]
pub struct R(crate::R<DCDC18_TOP_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDC18_TOP_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDC18_TOP_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDC18_TOP_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dcdc18_top_0` writer"]
pub struct W(crate::W<DCDC18_TOP_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDC18_TOP_0_SPEC>;
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
impl From<crate::W<DCDC18_TOP_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDC18_TOP_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dcdc18_rdy_aon` reader - "]
pub struct DCDC18_RDY_AON_R(crate::FieldReader<bool, bool>);
impl DCDC18_RDY_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC18_RDY_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_RDY_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_sstart_time_aon` reader - "]
pub struct DCDC18_SSTART_TIME_AON_R(crate::FieldReader<u8, u8>);
impl DCDC18_SSTART_TIME_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC18_SSTART_TIME_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_SSTART_TIME_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_sstart_time_aon` writer - "]
pub struct DCDC18_SSTART_TIME_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_SSTART_TIME_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `dcdc18_osc_inhibit_t2_aon` reader - "]
pub struct DCDC18_OSC_INHIBIT_T2_AON_R(crate::FieldReader<bool, bool>);
impl DCDC18_OSC_INHIBIT_T2_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC18_OSC_INHIBIT_T2_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_OSC_INHIBIT_T2_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_osc_inhibit_t2_aon` writer - "]
pub struct DCDC18_OSC_INHIBIT_T2_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_OSC_INHIBIT_T2_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `dcdc18_slow_osc_aon` reader - "]
pub struct DCDC18_SLOW_OSC_AON_R(crate::FieldReader<bool, bool>);
impl DCDC18_SLOW_OSC_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC18_SLOW_OSC_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_SLOW_OSC_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_slow_osc_aon` writer - "]
pub struct DCDC18_SLOW_OSC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_SLOW_OSC_AON_W<'a> {
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
#[doc = "Field `dcdc18_stop_osc_aon` reader - "]
pub struct DCDC18_STOP_OSC_AON_R(crate::FieldReader<bool, bool>);
impl DCDC18_STOP_OSC_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC18_STOP_OSC_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_STOP_OSC_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_stop_osc_aon` writer - "]
pub struct DCDC18_STOP_OSC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_STOP_OSC_AON_W<'a> {
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
#[doc = "Field `dcdc18_slope_curr_sel_aon` reader - "]
pub struct DCDC18_SLOPE_CURR_SEL_AON_R(crate::FieldReader<u8, u8>);
impl DCDC18_SLOPE_CURR_SEL_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC18_SLOPE_CURR_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_SLOPE_CURR_SEL_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_slope_curr_sel_aon` writer - "]
pub struct DCDC18_SLOPE_CURR_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_SLOPE_CURR_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `dcdc18_osc_freq_trim_aon` reader - "]
pub struct DCDC18_OSC_FREQ_TRIM_AON_R(crate::FieldReader<u8, u8>);
impl DCDC18_OSC_FREQ_TRIM_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC18_OSC_FREQ_TRIM_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_OSC_FREQ_TRIM_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_osc_freq_trim_aon` writer - "]
pub struct DCDC18_OSC_FREQ_TRIM_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_OSC_FREQ_TRIM_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `dcdc18_osc_2m_mode_aon` reader - "]
pub struct DCDC18_OSC_2M_MODE_AON_R(crate::FieldReader<bool, bool>);
impl DCDC18_OSC_2M_MODE_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC18_OSC_2M_MODE_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_OSC_2M_MODE_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_osc_2m_mode_aon` writer - "]
pub struct DCDC18_OSC_2M_MODE_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_OSC_2M_MODE_AON_W<'a> {
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
#[doc = "Field `dcdc18_vpfm_aon` reader - "]
pub struct DCDC18_VPFM_AON_R(crate::FieldReader<u8, u8>);
impl DCDC18_VPFM_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC18_VPFM_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_VPFM_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_vpfm_aon` writer - "]
pub struct DCDC18_VPFM_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_VPFM_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `dcdc18_vout_sel_aon` reader - "]
pub struct DCDC18_VOUT_SEL_AON_R(crate::FieldReader<u8, u8>);
impl DCDC18_VOUT_SEL_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        DCDC18_VOUT_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC18_VOUT_SEL_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dcdc18_vout_sel_aon` writer - "]
pub struct DCDC18_VOUT_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC18_VOUT_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | ((value as u32 & 0x1f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn dcdc18_rdy_aon(&self) -> DCDC18_RDY_AON_R {
        DCDC18_RDY_AON_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc18_sstart_time_aon(&self) -> DCDC18_SSTART_TIME_AON_R {
        DCDC18_SSTART_TIME_AON_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc18_osc_inhibit_t2_aon(&self) -> DCDC18_OSC_INHIBIT_T2_AON_R {
        DCDC18_OSC_INHIBIT_T2_AON_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc18_slow_osc_aon(&self) -> DCDC18_SLOW_OSC_AON_R {
        DCDC18_SLOW_OSC_AON_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc18_stop_osc_aon(&self) -> DCDC18_STOP_OSC_AON_R {
        DCDC18_STOP_OSC_AON_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn dcdc18_slope_curr_sel_aon(&self) -> DCDC18_SLOPE_CURR_SEL_AON_R {
        DCDC18_SLOPE_CURR_SEL_AON_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_osc_freq_trim_aon(&self) -> DCDC18_OSC_FREQ_TRIM_AON_R {
        DCDC18_OSC_FREQ_TRIM_AON_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dcdc18_osc_2m_mode_aon(&self) -> DCDC18_OSC_2M_MODE_AON_R {
        DCDC18_OSC_2M_MODE_AON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc18_vpfm_aon(&self) -> DCDC18_VPFM_AON_R {
        DCDC18_VPFM_AON_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn dcdc18_vout_sel_aon(&self) -> DCDC18_VOUT_SEL_AON_R {
        DCDC18_VOUT_SEL_AON_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn dcdc18_sstart_time_aon(&mut self) -> DCDC18_SSTART_TIME_AON_W {
        DCDC18_SSTART_TIME_AON_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn dcdc18_osc_inhibit_t2_aon(&mut self) -> DCDC18_OSC_INHIBIT_T2_AON_W {
        DCDC18_OSC_INHIBIT_T2_AON_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn dcdc18_slow_osc_aon(&mut self) -> DCDC18_SLOW_OSC_AON_W {
        DCDC18_SLOW_OSC_AON_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn dcdc18_stop_osc_aon(&mut self) -> DCDC18_STOP_OSC_AON_W {
        DCDC18_STOP_OSC_AON_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn dcdc18_slope_curr_sel_aon(&mut self) -> DCDC18_SLOPE_CURR_SEL_AON_W {
        DCDC18_SLOPE_CURR_SEL_AON_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn dcdc18_osc_freq_trim_aon(&mut self) -> DCDC18_OSC_FREQ_TRIM_AON_W {
        DCDC18_OSC_FREQ_TRIM_AON_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dcdc18_osc_2m_mode_aon(&mut self) -> DCDC18_OSC_2M_MODE_AON_W {
        DCDC18_OSC_2M_MODE_AON_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn dcdc18_vpfm_aon(&mut self) -> DCDC18_VPFM_AON_W {
        DCDC18_VPFM_AON_W { w: self }
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn dcdc18_vout_sel_aon(&mut self) -> DCDC18_VOUT_SEL_AON_W {
        DCDC18_VOUT_SEL_AON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dcdc18_top_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdc18_top_0](index.html) module"]
pub struct DCDC18_TOP_0_SPEC;
impl crate::RegisterSpec for DCDC18_TOP_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdc18_top_0::R](R) reader structure"]
impl crate::Readable for DCDC18_TOP_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdc18_top_0::W](W) writer structure"]
impl crate::Writable for DCDC18_TOP_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dcdc18_top_0 to value 0x8a58_0736"]
impl crate::Resettable for DCDC18_TOP_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8a58_0736
    }
}
