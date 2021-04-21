#[doc = "Register `led_driver` reader"]
pub struct R(crate::R<LED_DRIVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LED_DRIVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LED_DRIVER_SPEC>> for R {
    fn from(reader: crate::R<LED_DRIVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `led_driver` writer"]
pub struct W(crate::W<LED_DRIVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LED_DRIVER_SPEC>;
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
impl core::convert::From<crate::W<LED_DRIVER_SPEC>> for W {
    fn from(writer: crate::W<LED_DRIVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_leddrv` reader - "]
pub struct PU_LEDDRV_R(crate::FieldReader<bool, bool>);
impl PU_LEDDRV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_LEDDRV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_LEDDRV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_leddrv` writer - "]
pub struct PU_LEDDRV_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LEDDRV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `ir_rx_gpio_sel` reader - "]
pub struct IR_RX_GPIO_SEL_R(crate::FieldReader<u8, u8>);
impl IR_RX_GPIO_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        IR_RX_GPIO_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IR_RX_GPIO_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ir_rx_gpio_sel` writer - "]
pub struct IR_RX_GPIO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IR_RX_GPIO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `leddrv_ibias` reader - "]
pub struct LEDDRV_IBIAS_R(crate::FieldReader<u8, u8>);
impl LEDDRV_IBIAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEDDRV_IBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEDDRV_IBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `leddrv_ibias` writer - "]
pub struct LEDDRV_IBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDDRV_IBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `led_din_polarity_sel` reader - "]
pub struct LED_DIN_POLARITY_SEL_R(crate::FieldReader<bool, bool>);
impl LED_DIN_POLARITY_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LED_DIN_POLARITY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LED_DIN_POLARITY_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `led_din_polarity_sel` writer - "]
pub struct LED_DIN_POLARITY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_DIN_POLARITY_SEL_W<'a> {
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
#[doc = "Field `led_din_sel` reader - "]
pub struct LED_DIN_SEL_R(crate::FieldReader<bool, bool>);
impl LED_DIN_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LED_DIN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LED_DIN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `led_din_sel` writer - "]
pub struct LED_DIN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_DIN_SEL_W<'a> {
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
#[doc = "Field `led_din_reg` reader - "]
pub struct LED_DIN_REG_R(crate::FieldReader<bool, bool>);
impl LED_DIN_REG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LED_DIN_REG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LED_DIN_REG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `led_din_reg` writer - "]
pub struct LED_DIN_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> LED_DIN_REG_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_leddrv(&self) -> PU_LEDDRV_R {
        PU_LEDDRV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ir_rx_gpio_sel(&self) -> IR_RX_GPIO_SEL_R {
        IR_RX_GPIO_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn leddrv_ibias(&self) -> LEDDRV_IBIAS_R {
        LEDDRV_IBIAS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_din_polarity_sel(&self) -> LED_DIN_POLARITY_SEL_R {
        LED_DIN_POLARITY_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn led_din_sel(&self) -> LED_DIN_SEL_R {
        LED_DIN_SEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_din_reg(&self) -> LED_DIN_REG_R {
        LED_DIN_REG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_leddrv(&mut self) -> PU_LEDDRV_W {
        PU_LEDDRV_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ir_rx_gpio_sel(&mut self) -> IR_RX_GPIO_SEL_W {
        IR_RX_GPIO_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn leddrv_ibias(&mut self) -> LEDDRV_IBIAS_W {
        LEDDRV_IBIAS_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn led_din_polarity_sel(&mut self) -> LED_DIN_POLARITY_SEL_W {
        LED_DIN_POLARITY_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn led_din_sel(&mut self) -> LED_DIN_SEL_W {
        LED_DIN_SEL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn led_din_reg(&mut self) -> LED_DIN_REG_W {
        LED_DIN_REG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "led_driver.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [led_driver](index.html) module"]
pub struct LED_DRIVER_SPEC;
impl crate::RegisterSpec for LED_DRIVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [led_driver::R](R) reader structure"]
impl crate::Readable for LED_DRIVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [led_driver::W](W) writer structure"]
impl crate::Writable for LED_DRIVER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets led_driver to value 0x80"]
impl crate::Resettable for LED_DRIVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
