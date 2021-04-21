#[doc = "Register `spi_fifo_rdata` reader"]
pub struct R(crate::R<SPI_FIFO_RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_FIFO_RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPI_FIFO_RDATA_SPEC>> for R {
    fn from(reader: crate::R<SPI_FIFO_RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `spi_fifo_rdata` reader - "]
pub struct SPI_FIFO_RDATA_R(crate::FieldReader<u32, u32>);
impl SPI_FIFO_RDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        SPI_FIFO_RDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_FIFO_RDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_fifo_rdata(&self) -> SPI_FIFO_RDATA_R {
        SPI_FIFO_RDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "spi_fifo_rdata.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_fifo_rdata](index.html) module"]
pub struct SPI_FIFO_RDATA_SPEC;
impl crate::RegisterSpec for SPI_FIFO_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_fifo_rdata::R](R) reader structure"]
impl crate::Readable for SPI_FIFO_RDATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets spi_fifo_rdata to value 0"]
impl crate::Resettable for SPI_FIFO_RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
