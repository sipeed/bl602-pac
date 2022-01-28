#[doc = "Register `pwm2_period` reader"]
pub struct R(crate::R<PWM2_PERIOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM2_PERIOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM2_PERIOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM2_PERIOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwm2_period` writer"]
pub struct W(crate::W<PWM2_PERIOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM2_PERIOD_SPEC>;
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
impl From<crate::W<PWM2_PERIOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM2_PERIOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_period` reader - "]
pub struct PWM_PERIOD_R(crate::FieldReader<u16, u16>);
impl PWM_PERIOD_R {
    pub(crate) fn new(bits: u16) -> Self {
        PWM_PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_PERIOD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm_period` writer - "]
pub struct PWM_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_period(&self) -> PWM_PERIOD_R {
        PWM_PERIOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_period(&mut self) -> PWM_PERIOD_W {
        PWM_PERIOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm2_period.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm2_period](index.html) module"]
pub struct PWM2_PERIOD_SPEC;
impl crate::RegisterSpec for PWM2_PERIOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm2_period::R](R) reader structure"]
impl crate::Readable for PWM2_PERIOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm2_period::W](W) writer structure"]
impl crate::Writable for PWM2_PERIOD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pwm2_period to value 0"]
impl crate::Resettable for PWM2_PERIOD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
