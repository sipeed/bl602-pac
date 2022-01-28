#[doc = "Register `irrx_data_word1` reader"]
pub struct R(crate::R<IRRX_DATA_WORD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_DATA_WORD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRRX_DATA_WORD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRRX_DATA_WORD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sts_irrx_data_word1` reader - "]
pub struct STS_IRRX_DATA_WORD1_R(crate::FieldReader<u32, u32>);
impl STS_IRRX_DATA_WORD1_R {
    pub(crate) fn new(bits: u32) -> Self {
        STS_IRRX_DATA_WORD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_IRRX_DATA_WORD1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sts_irrx_data_word1(&self) -> STS_IRRX_DATA_WORD1_R {
        STS_IRRX_DATA_WORD1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "irrx_data_word1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_data_word1](index.html) module"]
pub struct IRRX_DATA_WORD1_SPEC;
impl crate::RegisterSpec for IRRX_DATA_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_data_word1::R](R) reader structure"]
impl crate::Readable for IRRX_DATA_WORD1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets irrx_data_word1 to value 0"]
impl crate::Resettable for IRRX_DATA_WORD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
