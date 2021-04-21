#[doc = "Register `TCR2` reader"]
pub struct R(crate::R<TCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TCR2_SPEC>> for R {
    fn from(reader: crate::R<TCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tcr` reader - "]
pub struct TCR_R(crate::FieldReader<u32, u32>);
impl TCR_R {
    pub(crate) fn new(bits: u32) -> Self {
        TCR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "TCR2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr2](index.html) module"]
pub struct TCR2_SPEC;
impl crate::RegisterSpec for TCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr2::R](R) reader structure"]
impl crate::Readable for TCR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCR2 to value 0"]
impl crate::Resettable for TCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
