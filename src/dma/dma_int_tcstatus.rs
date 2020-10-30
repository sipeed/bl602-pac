#[doc = "Register `DMA_IntTCStatus` reader"]
pub struct R(crate::R<DMA_INTTCSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INTTCSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA_INTTCSTATUS_SPEC>> for R {
    fn from(reader: crate::R<DMA_INTTCSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_IntTCStatus` writer"]
pub struct W(crate::W<DMA_INTTCSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INTTCSTATUS_SPEC>;
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
impl core::convert::From<crate::W<DMA_INTTCSTATUS_SPEC>> for W {
    fn from(writer: crate::W<DMA_INTTCSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IntTCStatus` reader - "]
pub struct INTTCSTATUS_R(crate::FieldReader<u8, u8>);
impl INTTCSTATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTTCSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTTCSTATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IntTCStatus` writer - "]
pub struct INTTCSTATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCSTATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_tcstatus(&self) -> INTTCSTATUS_R {
        INTTCSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_tcstatus(&mut self) -> INTTCSTATUS_W {
        INTTCSTATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IntTCStatus.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_tcstatus](index.html) module"]
pub struct DMA_INTTCSTATUS_SPEC;
impl crate::RegisterSpec for DMA_INTTCSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_tcstatus::R](R) reader structure"]
impl crate::Readable for DMA_INTTCSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int_tcstatus::W](W) writer structure"]
impl crate::Writable for DMA_INTTCSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_IntTCStatus to value 0"]
impl crate::Resettable for DMA_INTTCSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
