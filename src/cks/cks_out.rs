#[doc = "Register `cks_out` reader"]
pub struct R(crate::R<CKS_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKS_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKS_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKS_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `cks_out` reader - "]
pub struct CKS_OUT_R(crate::FieldReader<u16, u16>);
impl CKS_OUT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CKS_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKS_OUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cks_out(&self) -> CKS_OUT_R {
        CKS_OUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Checksum value out\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cks_out](index.html) module"]
pub struct CKS_OUT_SPEC;
impl crate::RegisterSpec for CKS_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cks_out::R](R) reader structure"]
impl crate::Readable for CKS_OUT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets cks_out to value 0xffff"]
impl crate::Resettable for CKS_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
