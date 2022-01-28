#[doc = "Register `TCER` reader"]
pub struct R(crate::R<TCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCER` writer"]
pub struct W(crate::W<TCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCER_SPEC>;
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
impl From<crate::W<TCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer3_en` reader - "]
pub struct TIMER3_EN_R(crate::FieldReader<bool, bool>);
impl TIMER3_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER3_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER3_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer3_en` writer - "]
pub struct TIMER3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_EN_W<'a> {
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
#[doc = "Field `timer2_en` reader - "]
pub struct TIMER2_EN_R(crate::FieldReader<bool, bool>);
impl TIMER2_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer2_en` writer - "]
pub struct TIMER2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_EN_W<'a> {
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
impl R {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_en(&self) -> TIMER3_EN_R {
        TIMER3_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_en(&self) -> TIMER2_EN_R {
        TIMER2_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_en(&mut self) -> TIMER3_EN_W {
        TIMER3_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_en(&mut self) -> TIMER2_EN_W {
        TIMER2_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCER.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcer](index.html) module"]
pub struct TCER_SPEC;
impl crate::RegisterSpec for TCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcer::R](R) reader structure"]
impl crate::Readable for TCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcer::W](W) writer structure"]
impl crate::Writable for TCER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCER to value 0"]
impl crate::Resettable for TCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
