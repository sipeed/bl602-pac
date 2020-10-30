#[doc = "Register `irom1_misr_dataout_1` reader"]
pub struct R(crate::R<IROM1_MISR_DATAOUT_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IROM1_MISR_DATAOUT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IROM1_MISR_DATAOUT_1_SPEC>> for R {
    fn from(reader: crate::R<IROM1_MISR_DATAOUT_1_SPEC>) -> Self {
        R(reader)
    }
}
impl R {}
#[doc = "irom1_misr_dataout_1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irom1_misr_dataout_1](index.html) module"]
pub struct IROM1_MISR_DATAOUT_1_SPEC;
impl crate::RegisterSpec for IROM1_MISR_DATAOUT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irom1_misr_dataout_1::R](R) reader structure"]
impl crate::Readable for IROM1_MISR_DATAOUT_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets irom1_misr_dataout_1 to value 0"]
impl crate::Resettable for IROM1_MISR_DATAOUT_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
