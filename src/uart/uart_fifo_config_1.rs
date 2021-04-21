#[doc = "Register `uart_fifo_config_1` reader"]
pub struct R(crate::R<UART_FIFO_CONFIG_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_FIFO_CONFIG_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_FIFO_CONFIG_1_SPEC>> for R {
    fn from(reader: crate::R<UART_FIFO_CONFIG_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_fifo_config_1` writer"]
pub struct W(crate::W<UART_FIFO_CONFIG_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_FIFO_CONFIG_1_SPEC>;
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
impl core::convert::From<crate::W<UART_FIFO_CONFIG_1_SPEC>> for W {
    fn from(writer: crate::W<UART_FIFO_CONFIG_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rx_fifo_th` reader - "]
pub struct RX_FIFO_TH_R(crate::FieldReader<u8, u8>);
impl RX_FIFO_TH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_TH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_TH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_th` writer - "]
pub struct RX_FIFO_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `tx_fifo_th` reader - "]
pub struct TX_FIFO_TH_R(crate::FieldReader<u8, u8>);
impl TX_FIFO_TH_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_FIFO_TH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_TH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_th` writer - "]
pub struct TX_FIFO_TH_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_TH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `rx_fifo_cnt` reader - "]
pub struct RX_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl RX_FIFO_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_cnt` reader - "]
pub struct TX_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl TX_FIFO_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rx_fifo_th(&self) -> RX_FIFO_TH_R {
        RX_FIFO_TH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn tx_fifo_th(&self) -> TX_FIFO_TH_R {
        TX_FIFO_TH_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tx_fifo_cnt(&self) -> TX_FIFO_CNT_R {
        TX_FIFO_CNT_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28"]
    #[inline(always)]
    pub fn rx_fifo_th(&mut self) -> RX_FIFO_TH_W {
        RX_FIFO_TH_W { w: self }
    }
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn tx_fifo_th(&mut self) -> TX_FIFO_TH_W {
        TX_FIFO_TH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_fifo_config_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo_config_1](index.html) module"]
pub struct UART_FIFO_CONFIG_1_SPEC;
impl crate::RegisterSpec for UART_FIFO_CONFIG_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_fifo_config_1::R](R) reader structure"]
impl crate::Readable for UART_FIFO_CONFIG_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_fifo_config_1::W](W) writer structure"]
impl crate::Writable for UART_FIFO_CONFIG_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets uart_fifo_config_1 to value 0x20"]
impl crate::Resettable for UART_FIFO_CONFIG_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
