#[doc = "Register `gpdac_tx_fifo_status` reader"]
pub struct R(crate::R<GPDAC_TX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_TX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPDAC_TX_FIFO_STATUS_SPEC>> for R {
    fn from(reader: crate::R<GPDAC_TX_FIFO_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpdac_tx_fifo_status` writer"]
pub struct W(crate::W<GPDAC_TX_FIFO_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDAC_TX_FIFO_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<GPDAC_TX_FIFO_STATUS_SPEC>> for W {
    fn from(writer: crate::W<GPDAC_TX_FIFO_STATUS_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `TxFifoWrPtr` writer - "]
pub struct TXFIFOWRPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFOWRPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
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
#[doc = "Field `TxFifoRdPtr` writer - "]
pub struct TXFIFORDPTR_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFIFORDPTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
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
#[doc = "Field `tx_cs` writer - "]
pub struct TX_CS_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
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
#[doc = "Field `tx_fifo_full` writer - "]
pub struct TX_FIFO_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_FULL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
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
#[doc = "Field `tx_fifo_empty` writer - "]
pub struct TX_FIFO_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_EMPTY_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
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
impl W {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn tx_fifo_wr_ptr(&mut self) -> TXFIFOWRPTR_W {
        TXFIFOWRPTR_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tx_fifo_rd_ptr(&mut self) -> TXFIFORDPTR_W {
        TXFIFORDPTR_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tx_cs(&mut self) -> TX_CS_W {
        TX_CS_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tx_fifo_full(&mut self) -> TX_FIFO_FULL_W {
        TX_FIFO_FULL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tx_fifo_empty(&mut self) -> TX_FIFO_EMPTY_W {
        TX_FIFO_EMPTY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpdac_tx_fifo_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_tx_fifo_status](index.html) module"]
pub struct GPDAC_TX_FIFO_STATUS_SPEC;
impl crate::RegisterSpec for GPDAC_TX_FIFO_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_tx_fifo_status::R](R) reader structure"]
impl crate::Readable for GPDAC_TX_FIFO_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdac_tx_fifo_status::W](W) writer structure"]
impl crate::Writable for GPDAC_TX_FIFO_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpdac_tx_fifo_status to value 0"]
impl crate::Resettable for GPDAC_TX_FIFO_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
