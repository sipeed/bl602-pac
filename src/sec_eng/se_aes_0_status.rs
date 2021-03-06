#[doc = "Register `se_aes_0_status` reader"]
pub struct R(crate::R<SE_AES_0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_AES_0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_AES_0_STATUS_SPEC>> for R {
    fn from(reader: crate::R<SE_AES_0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `se_aes_0_status` reader - "]
pub struct SE_AES_0_STATUS_R(crate::FieldReader<u32, u32>);
impl SE_AES_0_STATUS_R {
    pub(crate) fn new(bits: u32) -> Self {
        SE_AES_0_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_0_STATUS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_aes_0_status(&self) -> SE_AES_0_STATUS_R {
        SE_AES_0_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "se_aes_0_status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_status](index.html) module"]
pub struct SE_AES_0_STATUS_SPEC;
impl crate::RegisterSpec for SE_AES_0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_aes_0_status::R](R) reader structure"]
impl crate::Readable for SE_AES_0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets se_aes_0_status to value 0x0100"]
impl crate::Resettable for SE_AES_0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
