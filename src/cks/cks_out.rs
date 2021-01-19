#[doc = "Register `cks_out` reader"]
pub struct R(crate::R<CKS_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKS_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CKS_OUT_SPEC>> for R {
    fn from(reader: crate::R<CKS_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cks_out` writer"]
pub struct W(crate::W<CKS_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKS_OUT_SPEC>;
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
impl core::convert::From<crate::W<CKS_OUT_SPEC>> for W {
    fn from(writer: crate::W<CKS_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cks_out` reader - "]
pub struct CKS_OUT_R(crate::FieldReader<u16, u16>);
impl CKS_OUT_R {
    pub(crate) fn new(bits: u16) -> Self {
        CKS_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKS_OUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cks_out` writer - "]
pub struct CKS_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> CKS_OUT_W<'a> {
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
    pub fn cks_out(&self) -> CKS_OUT_R {
        CKS_OUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cks_out(&mut self) -> CKS_OUT_W {
        CKS_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Checksum value out\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cks_out](index.html) module"]
pub struct CKS_OUT_SPEC;
impl crate::RegisterSpec for CKS_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cks_out::R](R) reader structure"]
impl crate::Readable for CKS_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cks_out::W](W) writer structure"]
impl crate::Writable for CKS_OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cks_out to value 0"]
impl crate::Resettable for CKS_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
