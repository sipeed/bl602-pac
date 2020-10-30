#[doc = "Register `clkpll_cp` reader"]
pub struct R(crate::R<CLKPLL_CP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_CP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLKPLL_CP_SPEC>> for R {
    fn from(reader: crate::R<CLKPLL_CP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_cp` writer"]
pub struct W(crate::W<CLKPLL_CP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_CP_SPEC>;
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
impl core::convert::From<crate::W<CLKPLL_CP_SPEC>> for W {
    fn from(writer: crate::W<CLKPLL_CP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_cp_opamp_en` reader - "]
pub struct CLKPLL_CP_OPAMP_EN_R(crate::FieldReader<bool, bool>);
impl CLKPLL_CP_OPAMP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_CP_OPAMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_CP_OPAMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_cp_opamp_en` writer - "]
pub struct CLKPLL_CP_OPAMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CP_OPAMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `clkpll_cp_startup_en` reader - "]
pub struct CLKPLL_CP_STARTUP_EN_R(crate::FieldReader<bool, bool>);
impl CLKPLL_CP_STARTUP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_CP_STARTUP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_CP_STARTUP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_cp_startup_en` writer - "]
pub struct CLKPLL_CP_STARTUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_CP_STARTUP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `clkpll_int_frac_sw` reader - "]
pub struct CLKPLL_INT_FRAC_SW_R(crate::FieldReader<bool, bool>);
impl CLKPLL_INT_FRAC_SW_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_INT_FRAC_SW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_INT_FRAC_SW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_int_frac_sw` writer - "]
pub struct CLKPLL_INT_FRAC_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_INT_FRAC_SW_W<'a> {
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
#[doc = "Field `clkpll_icp_1u` reader - "]
pub struct CLKPLL_ICP_1U_R(crate::FieldReader<u8, u8>);
impl CLKPLL_ICP_1U_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_ICP_1U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_ICP_1U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_icp_1u` writer - "]
pub struct CLKPLL_ICP_1U_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_ICP_1U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `clkpll_icp_5u` reader - "]
pub struct CLKPLL_ICP_5U_R(crate::FieldReader<u8, u8>);
impl CLKPLL_ICP_5U_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_ICP_5U_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_ICP_5U_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_icp_5u` writer - "]
pub struct CLKPLL_ICP_5U_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_ICP_5U_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `clkpll_sel_cp_bias` reader - "]
pub struct CLKPLL_SEL_CP_BIAS_R(crate::FieldReader<bool, bool>);
impl CLKPLL_SEL_CP_BIAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_SEL_CP_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_SEL_CP_BIAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_sel_cp_bias` writer - "]
pub struct CLKPLL_SEL_CP_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SEL_CP_BIAS_W<'a> {
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
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkpll_cp_opamp_en(&self) -> CLKPLL_CP_OPAMP_EN_R {
        CLKPLL_CP_OPAMP_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_cp_startup_en(&self) -> CLKPLL_CP_STARTUP_EN_R {
        CLKPLL_CP_STARTUP_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_int_frac_sw(&self) -> CLKPLL_INT_FRAC_SW_R {
        CLKPLL_INT_FRAC_SW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn clkpll_icp_1u(&self) -> CLKPLL_ICP_1U_R {
        CLKPLL_ICP_1U_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_icp_5u(&self) -> CLKPLL_ICP_5U_R {
        CLKPLL_ICP_5U_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sel_cp_bias(&self) -> CLKPLL_SEL_CP_BIAS_R {
        CLKPLL_SEL_CP_BIAS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn clkpll_cp_opamp_en(&mut self) -> CLKPLL_CP_OPAMP_EN_W {
        CLKPLL_CP_OPAMP_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_cp_startup_en(&mut self) -> CLKPLL_CP_STARTUP_EN_W {
        CLKPLL_CP_STARTUP_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_int_frac_sw(&mut self) -> CLKPLL_INT_FRAC_SW_W {
        CLKPLL_INT_FRAC_SW_W { w: self }
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn clkpll_icp_1u(&mut self) -> CLKPLL_ICP_1U_W {
        CLKPLL_ICP_1U_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clkpll_icp_5u(&mut self) -> CLKPLL_ICP_5U_W {
        CLKPLL_ICP_5U_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_sel_cp_bias(&mut self) -> CLKPLL_SEL_CP_BIAS_W {
        CLKPLL_SEL_CP_BIAS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_cp.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_cp](index.html) module"]
pub struct CLKPLL_CP_SPEC;
impl crate::RegisterSpec for CLKPLL_CP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_cp::R](R) reader structure"]
impl crate::Readable for CLKPLL_CP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_cp::W](W) writer structure"]
impl crate::Writable for CLKPLL_CP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clkpll_cp to value 0"]
impl crate::Resettable for CLKPLL_CP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
