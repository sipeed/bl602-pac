#[doc = "Register `seam_misc` reader"]
pub struct R(crate::R<SEAM_MISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEAM_MISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEAM_MISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEAM_MISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `seam_misc` writer"]
pub struct W(crate::W<SEAM_MISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEAM_MISC_SPEC>;
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
impl From<crate::W<SEAM_MISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEAM_MISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `em_sel` reader - "]
pub struct EM_SEL_R(crate::FieldReader<u8, u8>);
impl EM_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EM_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `em_sel` writer - "]
pub struct EM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EM_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn em_sel(&self) -> EM_SEL_R {
        EM_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn em_sel(&mut self) -> EM_SEL_W {
        EM_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "seam_misc.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seam_misc](index.html) module"]
pub struct SEAM_MISC_SPEC;
impl crate::RegisterSpec for SEAM_MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seam_misc::R](R) reader structure"]
impl crate::Readable for SEAM_MISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seam_misc::W](W) writer structure"]
impl crate::Writable for SEAM_MISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets seam_misc to value 0x03"]
impl crate::Resettable for SEAM_MISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
