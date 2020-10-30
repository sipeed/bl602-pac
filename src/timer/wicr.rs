#[doc = "Register `WICR` reader"]
pub struct R(crate::R<WICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WICR_SPEC>> for R {
    fn from(reader: crate::R<WICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WICR` writer"]
pub struct W(crate::W<WICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WICR_SPEC>;
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
impl core::convert::From<crate::W<WICR_SPEC>> for W {
    fn from(writer: crate::W<WICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wiclr` reader - "]
pub struct WICLR_R(crate::FieldReader<bool, bool>);
impl WICLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WICLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WICLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wiclr` writer - "]
pub struct WICLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WICLR_W<'a> {
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
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wiclr(&self) -> WICLR_R {
        WICLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wiclr(&mut self) -> WICLR_W {
        WICLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WICR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wicr](index.html) module"]
pub struct WICR_SPEC;
impl crate::RegisterSpec for WICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wicr::R](R) reader structure"]
impl crate::Readable for WICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wicr::W](W) writer structure"]
impl crate::Writable for WICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WICR to value 0"]
impl crate::Resettable for WICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
