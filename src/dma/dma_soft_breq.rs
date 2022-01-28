#[doc = "Register `DMA_SoftBReq` reader"]
pub struct R(crate::R<DMA_SOFTBREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SOFTBREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SOFTBREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SOFTBREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_SoftBReq` writer"]
pub struct W(crate::W<DMA_SOFTBREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SOFTBREQ_SPEC>;
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
impl From<crate::W<DMA_SOFTBREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SOFTBREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SoftBReq` reader - "]
pub struct SOFTBREQ_R(crate::FieldReader<u32, u32>);
impl SOFTBREQ_R {
    pub(crate) fn new(bits: u32) -> Self {
        SOFTBREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTBREQ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SoftBReq` writer - "]
pub struct SOFTBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTBREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_breq(&self) -> SOFTBREQ_R {
        SOFTBREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_breq(&mut self) -> SOFTBREQ_W {
        SOFTBREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_SoftBReq.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_soft_breq](index.html) module"]
pub struct DMA_SOFTBREQ_SPEC;
impl crate::RegisterSpec for DMA_SOFTBREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_soft_breq::R](R) reader structure"]
impl crate::Readable for DMA_SOFTBREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_soft_breq::W](W) writer structure"]
impl crate::Writable for DMA_SOFTBREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_SoftBReq to value 0"]
impl crate::Resettable for DMA_SOFTBREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
