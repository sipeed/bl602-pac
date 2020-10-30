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
#[doc = "Register `uart_status` writer"]
pub struct W(crate::W<UART_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_STATUS_SPEC>;
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
impl core::convert::From<crate::W<UART_STATUS_SPEC>> for W {
    fn from(writer: crate::W<UART_STATUS_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `sts_urx_bus_busy` writer - "]
pub struct STS_URX_BUS_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_URX_BUS_BUSY_W<'a> {
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
#[doc = "Field `sts_utx_bus_busy` writer - "]
pub struct STS_UTX_BUS_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> STS_UTX_BUS_BUSY_W<'a> {
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
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sts_urx_bus_busy(&mut self) -> STS_URX_BUS_BUSY_W {
        STS_URX_BUS_BUSY_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sts_utx_bus_busy(&mut self) -> STS_UTX_BUS_BUSY_W {
        STS_UTX_BUS_BUSY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "uart_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_status](index.html) module"]
pub struct UART_STATUS_SPEC;
impl crate::RegisterSpec for UART_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_status::R](R) reader structure"]
impl crate::Readable for UART_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_status::W](W) writer structure"]
impl crate::Writable for UART_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets uart_status to value 0"]
impl crate::Resettable for UART_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
