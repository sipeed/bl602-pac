#[doc = "Register `TCVWR3` reader"]
pub struct R(crate::R<TCVWR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCVWR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TCVWR3_SPEC>> for R {
    fn from(reader: crate::R<TCVWR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tcvwr` reader - "]
pub struct TCVWR_R(crate::FieldReader<u32, u32>);
impl TCVWR_R {
    pub(crate) fn new(bits: u32) -> Self {
        TCVWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCVWR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvwr(&self) -> TCVWR_R {
        TCVWR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "TCVWR3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvwr3](index.html) module"]
pub struct TCVWR3_SPEC;
impl crate::RegisterSpec for TCVWR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcvwr3::R](R) reader structure"]
impl crate::Readable for TCVWR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCVWR3 to value 0"]
impl crate::Resettable for TCVWR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
