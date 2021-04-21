#[doc = "Register `WSAR` writer"]
pub struct W(crate::W<WSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WSAR_SPEC>;
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
impl core::convert::From<crate::W<WSAR_SPEC>> for W {
    fn from(writer: crate::W<WSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wsar` writer - "]
pub struct WSAR_W<'a> {
    w: &'a mut W,
}
impl<'a> WSAR_W<'a> {
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
    pub fn wsar(&mut self) -> WSAR_W {
        WSAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WSAR.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wsar](index.html) module"]
pub struct WSAR_SPEC;
impl crate::RegisterSpec for WSAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [wsar::W](W) writer structure"]
impl crate::Writable for WSAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WSAR to value 0"]
impl crate::Resettable for WSAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
