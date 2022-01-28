#[doc = "Register `DMA_IntErrorStatus` reader"]
pub struct R(crate::R<DMA_INTERRORSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INTERRORSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INTERRORSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INTERRORSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IntErrorStatus` reader - "]
pub struct INTERRORSTATUS_R(crate::FieldReader<u8, u8>);
impl INTERRORSTATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTERRORSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRORSTATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn int_error_status(&self) -> INTERRORSTATUS_R {
        INTERRORSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA_IntErrorStatus.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_error_status](index.html) module"]
pub struct DMA_INTERRORSTATUS_SPEC;
impl crate::RegisterSpec for DMA_INTERRORSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_error_status::R](R) reader structure"]
impl crate::Readable for DMA_INTERRORSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_IntErrorStatus to value 0"]
impl crate::Resettable for DMA_INTERRORSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
