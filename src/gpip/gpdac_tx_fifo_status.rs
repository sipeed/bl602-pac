#[doc = "Register `gpdac_tx_fifo_status` reader"]
pub struct R(crate::R<GPDAC_TX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_TX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDAC_TX_FIFO_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDAC_TX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TxFifoWrPtr` reader - "]
pub struct TXFIFOWRPTR_R(crate::FieldReader<u8, u8>);
impl TXFIFOWRPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFOWRPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFOWRPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TxFifoRdPtr` reader - "]
pub struct TXFIFORDPTR_R(crate::FieldReader<u8, u8>);
impl TXFIFORDPTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXFIFORDPTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFIFORDPTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_cs` reader - "]
pub struct TX_CS_R(crate::FieldReader<u8, u8>);
impl TX_CS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_CS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_full` reader - "]
pub struct TX_FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_empty` reader - "]
pub struct TX_FIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_EMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tx_fifo_wr_ptr(&self) -> TXFIFOWRPTR_R {
        TXFIFOWRPTR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_fifo_rd_ptr(&self) -> TXFIFORDPTR_R {
        TXFIFORDPTR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_cs(&self) -> TX_CS_R {
        TX_CS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_full(&self) -> TX_FIFO_FULL_R {
        TX_FIFO_FULL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TX_FIFO_EMPTY_R {
        TX_FIFO_EMPTY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "gpdac_tx_fifo_status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_tx_fifo_status](index.html) module"]
pub struct GPDAC_TX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for GPDAC_TX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_tx_fifo_status::R](R) reader structure"]
impl crate::Readable for GPDAC_TX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets gpdac_tx_fifo_status to value 0x40"]
impl crate::Resettable for GPDAC_TX_FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
