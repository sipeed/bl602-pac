#[doc = "Register `WVR` reader"]
pub struct R(crate::R<WVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WVR_SPEC>> for R {
    fn from(reader: crate::R<WVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `wvr` reader - "]
pub struct WVR_R(crate::FieldReader<u16, u16>);
impl WVR_R {
    pub(crate) fn new(bits: u16) -> Self {
        WVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WVR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wvr(&self) -> WVR_R {
        WVR_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "WVR.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wvr](index.html) module"]
pub struct WVR_SPEC;
impl crate::RegisterSpec for WVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wvr::R](R) reader structure"]
impl crate::Readable for WVR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WVR to value 0"]
impl crate::Resettable for WVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
