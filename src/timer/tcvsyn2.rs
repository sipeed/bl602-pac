#[doc = "Register `TCVSYN2` reader"]
pub struct R(crate::R<TCVSYN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCVSYN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TCVSYN2_SPEC>> for R {
    fn from(reader: crate::R<TCVSYN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `tcvsyn2` reader - "]
pub struct TCVSYN2_R(crate::FieldReader<u32, u32>);
impl TCVSYN2_R {
    pub(crate) fn new(bits: u32) -> Self {
        TCVSYN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCVSYN2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn2(&self) -> TCVSYN2_R {
        TCVSYN2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "TCVSYN2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvsyn2](index.html) module"]
pub struct TCVSYN2_SPEC;
impl crate::RegisterSpec for TCVSYN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcvsyn2::R](R) reader structure"]
impl crate::Readable for TCVSYN2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCVSYN2 to value 0"]
impl crate::Resettable for TCVSYN2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
