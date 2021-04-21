#[doc = "Register `gpadc_reg_result` reader"]
pub struct R(crate::R<GPADC_REG_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPADC_REG_RESULT_SPEC>> for R {
    fn from(reader: crate::R<GPADC_REG_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `gpadc_data_out` reader - "]
pub struct GPADC_DATA_OUT_R(crate::FieldReader<u32, u32>);
impl GPADC_DATA_OUT_R {
    pub(crate) fn new(bits: u32) -> Self {
        GPADC_DATA_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_DATA_OUT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpadc_data_out(&self) -> GPADC_DATA_OUT_R {
        GPADC_DATA_OUT_R::new((self.bits & 0x03ff_ffff) as u32)
    }
}
#[doc = "gpadc_reg_result.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_result](index.html) module"]
pub struct GPADC_REG_RESULT_SPEC;
impl crate::RegisterSpec for GPADC_REG_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_result::R](R) reader structure"]
impl crate::Readable for GPADC_REG_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets gpadc_reg_result to value 0x01ef_0000"]
impl crate::Resettable for GPADC_REG_RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01ef_0000
    }
}
