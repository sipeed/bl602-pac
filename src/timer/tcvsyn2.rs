#[doc = "Register `TCVSYN2` reader"]
pub struct R(crate::R<TCVSYN2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCVSYN2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TCVSYN2_SPEC>> for R {
    fn from(reader: crate::R<TCVSYN2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCVSYN2` writer"]
pub struct W(crate::W<TCVSYN2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCVSYN2_SPEC>;
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
impl core::convert::From<crate::W<TCVSYN2_SPEC>> for W {
    fn from(writer: crate::W<TCVSYN2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tcvsyn2` reader - "]
pub struct TCVSYN2_R(crate::FieldReader<u32, u32>);
impl TCVSYN2_R {
    pub(crate) fn new(bits: u32) -> Self {
        TCVSYN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCVSYN2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tcvsyn2` writer - "]
pub struct TCVSYN2_W<'a> {
    w: &'a mut W,
}
impl<'a> TCVSYN2_W<'a> {
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
    pub fn tcvsyn2(&self) -> TCVSYN2_R {
        TCVSYN2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tcvsyn2(&mut self) -> TCVSYN2_W {
        TCVSYN2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCVSYN2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcvsyn2](index.html) module"]
pub struct TCVSYN2_SPEC;
impl crate::RegisterSpec for TCVSYN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcvsyn2::R](R) reader structure"]
impl crate::Readable for TCVSYN2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcvsyn2::W](W) writer structure"]
impl crate::Writable for TCVSYN2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCVSYN2 to value 0"]
impl crate::Resettable for TCVSYN2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
