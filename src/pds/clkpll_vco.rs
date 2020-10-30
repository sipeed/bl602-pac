#[doc = "Register `clkpll_vco` reader"]
pub struct R(crate::R<CLKPLL_VCO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_VCO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLKPLL_VCO_SPEC>> for R {
    fn from(reader: crate::R<CLKPLL_VCO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_vco` writer"]
pub struct W(crate::W<CLKPLL_VCO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_VCO_SPEC>;
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
impl core::convert::From<crate::W<CLKPLL_VCO_SPEC>> for W {
    fn from(writer: crate::W<CLKPLL_VCO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_shrtr` reader - "]
pub struct CLKPLL_SHRTR_R(crate::FieldReader<bool, bool>);
impl CLKPLL_SHRTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_SHRTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_SHRTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_shrtr` writer - "]
pub struct CLKPLL_SHRTR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SHRTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `clkpll_vco_speed` reader - "]
pub struct CLKPLL_VCO_SPEED_R(crate::FieldReader<u8, u8>);
impl CLKPLL_VCO_SPEED_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_VCO_SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_VCO_SPEED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_vco_speed` writer - "]
pub struct CLKPLL_VCO_SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_VCO_SPEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_shrtr(&self) -> CLKPLL_SHRTR_R {
        CLKPLL_SHRTR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clkpll_vco_speed(&self) -> CLKPLL_VCO_SPEED_R {
        CLKPLL_VCO_SPEED_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_shrtr(&mut self) -> CLKPLL_SHRTR_W {
        CLKPLL_SHRTR_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn clkpll_vco_speed(&mut self) -> CLKPLL_VCO_SPEED_W {
        CLKPLL_VCO_SPEED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_vco.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_vco](index.html) module"]
pub struct CLKPLL_VCO_SPEC;
impl crate::RegisterSpec for CLKPLL_VCO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_vco::R](R) reader structure"]
impl crate::Readable for CLKPLL_VCO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_vco::W](W) writer structure"]
impl crate::Writable for CLKPLL_VCO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clkpll_vco to value 0"]
impl crate::Resettable for CLKPLL_VCO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
