#[doc = "Register `WMR` reader"]
pub struct R(crate::R<WMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WMR_SPEC>> for R {
    fn from(reader: crate::R<WMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WMR` writer"]
pub struct W(crate::W<WMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WMR_SPEC>;
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
impl core::convert::From<crate::W<WMR_SPEC>> for W {
    fn from(writer: crate::W<WMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wmr` reader - "]
pub struct WMR_R(crate::FieldReader<u16, u16>);
impl WMR_R {
    pub(crate) fn new(bits: u16) -> Self {
        WMR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WMR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wmr` writer - "]
pub struct WMR_W<'a> {
    w: &'a mut W,
}
impl<'a> WMR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wmr(&self) -> WMR_R {
        WMR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn wmr(&mut self) -> WMR_W {
        WMR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WMR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmr](index.html) module"]
pub struct WMR_SPEC;
impl crate::RegisterSpec for WMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wmr::R](R) reader structure"]
impl crate::Readable for WMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wmr::W](W) writer structure"]
impl crate::Writable for WMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WMR to value 0xffff"]
impl crate::Resettable for WMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
