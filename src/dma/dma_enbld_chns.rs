#[doc = "Register `DMA_EnbldChns` reader"]
pub struct R(crate::R<DMA_ENBLDCHNS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ENBLDCHNS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA_ENBLDCHNS_SPEC>> for R {
    fn from(reader: crate::R<DMA_ENBLDCHNS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_EnbldChns` writer"]
pub struct W(crate::W<DMA_ENBLDCHNS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ENBLDCHNS_SPEC>;
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
impl core::convert::From<crate::W<DMA_ENBLDCHNS_SPEC>> for W {
    fn from(writer: crate::W<DMA_ENBLDCHNS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EnabledChannels` reader - "]
pub struct ENABLEDCHANNELS_R(crate::FieldReader<u8, u8>);
impl ENABLEDCHANNELS_R {
    pub(crate) fn new(bits: u8) -> Self {
        ENABLEDCHANNELS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLEDCHANNELS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EnabledChannels` writer - "]
pub struct ENABLEDCHANNELS_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEDCHANNELS_W<'a> {
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
    pub fn enabled_channels(&self) -> ENABLEDCHANNELS_R {
        ENABLEDCHANNELS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn enabled_channels(&mut self) -> ENABLEDCHANNELS_W {
        ENABLEDCHANNELS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_EnbldChns.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_enbld_chns](index.html) module"]
pub struct DMA_ENBLDCHNS_SPEC;
impl crate::RegisterSpec for DMA_ENBLDCHNS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_enbld_chns::R](R) reader structure"]
impl crate::Readable for DMA_ENBLDCHNS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_enbld_chns::W](W) writer structure"]
impl crate::Writable for DMA_ENBLDCHNS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_EnbldChns to value 0"]
impl crate::Resettable for DMA_ENBLDCHNS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
