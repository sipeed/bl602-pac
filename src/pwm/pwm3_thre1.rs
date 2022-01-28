#[doc = "Register `pwm3_thre1` reader"]
pub struct R(crate::R<PWM3_THRE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWM3_THRE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWM3_THRE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWM3_THRE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pwm3_thre1` writer"]
pub struct W(crate::W<PWM3_THRE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWM3_THRE1_SPEC>;
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
impl From<crate::W<PWM3_THRE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWM3_THRE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwm_thre1` reader - "]
pub struct PWM_THRE1_R(crate::FieldReader<u16, u16>);
impl PWM_THRE1_R {
    pub(crate) fn new(bits: u16) -> Self {
        PWM_THRE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWM_THRE1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwm_thre1` writer - "]
pub struct PWM_THRE1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWM_THRE1_W<'a> {
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
    pub fn pwm_thre1(&self) -> PWM_THRE1_R {
        PWM_THRE1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm_thre1(&mut self) -> PWM_THRE1_W {
        PWM_THRE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pwm3_thre1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwm3_thre1](index.html) module"]
pub struct PWM3_THRE1_SPEC;
impl crate::RegisterSpec for PWM3_THRE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwm3_thre1::R](R) reader structure"]
impl crate::Readable for PWM3_THRE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwm3_thre1::W](W) writer structure"]
impl crate::Writable for PWM3_THRE1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pwm3_thre1 to value 0"]
impl crate::Resettable for PWM3_THRE1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
