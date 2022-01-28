#[doc = "Register `clkpll_sdm` reader"]
pub struct R(crate::R<CLKPLL_SDM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_SDM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKPLL_SDM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKPLL_SDM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_sdm` writer"]
pub struct W(crate::W<CLKPLL_SDM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_SDM_SPEC>;
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
impl From<crate::W<CLKPLL_SDM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKPLL_SDM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_sdm_bypass` reader - "]
pub struct CLKPLL_SDM_BYPASS_R(crate::FieldReader<bool, bool>);
impl CLKPLL_SDM_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_SDM_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_SDM_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_sdm_bypass` writer - "]
pub struct CLKPLL_SDM_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SDM_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `clkpll_sdm_flag` reader - "]
pub struct CLKPLL_SDM_FLAG_R(crate::FieldReader<bool, bool>);
impl CLKPLL_SDM_FLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_SDM_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_SDM_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_sdm_flag` writer - "]
pub struct CLKPLL_SDM_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SDM_FLAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `clkpll_dither_sel` reader - "]
pub struct CLKPLL_DITHER_SEL_R(crate::FieldReader<u8, u8>);
impl CLKPLL_DITHER_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKPLL_DITHER_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_DITHER_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_dither_sel` writer - "]
pub struct CLKPLL_DITHER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_DITHER_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `clkpll_sdmin` reader - "]
pub struct CLKPLL_SDMIN_R(crate::FieldReader<u32, u32>);
impl CLKPLL_SDMIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        CLKPLL_SDMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_SDMIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_sdmin` writer - "]
pub struct CLKPLL_SDMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_SDMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clkpll_sdm_bypass(&self) -> CLKPLL_SDM_BYPASS_R {
        CLKPLL_SDM_BYPASS_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clkpll_sdm_flag(&self) -> CLKPLL_SDM_FLAG_R {
        CLKPLL_SDM_FLAG_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_dither_sel(&self) -> CLKPLL_DITHER_SEL_R {
        CLKPLL_DITHER_SEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn clkpll_sdmin(&self) -> CLKPLL_SDMIN_R {
        CLKPLL_SDMIN_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn clkpll_sdm_bypass(&mut self) -> CLKPLL_SDM_BYPASS_W {
        CLKPLL_SDM_BYPASS_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn clkpll_sdm_flag(&mut self) -> CLKPLL_SDM_FLAG_W {
        CLKPLL_SDM_FLAG_W { w: self }
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn clkpll_dither_sel(&mut self) -> CLKPLL_DITHER_SEL_W {
        CLKPLL_DITHER_SEL_W { w: self }
    }
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn clkpll_sdmin(&mut self) -> CLKPLL_SDMIN_W {
        CLKPLL_SDMIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_sdm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_sdm](index.html) module"]
pub struct CLKPLL_SDM_SPEC;
impl crate::RegisterSpec for CLKPLL_SDM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_sdm::R](R) reader structure"]
impl crate::Readable for CLKPLL_SDM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_sdm::W](W) writer structure"]
impl crate::Writable for CLKPLL_SDM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clkpll_sdm to value 0x1060_0000"]
impl crate::Resettable for CLKPLL_SDM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1060_0000
    }
}
