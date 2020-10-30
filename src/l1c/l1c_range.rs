#[doc = "Register `l1c_range` reader"]
pub struct R(crate::R<L1C_RANGE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1C_RANGE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<L1C_RANGE_SPEC>> for R {
    fn from(reader: crate::R<L1C_RANGE_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "l1c_range.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_range](index.html) module"]
pub struct L1C_RANGE_SPEC;
impl crate::RegisterSpec for L1C_RANGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1c_range::R](R) reader structure"]
impl crate::Readable for L1C_RANGE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets l1c_range to value 0"]
impl crate::Resettable for L1C_RANGE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
