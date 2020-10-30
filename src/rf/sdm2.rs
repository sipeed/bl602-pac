#[doc = "Register `sdm2` reader"]
pub struct R(crate::R<SDM2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDM2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SDM2_SPEC>> for R {
    fn from(reader: crate::R<SDM2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sdm2` writer"]
pub struct W(crate::W<SDM2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDM2_SPEC>;
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
impl core::convert::From<crate::W<SDM2_SPEC>> for W {
    fn from(writer: crate::W<SDM2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdmin` reader - "]
pub struct LO_SDMIN_R(crate::FieldReader<u32, u32>);
impl LO_SDMIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        LO_SDMIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDMIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdmin` writer - "]
pub struct LO_SDMIN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDMIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | ((value as u32) & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin(&self) -> LO_SDMIN_R {
        LO_SDMIN_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin(&mut self) -> LO_SDMIN_W {
        LO_SDMIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdm2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdm2](index.html) module"]
pub struct SDM2_SPEC;
impl crate::RegisterSpec for SDM2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdm2::R](R) reader structure"]
impl crate::Readable for SDM2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdm2::W](W) writer structure"]
impl crate::Writable for SDM2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sdm2 to value 0"]
impl crate::Resettable for SDM2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
