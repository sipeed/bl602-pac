#[doc = "Register `uart_fifo_wdata` writer"]
pub struct W(crate::W<UART_FIFO_WDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_FIFO_WDATA_SPEC>;
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
impl core::convert::From<crate::W<UART_FIFO_WDATA_SPEC>> for W {
    fn from(writer: crate::W<UART_FIFO_WDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart_fifo_wdata` writer - "]
pub struct UART_FIFO_WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_FIFO_WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn uart_fifo_wdata(&mut self) -> UART_FIFO_WDATA_W {
        UART_FIFO_WDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_fifo_wdata.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_fifo_wdata](index.html) module"]
pub struct UART_FIFO_WDATA_SPEC;
impl crate::RegisterSpec for UART_FIFO_WDATA_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uart_fifo_wdata::W](W) writer structure"]
impl crate::Writable for UART_FIFO_WDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets uart_fifo_wdata to value 0"]
impl crate::Resettable for UART_FIFO_WDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
