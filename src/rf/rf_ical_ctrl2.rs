#[doc = "Register `rf_ical_ctrl2` reader"]
pub struct R(crate::R<RF_ICAL_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_ICAL_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RF_ICAL_CTRL2_SPEC>> for R {
    fn from(reader: crate::R<RF_ICAL_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_ical_ctrl2` writer"]
pub struct W(crate::W<RF_ICAL_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_ICAL_CTRL2_SPEC>;
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
impl core::convert::From<crate::W<RF_ICAL_CTRL2_SPEC>> for W {
    fn from(writer: crate::W<RF_ICAL_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_ical_period_n` reader - "]
pub struct RF_ICAL_PERIOD_N_R(crate::FieldReader<u16, u16>);
impl RF_ICAL_PERIOD_N_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_ICAL_PERIOD_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_ICAL_PERIOD_N_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ical_period_n` writer - "]
pub struct RF_ICAL_PERIOD_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_ICAL_PERIOD_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_ical_period_n(&self) -> RF_ICAL_PERIOD_N_R {
        RF_ICAL_PERIOD_N_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_ical_period_n(&mut self) -> RF_ICAL_PERIOD_N_W {
        RF_ICAL_PERIOD_N_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_ical_ctrl2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_ical_ctrl2](index.html) module"]
pub struct RF_ICAL_CTRL2_SPEC;
impl crate::RegisterSpec for RF_ICAL_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_ical_ctrl2::R](R) reader structure"]
impl crate::Readable for RF_ICAL_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_ical_ctrl2::W](W) writer structure"]
impl crate::Writable for RF_ICAL_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_ical_ctrl2 to value 0"]
impl crate::Resettable for RF_ICAL_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
