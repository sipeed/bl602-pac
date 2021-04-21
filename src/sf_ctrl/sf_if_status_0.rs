#[doc = "Register `sf_if_status_0` reader"]
pub struct R(crate::R<SF_IF_STATUS_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_IF_STATUS_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SF_IF_STATUS_0_SPEC>> for R {
    fn from(reader: crate::R<SF_IF_STATUS_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sf_if_status_0` reader - "]
pub struct SF_IF_STATUS_0_R(crate::FieldReader<u32, u32>);
impl SF_IF_STATUS_0_R {
    pub(crate) fn new(bits: u32) -> Self {
        SF_IF_STATUS_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_STATUS_0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_if_status_0(&self) -> SF_IF_STATUS_0_R {
        SF_IF_STATUS_0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "sf_if_status_0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_status_0](index.html) module"]
pub struct SF_IF_STATUS_0_SPEC;
impl crate::RegisterSpec for SF_IF_STATUS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_if_status_0::R](R) reader structure"]
impl crate::Readable for SF_IF_STATUS_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sf_if_status_0 to value 0"]
impl crate::Resettable for SF_IF_STATUS_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
