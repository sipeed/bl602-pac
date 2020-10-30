#[doc = "Register `gpadc_dma_rdata` reader"]
pub struct R(crate::R<GPADC_DMA_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_DMA_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPADC_DMA_RDATA_SPEC>> for R {
    fn from(reader: crate::R<GPADC_DMA_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_dma_rdata` writer"]
pub struct W(crate::W<GPADC_DMA_RDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_DMA_RDATA_SPEC>;
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
impl core::convert::From<crate::W<GPADC_DMA_RDATA_SPEC>> for W {
    fn from(writer: crate::W<GPADC_DMA_RDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rsvd_31_26` reader - "]
pub struct RSVD_31_26_R(crate::FieldReader<u8, u8>);
impl RSVD_31_26_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSVD_31_26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_31_26_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsvd_31_26` writer - "]
pub struct RSVD_31_26_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_31_26_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
#[doc = "Field `gpadc_dma_rdata` reader - "]
pub struct GPADC_DMA_RDATA_R(crate::FieldReader<u32, u32>);
impl GPADC_DMA_RDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        GPADC_DMA_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_DMA_RDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_dma_rdata` writer - "]
pub struct GPADC_DMA_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DMA_RDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rsvd_31_26(&self) -> RSVD_31_26_R {
        RSVD_31_26_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_dma_rdata(&self) -> GPADC_DMA_RDATA_R {
        GPADC_DMA_RDATA_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn rsvd_31_26(&mut self) -> RSVD_31_26_W {
        RSVD_31_26_W { w: self }
    }
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_dma_rdata(&mut self) -> GPADC_DMA_RDATA_W {
        GPADC_DMA_RDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_dma_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_dma_rdata](index.html) module"]
pub struct GPADC_DMA_RDATA_SPEC;
impl crate::RegisterSpec for GPADC_DMA_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_dma_rdata::R](R) reader structure"]
impl crate::Readable for GPADC_DMA_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_dma_rdata::W](W) writer structure"]
impl crate::Writable for GPADC_DMA_RDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_dma_rdata to value 0"]
impl crate::Resettable for GPADC_DMA_RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
