#[doc = "Register `RTC_TIME_H` reader"]
pub struct R(crate::R<RTC_TIME_H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIME_H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RTC_TIME_H_SPEC>> for R {
    fn from(reader: crate::R<RTC_TIME_H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIME_H` writer"]
pub struct W(crate::W<RTC_TIME_H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIME_H_SPEC>;
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
impl core::convert::From<crate::W<RTC_TIME_H_SPEC>> for W {
    fn from(writer: crate::W<RTC_TIME_H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rtc_time_latch` writer - "]
pub struct RTC_TIME_LATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_TIME_LATCH_W<'a> {
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
#[doc = "Field `rtc_time_latch_h` reader - "]
pub struct RTC_TIME_LATCH_H_R(crate::FieldReader<u8, u8>);
impl RTC_TIME_LATCH_H_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTC_TIME_LATCH_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_TIME_LATCH_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtc_time_latch_h(&self) -> RTC_TIME_LATCH_H_R {
        RTC_TIME_LATCH_H_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_time_latch(&mut self) -> RTC_TIME_LATCH_W {
        RTC_TIME_LATCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC_TIME_H.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_time_h](index.html) module"]
pub struct RTC_TIME_H_SPEC;
impl crate::RegisterSpec for RTC_TIME_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_time_h::R](R) reader structure"]
impl crate::Readable for RTC_TIME_H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_time_h::W](W) writer structure"]
impl crate::Writable for RTC_TIME_H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIME_H to value 0"]
impl crate::Resettable for RTC_TIME_H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
