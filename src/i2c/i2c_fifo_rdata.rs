#[doc = "Register `i2c_fifo_rdata` reader"]
pub struct R(crate::R<I2C_FIFO_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_FIFO_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_FIFO_RDATA_SPEC>> for R {
    fn from(reader: crate::R<I2C_FIFO_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `i2c_fifo_rdata` reader - "]
pub struct I2C_FIFO_RDATA_R(crate::FieldReader<u32, u32>);
impl I2C_FIFO_RDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        I2C_FIFO_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_FIFO_RDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2c_fifo_rdata(&self) -> I2C_FIFO_RDATA_R {
        I2C_FIFO_RDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "i2c_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_fifo_rdata](index.html) module"]
pub struct I2C_FIFO_RDATA_SPEC;
impl crate::RegisterSpec for I2C_FIFO_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_fifo_rdata::R](R) reader structure"]
impl crate::Readable for I2C_FIFO_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets i2c_fifo_rdata to value 0"]
impl crate::Resettable for I2C_FIFO_RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
