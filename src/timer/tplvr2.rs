#[doc = "Register `TPLVR2` reader"]
pub struct R(crate::R<TPLVR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPLVR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TPLVR2_SPEC>> for R {
    fn from(reader: crate::R<TPLVR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPLVR2` writer"]
pub struct W(crate::W<TPLVR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPLVR2_SPEC>;
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
impl core::convert::From<crate::W<TPLVR2_SPEC>> for W {
    fn from(writer: crate::W<TPLVR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tplvr` reader - "]
pub struct TPLVR_R(crate::FieldReader<u32, u32>);
impl TPLVR_R {
    pub(crate) fn new(bits: u32) -> Self {
        TPLVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPLVR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tplvr` writer - "]
pub struct TPLVR_W<'a> {
    w: &'a mut W,
}
impl<'a> TPLVR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tplvr(&self) -> TPLVR_R {
        TPLVR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tplvr(&mut self) -> TPLVR_W {
        TPLVR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TPLVR2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tplvr2](index.html) module"]
pub struct TPLVR2_SPEC;
impl crate::RegisterSpec for TPLVR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tplvr2::R](R) reader structure"]
impl crate::Readable for TPLVR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tplvr2::W](W) writer structure"]
impl crate::Writable for TPLVR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPLVR2 to value 0"]
impl crate::Resettable for TPLVR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
