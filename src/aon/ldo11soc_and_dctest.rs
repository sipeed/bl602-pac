#[doc = "Register `ldo11soc_and_dctest` reader"]
pub struct R(crate::R<LDO11SOC_AND_DCTEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDO11SOC_AND_DCTEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LDO11SOC_AND_DCTEST_SPEC>> for R {
    fn from(reader: crate::R<LDO11SOC_AND_DCTEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ldo11soc_and_dctest` writer"]
pub struct W(crate::W<LDO11SOC_AND_DCTEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDO11SOC_AND_DCTEST_SPEC>;
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
impl core::convert::From<crate::W<LDO11SOC_AND_DCTEST_SPEC>> for W {
    fn from(writer: crate::W<LDO11SOC_AND_DCTEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pmip_dc_tp_out_en_aon` reader - "]
pub struct PMIP_DC_TP_OUT_EN_AON_R(crate::FieldReader<bool, bool>);
impl PMIP_DC_TP_OUT_EN_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMIP_DC_TP_OUT_EN_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMIP_DC_TP_OUT_EN_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pmip_dc_tp_out_en_aon` writer - "]
pub struct PMIP_DC_TP_OUT_EN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PMIP_DC_TP_OUT_EN_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `pu_vddcore_misc_aon` reader - "]
pub struct PU_VDDCORE_MISC_AON_R(crate::FieldReader<bool, bool>);
impl PU_VDDCORE_MISC_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_VDDCORE_MISC_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_VDDCORE_MISC_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_vddcore_misc_aon` writer - "]
pub struct PU_VDDCORE_MISC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_VDDCORE_MISC_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ldo11soc_power_good_aon` reader - "]
pub struct LDO11SOC_POWER_GOOD_AON_R(crate::FieldReader<bool, bool>);
impl LDO11SOC_POWER_GOOD_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO11SOC_POWER_GOOD_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO11SOC_POWER_GOOD_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo11soc_rdy_aon` reader - "]
pub struct LDO11SOC_RDY_AON_R(crate::FieldReader<bool, bool>);
impl LDO11SOC_RDY_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO11SOC_RDY_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO11SOC_RDY_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo11soc_cc_aon` reader - "]
pub struct LDO11SOC_CC_AON_R(crate::FieldReader<u8, u8>);
impl LDO11SOC_CC_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        LDO11SOC_CC_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO11SOC_CC_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo11soc_cc_aon` writer - "]
pub struct LDO11SOC_CC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_CC_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `ldo11soc_vth_sel_aon` reader - "]
pub struct LDO11SOC_VTH_SEL_AON_R(crate::FieldReader<u8, u8>);
impl LDO11SOC_VTH_SEL_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        LDO11SOC_VTH_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO11SOC_VTH_SEL_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo11soc_vth_sel_aon` writer - "]
pub struct LDO11SOC_VTH_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_VTH_SEL_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `ldo11soc_pulldown_sel_aon` reader - "]
pub struct LDO11SOC_PULLDOWN_SEL_AON_R(crate::FieldReader<bool, bool>);
impl LDO11SOC_PULLDOWN_SEL_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO11SOC_PULLDOWN_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO11SOC_PULLDOWN_SEL_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo11soc_pulldown_sel_aon` writer - "]
pub struct LDO11SOC_PULLDOWN_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_PULLDOWN_SEL_AON_W<'a> {
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
#[doc = "Field `ldo11soc_pulldown_aon` reader - "]
pub struct LDO11SOC_PULLDOWN_AON_R(crate::FieldReader<bool, bool>);
impl LDO11SOC_PULLDOWN_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO11SOC_PULLDOWN_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO11SOC_PULLDOWN_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo11soc_pulldown_aon` writer - "]
pub struct LDO11SOC_PULLDOWN_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_PULLDOWN_AON_W<'a> {
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
#[doc = "Field `ldo11soc_sstart_delay_aon` reader - "]
pub struct LDO11SOC_SSTART_DELAY_AON_R(crate::FieldReader<u8, u8>);
impl LDO11SOC_SSTART_DELAY_AON_R {
    pub(crate) fn new(bits: u8) -> Self {
        LDO11SOC_SSTART_DELAY_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO11SOC_SSTART_DELAY_AON_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo11soc_sstart_delay_aon` writer - "]
pub struct LDO11SOC_SSTART_DELAY_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_SSTART_DELAY_AON_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `ldo11soc_sstart_sel_aon` reader - "]
pub struct LDO11SOC_SSTART_SEL_AON_R(crate::FieldReader<bool, bool>);
impl LDO11SOC_SSTART_SEL_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDO11SOC_SSTART_SEL_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LDO11SOC_SSTART_SEL_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ldo11soc_sstart_sel_aon` writer - "]
pub struct LDO11SOC_SSTART_SEL_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> LDO11SOC_SSTART_SEL_AON_W<'a> {
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
#[doc = "Field `pu_ldo11soc_aon` reader - "]
pub struct PU_LDO11SOC_AON_R(crate::FieldReader<bool, bool>);
impl PU_LDO11SOC_AON_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_LDO11SOC_AON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_LDO11SOC_AON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_ldo11soc_aon` writer - "]
pub struct PU_LDO11SOC_AON_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LDO11SOC_AON_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pmip_dc_tp_out_en_aon(&self) -> PMIP_DC_TP_OUT_EN_AON_R {
        PMIP_DC_TP_OUT_EN_AON_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_vddcore_misc_aon(&self) -> PU_VDDCORE_MISC_AON_R {
        PU_VDDCORE_MISC_AON_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn ldo11soc_power_good_aon(&self) -> LDO11SOC_POWER_GOOD_AON_R {
        LDO11SOC_POWER_GOOD_AON_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ldo11soc_rdy_aon(&self) -> LDO11SOC_RDY_AON_R {
        LDO11SOC_RDY_AON_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo11soc_cc_aon(&self) -> LDO11SOC_CC_AON_R {
        LDO11SOC_CC_AON_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ldo11soc_vth_sel_aon(&self) -> LDO11SOC_VTH_SEL_AON_R {
        LDO11SOC_VTH_SEL_AON_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_sel_aon(&self) -> LDO11SOC_PULLDOWN_SEL_AON_R {
        LDO11SOC_PULLDOWN_SEL_AON_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_aon(&self) -> LDO11SOC_PULLDOWN_AON_R {
        LDO11SOC_PULLDOWN_AON_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ldo11soc_sstart_delay_aon(&self) -> LDO11SOC_SSTART_DELAY_AON_R {
        LDO11SOC_SSTART_DELAY_AON_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ldo11soc_sstart_sel_aon(&self) -> LDO11SOC_SSTART_SEL_AON_R {
        LDO11SOC_SSTART_SEL_AON_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo11soc_aon(&self) -> PU_LDO11SOC_AON_R {
        PU_LDO11SOC_AON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pmip_dc_tp_out_en_aon(&mut self) -> PMIP_DC_TP_OUT_EN_AON_W {
        PMIP_DC_TP_OUT_EN_AON_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn pu_vddcore_misc_aon(&mut self) -> PU_VDDCORE_MISC_AON_W {
        PU_VDDCORE_MISC_AON_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn ldo11soc_cc_aon(&mut self) -> LDO11SOC_CC_AON_W {
        LDO11SOC_CC_AON_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ldo11soc_vth_sel_aon(&mut self) -> LDO11SOC_VTH_SEL_AON_W {
        LDO11SOC_VTH_SEL_AON_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_sel_aon(&mut self) -> LDO11SOC_PULLDOWN_SEL_AON_W {
        LDO11SOC_PULLDOWN_SEL_AON_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ldo11soc_pulldown_aon(&mut self) -> LDO11SOC_PULLDOWN_AON_W {
        LDO11SOC_PULLDOWN_AON_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ldo11soc_sstart_delay_aon(&mut self) -> LDO11SOC_SSTART_DELAY_AON_W {
        LDO11SOC_SSTART_DELAY_AON_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ldo11soc_sstart_sel_aon(&mut self) -> LDO11SOC_SSTART_SEL_AON_W {
        LDO11SOC_SSTART_SEL_AON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ldo11soc_aon(&mut self) -> PU_LDO11SOC_AON_W {
        PU_LDO11SOC_AON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ldo11soc_and_dctest.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldo11soc_and_dctest](index.html) module"]
pub struct LDO11SOC_AND_DCTEST_SPEC;
impl crate::RegisterSpec for LDO11SOC_AND_DCTEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldo11soc_and_dctest::R](R) reader structure"]
impl crate::Readable for LDO11SOC_AND_DCTEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldo11soc_and_dctest::W](W) writer structure"]
impl crate::Writable for LDO11SOC_AND_DCTEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ldo11soc_and_dctest to value 0x7000_1811"]
impl crate::Resettable for LDO11SOC_AND_DCTEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7000_1811
    }
}
