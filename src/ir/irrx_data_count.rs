#[doc = "Register `irrx_data_count` reader"]
pub struct R(crate::R<IRRX_DATA_COUNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_DATA_COUNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IRRX_DATA_COUNT_SPEC>> for R {
    fn from(reader: crate::R<IRRX_DATA_COUNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sts_irrx_data_cnt` reader - "]
pub struct STS_IRRX_DATA_CNT_R(crate::FieldReader<u8, u8>);
impl STS_IRRX_DATA_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        STS_IRRX_DATA_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_IRRX_DATA_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn sts_irrx_data_cnt(&self) -> STS_IRRX_DATA_CNT_R {
        STS_IRRX_DATA_CNT_R::new((self.bits & 0x7f) as u8)
    }
}
#[doc = "irrx_data_count.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_data_count](index.html) module"]
pub struct IRRX_DATA_COUNT_SPEC;
impl crate::RegisterSpec for IRRX_DATA_COUNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_data_count::R](R) reader structure"]
impl crate::Readable for IRRX_DATA_COUNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets irrx_data_count to value 0"]
impl crate::Resettable for IRRX_DATA_COUNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
