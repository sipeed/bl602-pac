#[doc = "Register `data_in` writer"]
pub struct W(crate::W<DATA_IN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_IN_SPEC>;
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
impl From<crate::W<DATA_IN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_IN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data_in` writer - "]
pub struct DATA_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_IN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn data_in(&mut self) -> DATA_IN_W {
        DATA_IN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Checksum data in\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_in](index.html) module"]
pub struct DATA_IN_SPEC;
impl crate::RegisterSpec for DATA_IN_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [data_in::W](W) writer structure"]
impl crate::Writable for DATA_IN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets data_in to value 0"]
impl crate::Resettable for DATA_IN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
