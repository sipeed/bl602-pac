#[doc = "Register `pwm4_interrupt` reader"]
pub struct R(crate::R<PWM4_INTERRUPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM4_INTERRUPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM4_INTERRUPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM4_INTERRUPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwm4_interrupt` writer"]
pub struct W(crate::W<PWM4_INTERRUPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM4_INTERRUPT_SPEC>;
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
impl From<crate::W<PWM4_INTERRUPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM4_INTERRUPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_int_enable` reader - "]
pub struct PWM_INT_ENABLE_R(crate::FieldReader<bool, bool>);
impl PWM_INT_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWM_INT_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_INT_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm_int_enable` writer - "]
pub struct PWM_INT_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INT_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `pwm_int_period_cnt` reader - "]
pub struct PWM_INT_PERIOD_CNT_R(crate::FieldReader<u16, u16>);
impl PWM_INT_PERIOD_CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        PWM_INT_PERIOD_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_INT_PERIOD_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm_int_period_cnt` writer - "]
pub struct PWM_INT_PERIOD_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_INT_PERIOD_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pwm_int_enable(&self) -> PWM_INT_ENABLE_R {
        PWM_INT_ENABLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_int_period_cnt(&self) -> PWM_INT_PERIOD_CNT_R {
        PWM_INT_PERIOD_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pwm_int_enable(&mut self) -> PWM_INT_ENABLE_W {
        PWM_INT_ENABLE_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_int_period_cnt(&mut self) -> PWM_INT_PERIOD_CNT_W {
        PWM_INT_PERIOD_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm4_interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm4_interrupt](index.html) module"]
pub struct PWM4_INTERRUPT_SPEC;
impl crate::RegisterSpec for PWM4_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm4_interrupt::R](R) reader structure"]
impl crate::Readable for PWM4_INTERRUPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm4_interrupt::W](W) writer structure"]
impl crate::Writable for PWM4_INTERRUPT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pwm4_interrupt to value 0"]
impl crate::Resettable for PWM4_INTERRUPT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
