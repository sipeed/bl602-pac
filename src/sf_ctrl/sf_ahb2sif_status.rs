#[doc = "Register `sf_ahb2sif_status` reader"]
pub struct R(crate::R<SF_AHB2SIF_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_AHB2SIF_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SF_AHB2SIF_STATUS_SPEC>> for R {
    fn from(reader: crate::R<SF_AHB2SIF_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ahb2sif_status` writer"]
pub struct W(crate::W<SF_AHB2SIF_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_AHB2SIF_STATUS_SPEC>;
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
impl core::convert::From<crate::W<SF_AHB2SIF_STATUS_SPEC>> for W {
    fn from(writer: crate::W<SF_AHB2SIF_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_ahb2sif_status` reader - "]
pub struct SF_AHB2SIF_STATUS_R(crate::FieldReader<u32, u32>);
impl SF_AHB2SIF_STATUS_R {
    pub(crate) fn new(bits: u32) -> Self {
        SF_AHB2SIF_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AHB2SIF_STATUS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_ahb2sif_status` writer - "]
pub struct SF_AHB2SIF_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AHB2SIF_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_ahb2sif_status(&self) -> SF_AHB2SIF_STATUS_R {
        SF_AHB2SIF_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_ahb2sif_status(&mut self) -> SF_AHB2SIF_STATUS_W {
        SF_AHB2SIF_STATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ahb2sif_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ahb2sif_status](index.html) module"]
pub struct SF_AHB2SIF_STATUS_SPEC;
impl crate::RegisterSpec for SF_AHB2SIF_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ahb2sif_status::R](R) reader structure"]
impl crate::Readable for SF_AHB2SIF_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ahb2sif_status::W](W) writer structure"]
impl crate::Writable for SF_AHB2SIF_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_ahb2sif_status to value 0"]
impl crate::Resettable for SF_AHB2SIF_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
