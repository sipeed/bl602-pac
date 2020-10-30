#[doc = "Register `WVR` reader"]
pub struct R(crate::R<WVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WVR_SPEC>> for R {
    fn from(reader: crate::R<WVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WVR` writer"]
pub struct W(crate::W<WVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WVR_SPEC>;
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
impl core::convert::From<crate::W<WVR_SPEC>> for W {
    fn from(writer: crate::W<WVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wvr` reader - "]
pub struct WVR_R(crate::FieldReader<u16, u16>);
impl WVR_R {
    pub(crate) fn new(bits: u16) -> Self {
        WVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WVR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wvr` writer - "]
pub struct WVR_W<'a> {
    w: &'a mut W,
}
impl<'a> WVR_W<'a> {
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
    pub fn wvr(&self) -> WVR_R {
        WVR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wvr(&mut self) -> WVR_W {
        WVR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WVR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wvr](index.html) module"]
pub struct WVR_SPEC;
impl crate::RegisterSpec for WVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wvr::R](R) reader structure"]
impl crate::Readable for WVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wvr::W](W) writer structure"]
impl crate::Writable for WVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WVR to value 0"]
impl crate::Resettable for WVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
