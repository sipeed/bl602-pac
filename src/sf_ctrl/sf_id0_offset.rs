#[doc = "Register `sf_id0_offset` reader"]
pub struct R(crate::R<SF_ID0_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_ID0_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_ID0_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_ID0_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_id0_offset` writer"]
pub struct W(crate::W<SF_ID0_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_ID0_OFFSET_SPEC>;
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
impl From<crate::W<SF_ID0_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_ID0_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_id0_offset` reader - "]
pub struct SF_ID0_OFFSET_R(crate::FieldReader<u32, u32>);
impl SF_ID0_OFFSET_R {
    pub(crate) fn new(bits: u32) -> Self {
        SF_ID0_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_ID0_OFFSET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_id0_offset` writer - "]
pub struct SF_ID0_OFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_ID0_OFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn sf_id0_offset(&self) -> SF_ID0_OFFSET_R {
        SF_ID0_OFFSET_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn sf_id0_offset(&mut self) -> SF_ID0_OFFSET_W {
        SF_ID0_OFFSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_id0_offset.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_id0_offset](index.html) module"]
pub struct SF_ID0_OFFSET_SPEC;
impl crate::RegisterSpec for SF_ID0_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_id0_offset::R](R) reader structure"]
impl crate::Readable for SF_ID0_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_id0_offset::W](W) writer structure"]
impl crate::Writable for SF_ID0_OFFSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_id0_offset to value 0"]
impl crate::Resettable for SF_ID0_OFFSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
