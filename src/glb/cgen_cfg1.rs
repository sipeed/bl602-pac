#[doc = "Register `cgen_cfg1` reader"]
pub struct R(crate::R<CGEN_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGEN_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGEN_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGEN_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cgen_cfg1` writer"]
pub struct W(crate::W<CGEN_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGEN_CFG1_SPEC>;
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
impl From<crate::W<CGEN_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGEN_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cgen_s1a` reader - "]
pub struct CGEN_S1A_R(crate::FieldReader<u8, u8>);
impl CGEN_S1A_R {
    pub(crate) fn new(bits: u8) -> Self {
        CGEN_S1A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGEN_S1A_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cgen_s1a` writer - "]
pub struct CGEN_S1A_W<'a> {
    w: &'a mut W,
}
impl<'a> CGEN_S1A_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `cgen_s1` reader - "]
pub struct CGEN_S1_R(crate::FieldReader<u16, u16>);
impl CGEN_S1_R {
    pub(crate) fn new(bits: u16) -> Self {
        CGEN_S1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CGEN_S1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cgen_s1` writer - "]
pub struct CGEN_S1_W<'a> {
    w: &'a mut W,
}
impl<'a> CGEN_S1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cgen_s1a(&self) -> CGEN_S1A_R {
        CGEN_S1A_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cgen_s1(&self) -> CGEN_S1_R {
        CGEN_S1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cgen_s1a(&mut self) -> CGEN_S1A_W {
        CGEN_S1A_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cgen_s1(&mut self) -> CGEN_S1_W {
        CGEN_S1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cgen_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgen_cfg1](index.html) module"]
pub struct CGEN_CFG1_SPEC;
impl crate::RegisterSpec for CGEN_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgen_cfg1::R](R) reader structure"]
impl crate::Readable for CGEN_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgen_cfg1::W](W) writer structure"]
impl crate::Writable for CGEN_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cgen_cfg1 to value 0x00ff_ffff"]
impl crate::Resettable for CGEN_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_ffff
    }
}
