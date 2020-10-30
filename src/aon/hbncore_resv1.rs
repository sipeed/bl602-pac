#[doc = "Register `hbncore_resv1` reader"]
pub struct R(crate::R<HBNCORE_RESV1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBNCORE_RESV1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HBNCORE_RESV1_SPEC>> for R {
    fn from(reader: crate::R<HBNCORE_RESV1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `hbncore_resv1` writer"]
pub struct W(crate::W<HBNCORE_RESV1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBNCORE_RESV1_SPEC>;
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
impl core::convert::From<crate::W<HBNCORE_RESV1_SPEC>> for W {
    fn from(writer: crate::W<HBNCORE_RESV1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hbncore_resv1_data` reader - "]
pub struct HBNCORE_RESV1_DATA_R(crate::FieldReader<u32, u32>);
impl HBNCORE_RESV1_DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        HBNCORE_RESV1_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBNCORE_RESV1_DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbncore_resv1_data` writer - "]
pub struct HBNCORE_RESV1_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> HBNCORE_RESV1_DATA_W<'a> {
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
    pub fn hbncore_resv1_data(&self) -> HBNCORE_RESV1_DATA_R {
        HBNCORE_RESV1_DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hbncore_resv1_data(&mut self) -> HBNCORE_RESV1_DATA_W {
        HBNCORE_RESV1_DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "hbncore_resv1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbncore_resv1](index.html) module"]
pub struct HBNCORE_RESV1_SPEC;
impl crate::RegisterSpec for HBNCORE_RESV1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbncore_resv1::R](R) reader structure"]
impl crate::Readable for HBNCORE_RESV1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbncore_resv1::W](W) writer structure"]
impl crate::Writable for HBNCORE_RESV1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets hbncore_resv1 to value 0"]
impl crate::Resettable for HBNCORE_RESV1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
