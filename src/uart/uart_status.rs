#[doc = "Register `uart_status` reader"]
pub struct R(crate::R<UART_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_STATUS_SPEC>> for R {
    fn from(reader: crate::R<UART_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `sts_urx_bus_busy` reader - "]
pub struct STS_URX_BUS_BUSY_R(crate::FieldReader<bool, bool>);
impl STS_URX_BUS_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_URX_BUS_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_URX_BUS_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sts_utx_bus_busy` reader - "]
pub struct STS_UTX_BUS_BUSY_R(crate::FieldReader<bool, bool>);
impl STS_UTX_BUS_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        STS_UTX_BUS_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STS_UTX_BUS_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sts_urx_bus_busy(&self) -> STS_URX_BUS_BUSY_R {
        STS_URX_BUS_BUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_utx_bus_busy(&self) -> STS_UTX_BUS_BUSY_R {
        STS_UTX_BUS_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "uart_status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_status](index.html) module"]
pub struct UART_STATUS_SPEC;
impl crate::RegisterSpec for UART_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_status::R](R) reader structure"]
impl crate::Readable for UART_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets uart_status to value 0"]
impl crate::Resettable for UART_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
