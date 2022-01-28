#[doc = "Register `DMA_RawIntTCStatus` reader"]
pub struct R(crate::R<DMA_RAWINTTCSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_RAWINTTCSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_RAWINTTCSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_RAWINTTCSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RawIntTCStatus` reader - "]
pub struct RAWINTTCSTATUS_R(crate::FieldReader<u8, u8>);
impl RAWINTTCSTATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RAWINTTCSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWINTTCSTATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn raw_int_tcstatus(&self) -> RAWINTTCSTATUS_R {
        RAWINTTCSTATUS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA_RawIntTCStatus.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_raw_int_tcstatus](index.html) module"]
pub struct DMA_RAWINTTCSTATUS_SPEC;
impl crate::RegisterSpec for DMA_RAWINTTCSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_raw_int_tcstatus::R](R) reader structure"]
impl crate::Readable for DMA_RAWINTTCSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_RawIntTCStatus to value 0"]
impl crate::Resettable for DMA_RAWINTTCSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
