#[doc = "Register `pmip_mv2aon` reader"]
pub struct R(crate::R<PMIP_MV2AON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMIP_MV2AON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PMIP_MV2AON_SPEC>> for R {
    fn from(reader: crate::R<PMIP_MV2AON_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "pmip_mv2aon.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmip_mv2aon](index.html) module"]
pub struct PMIP_MV2AON_SPEC;
impl crate::RegisterSpec for PMIP_MV2AON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmip_mv2aon::R](R) reader structure"]
impl crate::Readable for PMIP_MV2AON_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets pmip_mv2aon to value 0"]
impl crate::Resettable for PMIP_MV2AON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
