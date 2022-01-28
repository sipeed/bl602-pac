#[doc = "Register `TCDR` reader"]
pub struct R(crate::R<TCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCDR` writer"]
pub struct W(crate::W<TCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCDR_SPEC>;
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
impl From<crate::W<TCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wcdr` reader - "]
pub struct WCDR_R(crate::FieldReader<u8, u8>);
impl WCDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        WCDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wcdr` writer - "]
pub struct WCDR_W<'a> {
    w: &'a mut W,
}
impl<'a> WCDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `tcdr3` reader - "]
pub struct TCDR3_R(crate::FieldReader<u8, u8>);
impl TCDR3_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCDR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCDR3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tcdr3` writer - "]
pub struct TCDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> TCDR3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `tcdr2` reader - "]
pub struct TCDR2_R(crate::FieldReader<u8, u8>);
impl TCDR2_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCDR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCDR2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tcdr2` writer - "]
pub struct TCDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCDR2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn wcdr(&self) -> WCDR_R {
        WCDR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tcdr3(&self) -> TCDR3_R {
        TCDR3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tcdr2(&self) -> TCDR2_R {
        TCDR2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn wcdr(&mut self) -> WCDR_W {
        WCDR_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn tcdr3(&mut self) -> TCDR3_W {
        TCDR3_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn tcdr2(&mut self) -> TCDR2_W {
        TCDR2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCDR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcdr](index.html) module"]
pub struct TCDR_SPEC;
impl crate::RegisterSpec for TCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcdr::R](R) reader structure"]
impl crate::Readable for TCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcdr::W](W) writer structure"]
impl crate::Writable for TCDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCDR to value 0"]
impl crate::Resettable for TCDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
