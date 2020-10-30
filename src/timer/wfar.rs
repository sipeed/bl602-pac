#[doc = "Register `WFAR` reader"]
pub struct R(crate::R<WFAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WFAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WFAR_SPEC>> for R {
    fn from(reader: crate::R<WFAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WFAR` writer"]
pub struct W(crate::W<WFAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WFAR_SPEC>;
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
impl core::convert::From<crate::W<WFAR_SPEC>> for W {
    fn from(writer: crate::W<WFAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wfar` reader - "]
pub struct WFAR_R(crate::FieldReader<u16, u16>);
impl WFAR_R {
    pub(crate) fn new(bits: u16) -> Self {
        WFAR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFAR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wfar` writer - "]
pub struct WFAR_W<'a> {
    w: &'a mut W,
}
impl<'a> WFAR_W<'a> {
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
    pub fn wfar(&self) -> WFAR_R {
        WFAR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wfar(&mut self) -> WFAR_W {
        WFAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WFAR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wfar](index.html) module"]
pub struct WFAR_SPEC;
impl crate::RegisterSpec for WFAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wfar::R](R) reader structure"]
impl crate::Readable for WFAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wfar::W](W) writer structure"]
impl crate::Writable for WFAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WFAR to value 0"]
impl crate::Resettable for WFAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
