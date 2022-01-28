#[doc = "Register `cci_rdata` reader"]
pub struct R(crate::R<CCI_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCI_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCI_RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCI_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `apb_cci_rdata` reader - "]
pub struct APB_CCI_RDATA_R(crate::FieldReader<u32, u32>);
impl APB_CCI_RDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        APB_CCI_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APB_CCI_RDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn apb_cci_rdata(&self) -> APB_CCI_RDATA_R {
        APB_CCI_RDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "cci_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_rdata](index.html) module"]
pub struct CCI_RDATA_SPEC;
impl crate::RegisterSpec for CCI_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cci_rdata::R](R) reader structure"]
impl crate::Readable for CCI_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets cci_rdata to value 0"]
impl crate::Resettable for CCI_RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
