#[doc = "Register `ef_if_0_status` reader"]
pub struct R(crate::R<EF_IF_0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_IF_0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_IF_0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ef_if_0_status` reader - "]
pub struct EF_IF_0_STATUS_R(crate::FieldReader<u32, u32>);
impl EF_IF_0_STATUS_R {
    pub(crate) fn new(bits: u32) -> Self {
        EF_IF_0_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_STATUS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_if_0_status(&self) -> EF_IF_0_STATUS_R {
        EF_IF_0_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "ef_if_0_status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_0_status](index.html) module"]
pub struct EF_IF_0_STATUS_SPEC;
impl crate::RegisterSpec for EF_IF_0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_0_status::R](R) reader structure"]
impl crate::Readable for EF_IF_0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ef_if_0_status to value 0xe400"]
impl crate::Resettable for EF_IF_0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xe400
    }
}
