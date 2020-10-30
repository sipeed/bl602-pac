#[doc = "Register `rf_base_ctrl2` reader"]
pub struct R(crate::R<RF_BASE_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_BASE_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RF_BASE_CTRL2_SPEC>> for R {
    fn from(reader: crate::R<RF_BASE_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "ZRF Control register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_base_ctrl2](index.html) module"]
pub struct RF_BASE_CTRL2_SPEC;
impl crate::RegisterSpec for RF_BASE_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_base_ctrl2::R](R) reader structure"]
impl crate::Readable for RF_BASE_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rf_base_ctrl2 to value 0"]
impl crate::Resettable for RF_BASE_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
