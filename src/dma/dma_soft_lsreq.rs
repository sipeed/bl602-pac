#[doc = "Register `DMA_SoftLSReq` reader"]
pub struct R(crate::R<DMA_SOFTLSREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SOFTLSREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA_SOFTLSREQ_SPEC>> for R {
    fn from(reader: crate::R<DMA_SOFTLSREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_SoftLSReq` writer"]
pub struct W(crate::W<DMA_SOFTLSREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SOFTLSREQ_SPEC>;
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
impl core::convert::From<crate::W<DMA_SOFTLSREQ_SPEC>> for W {
    fn from(writer: crate::W<DMA_SOFTLSREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SoftLSReq` reader - "]
pub struct SOFTLSREQ_R(crate::FieldReader<u32, u32>);
impl SOFTLSREQ_R {
    pub(crate) fn new(bits: u32) -> Self {
        SOFTLSREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTLSREQ_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SoftLSReq` writer - "]
pub struct SOFTLSREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTLSREQ_W<'a> {
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
    pub fn soft_lsreq(&self) -> SOFTLSREQ_R {
        SOFTLSREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn soft_lsreq(&mut self) -> SOFTLSREQ_W {
        SOFTLSREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_SoftLSReq.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_soft_lsreq](index.html) module"]
pub struct DMA_SOFTLSREQ_SPEC;
impl crate::RegisterSpec for DMA_SOFTLSREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_soft_lsreq::R](R) reader structure"]
impl crate::Readable for DMA_SOFTLSREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_soft_lsreq::W](W) writer structure"]
impl crate::Writable for DMA_SOFTLSREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_SoftLSReq to value 0"]
impl crate::Resettable for DMA_SOFTLSREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
