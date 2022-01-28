#[doc = "Register `DMA_EnbldChns` reader"]
pub struct R(crate::R<DMA_ENBLDCHNS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ENBLDCHNS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ENBLDCHNS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ENBLDCHNS_SPEC>) -> Self {
        R(reader)
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
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn enabled_channels(&self) -> ENABLEDCHANNELS_R {
        ENABLEDCHANNELS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA_EnbldChns.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_enbld_chns](index.html) module"]
pub struct DMA_ENBLDCHNS_SPEC;
impl crate::RegisterSpec for DMA_ENBLDCHNS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_enbld_chns::R](R) reader structure"]
impl crate::Readable for DMA_ENBLDCHNS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_EnbldChns to value 0"]
impl crate::Resettable for DMA_ENBLDCHNS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
