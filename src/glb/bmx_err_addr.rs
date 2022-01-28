#[doc = "Register `bmx_err_addr` reader"]
pub struct R(crate::R<BMX_ERR_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_ERR_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_ERR_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_ERR_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `bmx_err_addr` reader - "]
pub struct BMX_ERR_ADDR_R(crate::FieldReader<u32, u32>);
impl BMX_ERR_ADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        BMX_ERR_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMX_ERR_ADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bmx_err_addr(&self) -> BMX_ERR_ADDR_R {
        BMX_ERR_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "bmx_err_addr.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_err_addr](index.html) module"]
pub struct BMX_ERR_ADDR_SPEC;
impl crate::RegisterSpec for BMX_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_err_addr::R](R) reader structure"]
impl crate::Readable for BMX_ERR_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets bmx_err_addr to value 0"]
impl crate::Resettable for BMX_ERR_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
