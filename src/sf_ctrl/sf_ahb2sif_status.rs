#[doc = "Register `sf_ahb2sif_status` reader"]
pub struct R(crate::R<SF_AHB2SIF_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_AHB2SIF_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SF_AHB2SIF_STATUS_SPEC>> for R {
    fn from(reader: crate::R<SF_AHB2SIF_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sf_ahb2sif_status` reader - "]
pub struct SF_AHB2SIF_STATUS_R(crate::FieldReader<u32, u32>);
impl SF_AHB2SIF_STATUS_R {
    pub(crate) fn new(bits: u32) -> Self {
        SF_AHB2SIF_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AHB2SIF_STATUS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_ahb2sif_status(&self) -> SF_AHB2SIF_STATUS_R {
        SF_AHB2SIF_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "sf_ahb2sif_status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ahb2sif_status](index.html) module"]
pub struct SF_AHB2SIF_STATUS_SPEC;
impl crate::RegisterSpec for SF_AHB2SIF_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ahb2sif_status::R](R) reader structure"]
impl crate::Readable for SF_AHB2SIF_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets sf_ahb2sif_status to value 0x1000_0003"]
impl crate::Resettable for SF_AHB2SIF_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_0003
    }
}
