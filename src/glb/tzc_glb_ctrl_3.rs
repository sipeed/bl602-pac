#[doc = "Register `tzc_glb_ctrl_3` reader"]
pub struct R(crate::R<TZC_GLB_CTRL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_GLB_CTRL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TZC_GLB_CTRL_3_SPEC>> for R {
    fn from(reader: crate::R<TZC_GLB_CTRL_3_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "tzc_glb_ctrl_3.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_3](index.html) module"]
pub struct TZC_GLB_CTRL_3_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_glb_ctrl_3::R](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_3 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
