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
impl From<crate::W<WFAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WFAR_SPEC>) -> Self {
        W(writer)
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
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wfar(&mut self) -> WFAR_W {
        WFAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WFAR.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wfar](index.html) module"]
pub struct WFAR_SPEC;
impl crate::RegisterSpec for WFAR_SPEC {
    type Ux = u32;
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
