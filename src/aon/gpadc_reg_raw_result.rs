#[doc = "Register `gpadc_reg_raw_result` reader"]
pub struct R(crate::R<GPADC_REG_RAW_RESULT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_RAW_RESULT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_RAW_RESULT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_RAW_RESULT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `gpadc_raw_data` reader - "]
pub struct GPADC_RAW_DATA_R(crate::FieldReader<u16, u16>);
impl GPADC_RAW_DATA_R {
    pub(crate) fn new(bits: u16) -> Self {
        GPADC_RAW_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_RAW_DATA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn gpadc_raw_data(&self) -> GPADC_RAW_DATA_R {
        GPADC_RAW_DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "gpadc_reg_raw_result.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_raw_result](index.html) module"]
pub struct GPADC_REG_RAW_RESULT_SPEC;
impl crate::RegisterSpec for GPADC_REG_RAW_RESULT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_raw_result::R](R) reader structure"]
impl crate::Readable for GPADC_REG_RAW_RESULT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets gpadc_reg_raw_result to value 0"]
impl crate::Resettable for GPADC_REG_RAW_RESULT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
