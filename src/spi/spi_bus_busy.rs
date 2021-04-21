#[doc = "Register `spi_bus_busy` reader"]
pub struct R(crate::R<SPI_BUS_BUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_BUS_BUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPI_BUS_BUSY_SPEC>> for R {
    fn from(reader: crate::R<SPI_BUS_BUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sts_spi_bus_busy` reader - "]
pub struct STS_SPI_BUS_BUSY_R(crate::FieldReader<bool, bool>);
impl STS_SPI_BUS_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_SPI_BUS_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_SPI_BUS_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_spi_bus_busy(&self) -> STS_SPI_BUS_BUSY_R {
        STS_SPI_BUS_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "spi_bus_busy.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_bus_busy](index.html) module"]
pub struct SPI_BUS_BUSY_SPEC;
impl crate::RegisterSpec for SPI_BUS_BUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_bus_busy::R](R) reader structure"]
impl crate::Readable for SPI_BUS_BUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets spi_bus_busy to value 0"]
impl crate::Resettable for SPI_BUS_BUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
