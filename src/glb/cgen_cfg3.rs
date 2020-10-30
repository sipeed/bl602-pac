#[doc = "Register `cgen_cfg3` reader"]
pub struct R(crate::R<CGEN_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CGEN_CFG3_SPEC>> for R {
    fn from(reader: crate::R<CGEN_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "cgen_cfg3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg3](index.html) module"]
pub struct CGEN_CFG3_SPEC;
impl crate::RegisterSpec for CGEN_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg3::R](R) reader structure"]
impl crate::Readable for CGEN_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets cgen_cfg3 to value 0"]
impl crate::Resettable for CGEN_CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
