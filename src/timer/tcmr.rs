#[doc = "Register `TCMR` reader"]
pub struct R(crate::R<TCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCMR` writer"]
pub struct W(crate::W<TCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCMR_SPEC>;
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
impl From<crate::W<TCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timer3_mode` reader - "]
pub struct TIMER3_MODE_R(crate::FieldReader<bool, bool>);
impl TIMER3_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER3_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER3_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer3_mode` writer - "]
pub struct TIMER3_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_MODE_W<'a> {
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
#[doc = "Field `timer2_mode` reader - "]
pub struct TIMER2_MODE_R(crate::FieldReader<bool, bool>);
impl TIMER2_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer2_mode` writer - "]
pub struct TIMER2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_MODE_W<'a> {
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
    pub fn timer3_mode(&self) -> TIMER3_MODE_R {
        TIMER3_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_mode(&self) -> TIMER2_MODE_R {
        TIMER2_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn timer3_mode(&mut self) -> TIMER3_MODE_W {
        TIMER3_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn timer2_mode(&mut self) -> TIMER2_MODE_W {
        TIMER2_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCMR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcmr](index.html) module"]
pub struct TCMR_SPEC;
impl crate::RegisterSpec for TCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcmr::R](R) reader structure"]
impl crate::Readable for TCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcmr::W](W) writer structure"]
impl crate::Writable for TCMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCMR to value 0"]
impl crate::Resettable for TCMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
