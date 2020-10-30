#[doc = "Register `rf_top_aon` reader"]
pub struct R(crate::R<RF_TOP_AON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_TOP_AON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RF_TOP_AON_SPEC>> for R {
    fn from(reader: crate::R<RF_TOP_AON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_top_aon` writer"]
pub struct W(crate::W<RF_TOP_AON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_TOP_AON_SPEC>;
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
impl core::convert::From<crate::W<RF_TOP_AON_SPEC>> for W {
    fn from(writer: crate::W<RF_TOP_AON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ldo15rf_bypass_aon` reader - "]
pub struct LDO15RF_BYPASS_AON_R(crate::FieldReader<bool, bool>);
impl LDO15RF_BYPASS_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO15RF_BYPASS_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO15RF_BYPASS_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo15rf_bypass_aon` writer - "]
pub struct LDO15RF_BYPASS_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_BYPASS_AON_W<'a> {
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
#[doc = "Field `ldo15rf_cc_aon` reader - "]
pub struct LDO15RF_CC_AON_R(crate::FieldReader<u8, u8>);
impl LDO15RF_CC_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        LDO15RF_CC_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO15RF_CC_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo15rf_cc_aon` writer - "]
pub struct LDO15RF_CC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_CC_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `ldo15rf_vout_sel_aon` reader - "]
pub struct LDO15RF_VOUT_SEL_AON_R(crate::FieldReader<u8, u8>);
impl LDO15RF_VOUT_SEL_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        LDO15RF_VOUT_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO15RF_VOUT_SEL_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo15rf_vout_sel_aon` writer - "]
pub struct LDO15RF_VOUT_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_VOUT_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `ldo15rf_pulldown_sel_aon` reader - "]
pub struct LDO15RF_PULLDOWN_SEL_AON_R(crate::FieldReader<bool, bool>);
impl LDO15RF_PULLDOWN_SEL_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO15RF_PULLDOWN_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO15RF_PULLDOWN_SEL_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo15rf_pulldown_sel_aon` writer - "]
pub struct LDO15RF_PULLDOWN_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_PULLDOWN_SEL_AON_W<'a> {
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
#[doc = "Field `ldo15rf_pulldown_aon` reader - "]
pub struct LDO15RF_PULLDOWN_AON_R(crate::FieldReader<bool, bool>);
impl LDO15RF_PULLDOWN_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO15RF_PULLDOWN_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO15RF_PULLDOWN_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo15rf_pulldown_aon` writer - "]
pub struct LDO15RF_PULLDOWN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_PULLDOWN_AON_W<'a> {
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
#[doc = "Field `ldo15rf_sstart_delay_aon` reader - "]
pub struct LDO15RF_SSTART_DELAY_AON_R(crate::FieldReader<u8, u8>);
impl LDO15RF_SSTART_DELAY_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        LDO15RF_SSTART_DELAY_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO15RF_SSTART_DELAY_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo15rf_sstart_delay_aon` writer - "]
pub struct LDO15RF_SSTART_DELAY_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_SSTART_DELAY_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | (((value as u32) & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `ldo15rf_sstart_sel_aon` reader - "]
pub struct LDO15RF_SSTART_SEL_AON_R(crate::FieldReader<bool, bool>);
impl LDO15RF_SSTART_SEL_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO15RF_SSTART_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO15RF_SSTART_SEL_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo15rf_sstart_sel_aon` writer - "]
pub struct LDO15RF_SSTART_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO15RF_SSTART_SEL_AON_W<'a> {
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
#[doc = "Field `pu_xtal_aon` reader - "]
pub struct PU_XTAL_AON_R(crate::FieldReader<bool, bool>);
impl PU_XTAL_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_XTAL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_XTAL_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_xtal_aon` writer - "]
pub struct PU_XTAL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_XTAL_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `pu_xtal_buf_aon` reader - "]
pub struct PU_XTAL_BUF_AON_R(crate::FieldReader<bool, bool>);
impl PU_XTAL_BUF_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_XTAL_BUF_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_XTAL_BUF_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_xtal_buf_aon` writer - "]
pub struct PU_XTAL_BUF_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_XTAL_BUF_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `pu_sfreg_aon` reader - "]
pub struct PU_SFREG_AON_R(crate::FieldReader<bool, bool>);
impl PU_SFREG_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_SFREG_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_SFREG_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_sfreg_aon` writer - "]
pub struct PU_SFREG_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_SFREG_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `pu_ldo15rf_aon` reader - "]
pub struct PU_LDO15RF_AON_R(crate::FieldReader<bool, bool>);
impl PU_LDO15RF_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_LDO15RF_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_LDO15RF_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_ldo15rf_aon` writer - "]
pub struct PU_LDO15RF_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LDO15RF_AON_W<'a> {
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
#[doc = "Field `pu_mbg_aon` reader - "]
pub struct PU_MBG_AON_R(crate::FieldReader<bool, bool>);
impl PU_MBG_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_MBG_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_MBG_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_mbg_aon` writer - "]
pub struct PU_MBG_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_MBG_AON_W<'a> {
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
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo15rf_bypass_aon(&self) -> LDO15RF_BYPASS_AON_R {
        LDO15RF_BYPASS_AON_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo15rf_cc_aon(&self) -> LDO15RF_CC_AON_R {
        LDO15RF_CC_AON_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo15rf_vout_sel_aon(&self) -> LDO15RF_VOUT_SEL_AON_R {
        LDO15RF_VOUT_SEL_AON_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_sel_aon(&self) -> LDO15RF_PULLDOWN_SEL_AON_R {
        LDO15RF_PULLDOWN_SEL_AON_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_aon(&self) -> LDO15RF_PULLDOWN_AON_R {
        LDO15RF_PULLDOWN_AON_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn ldo15rf_sstart_delay_aon(&self) -> LDO15RF_SSTART_DELAY_AON_R {
        LDO15RF_SSTART_DELAY_AON_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ldo15rf_sstart_sel_aon(&self) -> LDO15RF_SSTART_SEL_AON_R {
        LDO15RF_SSTART_SEL_AON_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_xtal_aon(&self) -> PU_XTAL_AON_R {
        PU_XTAL_AON_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_xtal_buf_aon(&self) -> PU_XTAL_BUF_AON_R {
        PU_XTAL_BUF_AON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_sfreg_aon(&self) -> PU_SFREG_AON_R {
        PU_SFREG_AON_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_ldo15rf_aon(&self) -> PU_LDO15RF_AON_R {
        PU_LDO15RF_AON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_mbg_aon(&self) -> PU_MBG_AON_R {
        PU_MBG_AON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo15rf_bypass_aon(&mut self) -> LDO15RF_BYPASS_AON_W {
        LDO15RF_BYPASS_AON_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo15rf_cc_aon(&mut self) -> LDO15RF_CC_AON_W {
        LDO15RF_CC_AON_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn ldo15rf_vout_sel_aon(&mut self) -> LDO15RF_VOUT_SEL_AON_W {
        LDO15RF_VOUT_SEL_AON_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_sel_aon(&mut self) -> LDO15RF_PULLDOWN_SEL_AON_W {
        LDO15RF_PULLDOWN_SEL_AON_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ldo15rf_pulldown_aon(&mut self) -> LDO15RF_PULLDOWN_AON_W {
        LDO15RF_PULLDOWN_AON_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn ldo15rf_sstart_delay_aon(&mut self) -> LDO15RF_SSTART_DELAY_AON_W {
        LDO15RF_SSTART_DELAY_AON_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ldo15rf_sstart_sel_aon(&mut self) -> LDO15RF_SSTART_SEL_AON_W {
        LDO15RF_SSTART_SEL_AON_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pu_xtal_aon(&mut self) -> PU_XTAL_AON_W {
        PU_XTAL_AON_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pu_xtal_buf_aon(&mut self) -> PU_XTAL_BUF_AON_W {
        PU_XTAL_BUF_AON_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pu_sfreg_aon(&mut self) -> PU_SFREG_AON_W {
        PU_SFREG_AON_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pu_ldo15rf_aon(&mut self) -> PU_LDO15RF_AON_W {
        PU_LDO15RF_AON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_mbg_aon(&mut self) -> PU_MBG_AON_W {
        PU_MBG_AON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_top_aon.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_top_aon](index.html) module"]
pub struct RF_TOP_AON_SPEC;
impl crate::RegisterSpec for RF_TOP_AON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_top_aon::R](R) reader structure"]
impl crate::Readable for RF_TOP_AON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_top_aon::W](W) writer structure"]
impl crate::Writable for RF_TOP_AON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_top_aon to value 0"]
impl crate::Resettable for RF_TOP_AON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
