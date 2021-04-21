#[doc = "Register `cgen_cfg0` reader"]
pub struct R(crate::R<CGEN_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CGEN_CFG0_SPEC>> for R {
    fn from(reader: crate::R<CGEN_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cgen_cfg0` writer"]
pub struct W(crate::W<CGEN_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGEN_CFG0_SPEC>;
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
impl core::convert::From<crate::W<CGEN_CFG0_SPEC>> for W {
    fn from(writer: crate::W<CGEN_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cgen_m` reader - "]
pub struct CGEN_M_R(crate::FieldReader<u8, u8>);
impl CGEN_M_R {
    pub(crate) fn new(bits: u8) -> Self {
        CGEN_M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGEN_M_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cgen_m` writer - "]
pub struct CGEN_M_W<'a> {
    w: &'a mut W,
}
impl<'a> CGEN_M_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cgen_m(&self) -> CGEN_M_R {
        CGEN_M_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cgen_m(&mut self) -> CGEN_M_W {
        CGEN_M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cgen_cfg0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg0](index.html) module"]
pub struct CGEN_CFG0_SPEC;
impl crate::RegisterSpec for CGEN_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg0::R](R) reader structure"]
impl crate::Readable for CGEN_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgen_cfg0::W](W) writer structure"]
impl crate::Writable for CGEN_CFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cgen_cfg0 to value 0xff"]
impl crate::Resettable for CGEN_CFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
