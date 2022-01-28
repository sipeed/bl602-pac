#[doc = "Register `irtx_swm_pw_4` reader"]
pub struct R(crate::R<IRTX_SWM_PW_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRTX_SWM_PW_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRTX_SWM_PW_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRTX_SWM_PW_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irtx_swm_pw_4` writer"]
pub struct W(crate::W<IRTX_SWM_PW_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRTX_SWM_PW_4_SPEC>;
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
impl From<crate::W<IRTX_SWM_PW_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRTX_SWM_PW_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irtx_swm_pw_4` reader - "]
pub struct CR_IRTX_SWM_PW_4_R(crate::FieldReader<u32, u32>);
impl CR_IRTX_SWM_PW_4_R {
    pub(crate) fn new(bits: u32) -> Self {
        CR_IRTX_SWM_PW_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_SWM_PW_4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_swm_pw_4` writer - "]
pub struct CR_IRTX_SWM_PW_4_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_SWM_PW_4_W<'a> {
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
    pub fn cr_irtx_swm_pw_4(&self) -> CR_IRTX_SWM_PW_4_R {
        CR_IRTX_SWM_PW_4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cr_irtx_swm_pw_4(&mut self) -> CR_IRTX_SWM_PW_4_W {
        CR_IRTX_SWM_PW_4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irtx_swm_pw_4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_swm_pw_4](index.html) module"]
pub struct IRTX_SWM_PW_4_SPEC;
impl crate::RegisterSpec for IRTX_SWM_PW_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irtx_swm_pw_4::R](R) reader structure"]
impl crate::Readable for IRTX_SWM_PW_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irtx_swm_pw_4::W](W) writer structure"]
impl crate::Writable for IRTX_SWM_PW_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irtx_swm_pw_4 to value 0"]
impl crate::Resettable for IRTX_SWM_PW_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
