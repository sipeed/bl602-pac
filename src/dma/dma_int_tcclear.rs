#[doc = "Register `DMA_IntTCClear` writer"]
pub struct W(crate::W<DMA_INTTCCLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INTTCCLEAR_SPEC>;
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
impl From<crate::W<DMA_INTTCCLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INTTCCLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IntTCClear` writer - "]
pub struct INTTCCLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTCCLEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_tcclear(&mut self) -> INTTCCLEAR_W {
        INTTCCLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_IntTCClear.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_tcclear](index.html) module"]
pub struct DMA_INTTCCLEAR_SPEC;
impl crate::RegisterSpec for DMA_INTTCCLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_int_tcclear::W](W) writer structure"]
impl crate::Writable for DMA_INTTCCLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_IntTCClear to value 0"]
impl crate::Resettable for DMA_INTTCCLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
