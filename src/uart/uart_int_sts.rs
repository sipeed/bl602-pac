#[doc = "Register `uart_int_sts` reader"]
pub struct R(crate::R<UART_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_INT_STS_SPEC>> for R {
    fn from(reader: crate::R<UART_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `urx_fer_int` reader - "]
pub struct URX_FER_INT_R(crate::FieldReader<bool, bool>);
impl URX_FER_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        URX_FER_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URX_FER_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `utx_fer_int` reader - "]
pub struct UTX_FER_INT_R(crate::FieldReader<bool, bool>);
impl UTX_FER_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTX_FER_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTX_FER_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `urx_pce_int` reader - "]
pub struct URX_PCE_INT_R(crate::FieldReader<bool, bool>);
impl URX_PCE_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        URX_PCE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URX_PCE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `urx_rto_int` reader - "]
pub struct URX_RTO_INT_R(crate::FieldReader<bool, bool>);
impl URX_RTO_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        URX_RTO_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URX_RTO_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `urx_fifo_int` reader - "]
pub struct URX_FIFO_INT_R(crate::FieldReader<bool, bool>);
impl URX_FIFO_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        URX_FIFO_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URX_FIFO_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `utx_fifo_int` reader - "]
pub struct UTX_FIFO_INT_R(crate::FieldReader<bool, bool>);
impl UTX_FIFO_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTX_FIFO_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTX_FIFO_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `urx_end_int` reader - "]
pub struct URX_END_INT_R(crate::FieldReader<bool, bool>);
impl URX_END_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        URX_END_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URX_END_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `utx_end_int` reader - "]
pub struct UTX_END_INT_R(crate::FieldReader<bool, bool>);
impl UTX_END_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTX_END_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTX_END_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn urx_fer_int(&self) -> URX_FER_INT_R {
        URX_FER_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn utx_fer_int(&self) -> UTX_FER_INT_R {
        UTX_FER_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn urx_pce_int(&self) -> URX_PCE_INT_R {
        URX_PCE_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn urx_rto_int(&self) -> URX_RTO_INT_R {
        URX_RTO_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn urx_fifo_int(&self) -> URX_FIFO_INT_R {
        URX_FIFO_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn utx_fifo_int(&self) -> UTX_FIFO_INT_R {
        UTX_FIFO_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn urx_end_int(&self) -> URX_END_INT_R {
        URX_END_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utx_end_int(&self) -> UTX_END_INT_R {
        UTX_END_INT_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "UART interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_sts](index.html) module"]
pub struct UART_INT_STS_SPEC;
impl crate::RegisterSpec for UART_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_int_sts::R](R) reader structure"]
impl crate::Readable for UART_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets uart_int_sts to value 0"]
impl crate::Resettable for UART_INT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
