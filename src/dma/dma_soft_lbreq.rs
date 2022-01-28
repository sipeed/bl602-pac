#[doc = "Register `DMA_SoftLBReq` reader"]
pub struct R(crate::R<DMA_SOFTLBREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SOFTLBREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SOFTLBREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SOFTLBREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_SoftLBReq` writer"]
pub struct W(crate::W<DMA_SOFTLBREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SOFTLBREQ_SPEC>;
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
impl From<crate::W<DMA_SOFTLBREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SOFTLBREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SoftLBReq` reader - "]
pub struct SOFTLBREQ_R(crate::FieldReader<u32, u32>);
impl SOFTLBREQ_R {
    pub(crate) fn new(bits: u32) -> Self {
        SOFTLBREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLBREQ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SoftLBReq` writer - "]
pub struct SOFTLBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLBREQ_W<'a> {
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
    pub fn soft_lbreq(&self) -> SOFTLBREQ_R {
        SOFTLBREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lbreq(&mut self) -> SOFTLBREQ_W {
        SOFTLBREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_SoftLBReq.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_soft_lbreq](index.html) module"]
pub struct DMA_SOFTLBREQ_SPEC;
impl crate::RegisterSpec for DMA_SOFTLBREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_soft_lbreq::R](R) reader structure"]
impl crate::Readable for DMA_SOFTLBREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_soft_lbreq::W](W) writer structure"]
impl crate::Writable for DMA_SOFTLBREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_SoftLBReq to value 0"]
impl crate::Resettable for DMA_SOFTLBREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
