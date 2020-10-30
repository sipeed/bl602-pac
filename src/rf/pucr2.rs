#[doc = "Register `pucr2` reader"]
pub struct R(crate::R<PUCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PUCR2_SPEC>> for R {
    fn from(reader: crate::R<PUCR2_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "pucr2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr2](index.html) module"]
pub struct PUCR2_SPEC;
impl crate::RegisterSpec for PUCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr2::R](R) reader structure"]
impl crate::Readable for PUCR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets pucr2 to value 0"]
impl crate::Resettable for PUCR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
