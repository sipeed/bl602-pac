#[doc = "Register `uart_fifo_config_0` reader"]
pub struct R(crate::R<UART_FIFO_CONFIG_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_FIFO_CONFIG_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_FIFO_CONFIG_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_FIFO_CONFIG_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_fifo_config_0` writer"]
pub struct W(crate::W<UART_FIFO_CONFIG_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_FIFO_CONFIG_0_SPEC>;
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
impl From<crate::W<UART_FIFO_CONFIG_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART_FIFO_CONFIG_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_fifo_underflow` reader - "]
pub struct RX_FIFO_UNDERFLOW_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_UNDERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_UNDERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_UNDERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_overflow` reader - "]
pub struct RX_FIFO_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_OVERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_underflow` reader - "]
pub struct TX_FIFO_UNDERFLOW_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_UNDERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_UNDERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_UNDERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_overflow` reader - "]
pub struct TX_FIFO_OVERFLOW_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_OVERFLOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_clr` reader - "]
pub struct RX_FIFO_CLR_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_clr` writer - "]
pub struct RX_FIFO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `tx_fifo_clr` reader - "]
pub struct TX_FIFO_CLR_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_clr` writer - "]
pub struct TX_FIFO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_CLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `uart_dma_rx_en` reader - "]
pub struct UART_DMA_RX_EN_R(crate::FieldReader<bool, bool>);
impl UART_DMA_RX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART_DMA_RX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_DMA_RX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_dma_rx_en` writer - "]
pub struct UART_DMA_RX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DMA_RX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `uart_dma_tx_en` reader - "]
pub struct UART_DMA_TX_EN_R(crate::FieldReader<bool, bool>);
impl UART_DMA_TX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UART_DMA_TX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_DMA_TX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_dma_tx_en` writer - "]
pub struct UART_DMA_TX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DMA_TX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rx_fifo_underflow(&self) -> RX_FIFO_UNDERFLOW_R {
        RX_FIFO_UNDERFLOW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOW_R {
        RX_FIFO_OVERFLOW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tx_fifo_underflow(&self) -> TX_FIFO_UNDERFLOW_R {
        TX_FIFO_UNDERFLOW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tx_fifo_overflow(&self) -> TX_FIFO_OVERFLOW_R {
        TX_FIFO_OVERFLOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_clr(&self) -> RX_FIFO_CLR_R {
        RX_FIFO_CLR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clr(&self) -> TX_FIFO_CLR_R {
        TX_FIFO_CLR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uart_dma_rx_en(&self) -> UART_DMA_RX_EN_R {
        UART_DMA_RX_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uart_dma_tx_en(&self) -> UART_DMA_TX_EN_R {
        UART_DMA_TX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rx_fifo_clr(&mut self) -> RX_FIFO_CLR_W {
        RX_FIFO_CLR_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_fifo_clr(&mut self) -> TX_FIFO_CLR_W {
        TX_FIFO_CLR_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn uart_dma_rx_en(&mut self) -> UART_DMA_RX_EN_W {
        UART_DMA_RX_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn uart_dma_tx_en(&mut self) -> UART_DMA_TX_EN_W {
        UART_DMA_TX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_fifo_config_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo_config_0](index.html) module"]
pub struct UART_FIFO_CONFIG_0_SPEC;
impl crate::RegisterSpec for UART_FIFO_CONFIG_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_fifo_config_0::R](R) reader structure"]
impl crate::Readable for UART_FIFO_CONFIG_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_fifo_config_0::W](W) writer structure"]
impl crate::Writable for UART_FIFO_CONFIG_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets uart_fifo_config_0 to value 0"]
impl crate::Resettable for UART_FIFO_CONFIG_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
