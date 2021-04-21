#[doc = "Register `se_sha_0_hash_h_1` reader"]
pub struct R(crate::R<SE_SHA_0_HASH_H_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_SHA_0_HASH_H_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_SHA_0_HASH_H_1_SPEC>> for R {
    fn from(reader: crate::R<SE_SHA_0_HASH_H_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `se_sha_0_hash_h_1` reader - "]
pub struct SE_SHA_0_HASH_H_1_R(crate::FieldReader<u32, u32>);
impl SE_SHA_0_HASH_H_1_R {
    pub(crate) fn new(bits: u32) -> Self {
        SE_SHA_0_HASH_H_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_HASH_H_1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn se_sha_0_hash_h_1(&self) -> SE_SHA_0_HASH_H_1_R {
        SE_SHA_0_HASH_H_1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "se_sha_0_hash_h_1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_hash_h_1](index.html) module"]
pub struct SE_SHA_0_HASH_H_1_SPEC;
impl crate::RegisterSpec for SE_SHA_0_HASH_H_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_sha_0_hash_h_1::R](R) reader structure"]
impl crate::Readable for SE_SHA_0_HASH_H_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets se_sha_0_hash_h_1 to value 0"]
impl crate::Resettable for SE_SHA_0_HASH_H_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
