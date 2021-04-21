#[doc = "Register `sdm3` reader"]
pub struct R(crate::R<SDM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SDM3_SPEC>> for R {
    fn from(reader: crate::R<SDM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sdm3` writer"]
pub struct W(crate::W<SDM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDM3_SPEC>;
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
impl core::convert::From<crate::W<SDM3_SPEC>> for W {
    fn from(writer: crate::W<SDM3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_sdmin_hw` reader - "]
pub struct LO_SDMIN_HW_R(crate::FieldReader<u32, u32>);
impl LO_SDMIN_HW_R {
    pub(crate) fn new(bits: u32) -> Self {
        LO_SDMIN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SDMIN_HW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_sdmin_hw` writer - "]
pub struct LO_SDMIN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SDMIN_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin_hw(&self) -> LO_SDMIN_HW_R {
        LO_SDMIN_HW_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29"]
    #[inline(always)]
    pub fn lo_sdmin_hw(&mut self) -> LO_SDMIN_HW_W {
        LO_SDMIN_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sdm3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdm3](index.html) module"]
pub struct SDM3_SPEC;
impl crate::RegisterSpec for SDM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdm3::R](R) reader structure"]
impl crate::Readable for SDM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdm3::W](W) writer structure"]
impl crate::Writable for SDM3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sdm3 to value 0"]
impl crate::Resettable for SDM3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
