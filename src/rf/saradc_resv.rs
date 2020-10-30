#[doc = "Register `saradc_resv` reader"]
pub struct R(crate::R<SARADC_RESV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SARADC_RESV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SARADC_RESV_SPEC>> for R {
    fn from(reader: crate::R<SARADC_RESV_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "SARADC Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saradc_resv](index.html) module"]
pub struct SARADC_RESV_SPEC;
impl crate::RegisterSpec for SARADC_RESV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saradc_resv::R](R) reader structure"]
impl crate::Readable for SARADC_RESV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets saradc_resv to value 0"]
impl crate::Resettable for SARADC_RESV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
