#[doc = "Register `sf_reserved` reader"]
pub struct R(crate::R<SF_RESERVED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_RESERVED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_RESERVED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_RESERVED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_reserved` writer"]
pub struct W(crate::W<SF_RESERVED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_RESERVED_SPEC>;
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
impl From<crate::W<SF_RESERVED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_RESERVED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_reserved` reader - "]
pub struct SF_RESERVED_R(crate::FieldReader<u32, u32>);
impl SF_RESERVED_R {
    pub(crate) fn new(bits: u32) -> Self {
        SF_RESERVED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_RESERVED_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_reserved` writer - "]
pub struct SF_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_RESERVED_W<'a> {
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
    pub fn sf_reserved(&self) -> SF_RESERVED_R {
        SF_RESERVED_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sf_reserved(&mut self) -> SF_RESERVED_W {
        SF_RESERVED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_reserved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_reserved](index.html) module"]
pub struct SF_RESERVED_SPEC;
impl crate::RegisterSpec for SF_RESERVED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_reserved::R](R) reader structure"]
impl crate::Readable for SF_RESERVED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_reserved::W](W) writer structure"]
impl crate::Writable for SF_RESERVED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_reserved to value 0"]
impl crate::Resettable for SF_RESERVED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
