#[doc = "Register `ef_if_sw_usage_0` reader"]
pub struct R(crate::R<EF_IF_SW_USAGE_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_SW_USAGE_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EF_IF_SW_USAGE_0_SPEC>> for R {
    fn from(reader: crate::R<EF_IF_SW_USAGE_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ef_if_sw_usage_0` reader - "]
pub struct EF_IF_SW_USAGE_0_R(crate::FieldReader<u32, u32>);
impl EF_IF_SW_USAGE_0_R {
    pub(crate) fn new(bits: u32) -> Self {
        EF_IF_SW_USAGE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_SW_USAGE_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_if_sw_usage_0(&self) -> EF_IF_SW_USAGE_0_R {
        EF_IF_SW_USAGE_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ef_if_sw_usage_0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_sw_usage_0](index.html) module"]
pub struct EF_IF_SW_USAGE_0_SPEC;
impl crate::RegisterSpec for EF_IF_SW_USAGE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_sw_usage_0::R](R) reader structure"]
impl crate::Readable for EF_IF_SW_USAGE_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ef_if_sw_usage_0 to value 0"]
impl crate::Resettable for EF_IF_SW_USAGE_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
