#[doc = "Register `pucr2_hw` reader"]
pub struct R(crate::R<PUCR2_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR2_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PUCR2_HW_SPEC>> for R {
    fn from(reader: crate::R<PUCR2_HW_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "pucr2_hw.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr2_hw](index.html) module"]
pub struct PUCR2_HW_SPEC;
impl crate::RegisterSpec for PUCR2_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr2_hw::R](R) reader structure"]
impl crate::Readable for PUCR2_HW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets pucr2_hw to value 0"]
impl crate::Resettable for PUCR2_HW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
