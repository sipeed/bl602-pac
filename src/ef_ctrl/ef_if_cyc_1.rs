#[doc = "Register `ef_if_cyc_1` reader"]
pub struct R(crate::R<EF_IF_CYC_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_CYC_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EF_IF_CYC_1_SPEC>> for R {
    fn from(reader: crate::R<EF_IF_CYC_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_cyc_1` writer"]
pub struct W(crate::W<EF_IF_CYC_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_CYC_1_SPEC>;
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
impl core::convert::From<crate::W<EF_IF_CYC_1_SPEC>> for W {
    fn from(writer: crate::W<EF_IF_CYC_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_if_cyc_pd_cs_h` reader - "]
pub struct EF_IF_CYC_PD_CS_H_R(crate::FieldReader<u8, u8>);
impl EF_IF_CYC_PD_CS_H_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_CYC_PD_CS_H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CYC_PD_CS_H_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_pd_cs_h` writer - "]
pub struct EF_IF_CYC_PD_CS_H_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_PD_CS_H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
#[doc = "Field `ef_if_cyc_ps_cs` reader - "]
pub struct EF_IF_CYC_PS_CS_R(crate::FieldReader<u8, u8>);
impl EF_IF_CYC_PS_CS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_CYC_PS_CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CYC_PS_CS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_ps_cs` writer - "]
pub struct EF_IF_CYC_PS_CS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_PS_CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | ((value as u32 & 0x3f) << 20);
        self.w
    }
}
#[doc = "Field `ef_if_cyc_wr_adr` reader - "]
pub struct EF_IF_CYC_WR_ADR_R(crate::FieldReader<u8, u8>);
impl EF_IF_CYC_WR_ADR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_CYC_WR_ADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CYC_WR_ADR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_wr_adr` writer - "]
pub struct EF_IF_CYC_WR_ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_WR_ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 14)) | ((value as u32 & 0x3f) << 14);
        self.w
    }
}
#[doc = "Field `ef_if_cyc_pp` reader - "]
pub struct EF_IF_CYC_PP_R(crate::FieldReader<u8, u8>);
impl EF_IF_CYC_PP_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_CYC_PP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CYC_PP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_pp` writer - "]
pub struct EF_IF_CYC_PP_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_PP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 6)) | ((value as u32 & 0xff) << 6);
        self.w
    }
}
#[doc = "Field `ef_if_cyc_pi` reader - "]
pub struct EF_IF_CYC_PI_R(crate::FieldReader<u8, u8>);
impl EF_IF_CYC_PI_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_CYC_PI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CYC_PI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_pi` writer - "]
pub struct EF_IF_CYC_PI_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_PI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_h(&self) -> EF_IF_CYC_PD_CS_H_R {
        EF_IF_CYC_PD_CS_H_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25"]
    #[inline(always)]
    pub fn ef_if_cyc_ps_cs(&self) -> EF_IF_CYC_PS_CS_R {
        EF_IF_CYC_PS_CS_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn ef_if_cyc_wr_adr(&self) -> EF_IF_CYC_WR_ADR_R {
        EF_IF_CYC_WR_ADR_R::new(((self.bits >> 14) & 0x3f) as u8)
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn ef_if_cyc_pp(&self) -> EF_IF_CYC_PP_R {
        EF_IF_CYC_PP_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_pi(&self) -> EF_IF_CYC_PI_R {
        EF_IF_CYC_PI_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_h(&mut self) -> EF_IF_CYC_PD_CS_H_W {
        EF_IF_CYC_PD_CS_H_W { w: self }
    }
    #[doc = "Bits 20:25"]
    #[inline(always)]
    pub fn ef_if_cyc_ps_cs(&mut self) -> EF_IF_CYC_PS_CS_W {
        EF_IF_CYC_PS_CS_W { w: self }
    }
    #[doc = "Bits 14:19"]
    #[inline(always)]
    pub fn ef_if_cyc_wr_adr(&mut self) -> EF_IF_CYC_WR_ADR_W {
        EF_IF_CYC_WR_ADR_W { w: self }
    }
    #[doc = "Bits 6:13"]
    #[inline(always)]
    pub fn ef_if_cyc_pp(&mut self) -> EF_IF_CYC_PP_W {
        EF_IF_CYC_PP_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_pi(&mut self) -> EF_IF_CYC_PI_W {
        EF_IF_CYC_PI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_if_cyc_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_cyc_1](index.html) module"]
pub struct EF_IF_CYC_1_SPEC;
impl crate::RegisterSpec for EF_IF_CYC_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_cyc_1::R](R) reader structure"]
impl crate::Readable for EF_IF_CYC_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_cyc_1::W](W) writer structure"]
impl crate::Writable for EF_IF_CYC_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_if_cyc_1 to value 0x0020_6609"]
impl crate::Resettable for EF_IF_CYC_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_6609
    }
}
