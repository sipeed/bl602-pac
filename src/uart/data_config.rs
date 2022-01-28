#[doc = "Register `data_config` reader"]
pub struct R(crate::R<DATA_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `data_config` writer"]
pub struct W(crate::W<DATA_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_CONFIG_SPEC>;
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
impl From<crate::W<DATA_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_uart_bit_inv` reader - "]
pub struct CR_UART_BIT_INV_R(crate::FieldReader<bool, bool>);
impl CR_UART_BIT_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_UART_BIT_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UART_BIT_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_uart_bit_inv` writer - "]
pub struct CR_UART_BIT_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UART_BIT_INV_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_uart_bit_inv(&self) -> CR_UART_BIT_INV_R {
        CR_UART_BIT_INV_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_uart_bit_inv(&mut self) -> CR_UART_BIT_INV_W {
        CR_UART_BIT_INV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "data_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_config](index.html) module"]
pub struct DATA_CONFIG_SPEC;
impl crate::RegisterSpec for DATA_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_config::R](R) reader structure"]
impl crate::Readable for DATA_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_config::W](W) writer structure"]
impl crate::Writable for DATA_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets data_config to value 0"]
impl crate::Resettable for DATA_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
