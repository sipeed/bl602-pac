#[doc = "Register `ef_if_0_status` reader"]
pub struct R(crate::R<EF_IF_0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EF_IF_0_STATUS_SPEC>> for R {
    fn from(reader: crate::R<EF_IF_0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_0_status` writer"]
pub struct W(crate::W<EF_IF_0_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_0_STATUS_SPEC>;
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
impl core::convert::From<crate::W<EF_IF_0_STATUS_SPEC>> for W {
    fn from(writer: crate::W<EF_IF_0_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_if_0_status` reader - "]
pub struct EF_IF_0_STATUS_R(crate::FieldReader<u32, u32>);
impl EF_IF_0_STATUS_R {
    pub(crate) fn new(bits: u32) -> Self {
        EF_IF_0_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_STATUS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_0_status` writer - "]
pub struct EF_IF_0_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_STATUS_W<'a> {
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
    pub fn ef_if_0_status(&self) -> EF_IF_0_STATUS_R {
        EF_IF_0_STATUS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_if_0_status(&mut self) -> EF_IF_0_STATUS_W {
        EF_IF_0_STATUS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_if_0_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_0_status](index.html) module"]
pub struct EF_IF_0_STATUS_SPEC;
impl crate::RegisterSpec for EF_IF_0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_0_status::R](R) reader structure"]
impl crate::Readable for EF_IF_0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_0_status::W](W) writer structure"]
impl crate::Writable for EF_IF_0_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_if_0_status to value 0"]
impl crate::Resettable for EF_IF_0_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
