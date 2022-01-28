#[doc = "Register `DMA_RawIntErrorStatus` reader"]
pub struct R(crate::R<DMA_RAWINTERRORSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RAWINTERRORSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RAWINTERRORSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RAWINTERRORSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RawIntErrorStatus` reader - "]
pub struct RAWINTERRORSTATUS_R(crate::FieldReader<u8, u8>);
impl RAWINTERRORSTATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RAWINTERRORSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTERRORSTATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn raw_int_error_status(&self) -> RAWINTERRORSTATUS_R {
        RAWINTERRORSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA_RawIntErrorStatus.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_raw_int_error_status](index.html) module"]
pub struct DMA_RAWINTERRORSTATUS_SPEC;
impl crate::RegisterSpec for DMA_RAWINTERRORSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_raw_int_error_status::R](R) reader structure"]
impl crate::Readable for DMA_RAWINTERRORSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_RawIntErrorStatus to value 0"]
impl crate::Resettable for DMA_RAWINTERRORSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
