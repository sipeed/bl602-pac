#[doc = "Register `ef_if_cyc_0` reader"]
pub struct R(crate::R<EF_IF_CYC_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_CYC_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EF_IF_CYC_0_SPEC>> for R {
    fn from(reader: crate::R<EF_IF_CYC_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_cyc_0` writer"]
pub struct W(crate::W<EF_IF_CYC_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_CYC_0_SPEC>;
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
impl core::convert::From<crate::W<EF_IF_CYC_0_SPEC>> for W {
    fn from(writer: crate::W<EF_IF_CYC_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_if_cyc_pd_cs_s` reader - "]
pub struct EF_IF_CYC_PD_CS_S_R(crate::FieldReader<u8, u8>);
impl EF_IF_CYC_PD_CS_S_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_CYC_PD_CS_S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CYC_PD_CS_S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_pd_cs_s` writer - "]
pub struct EF_IF_CYC_PD_CS_S_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_PD_CS_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `ef_if_cyc_cs` reader - "]
pub struct EF_IF_CYC_CS_R(crate::FieldReader<u8, u8>);
impl EF_IF_CYC_CS_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_CYC_CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CYC_CS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_cs` writer - "]
pub struct EF_IF_CYC_CS_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_CS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Field `ef_if_cyc_rd_adr` reader - "]
pub struct EF_IF_CYC_RD_ADR_R(crate::FieldReader<u8, u8>);
impl EF_IF_CYC_RD_ADR_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_CYC_RD_ADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CYC_RD_ADR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_rd_adr` writer - "]
pub struct EF_IF_CYC_RD_ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_RD_ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `ef_if_cyc_rd_dat` reader - "]
pub struct EF_IF_CYC_RD_DAT_R(crate::FieldReader<u8, u8>);
impl EF_IF_CYC_RD_DAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_CYC_RD_DAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CYC_RD_DAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_rd_dat` writer - "]
pub struct EF_IF_CYC_RD_DAT_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_RD_DAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Field `ef_if_cyc_rd_dmy` reader - "]
pub struct EF_IF_CYC_RD_DMY_R(crate::FieldReader<u8, u8>);
impl EF_IF_CYC_RD_DMY_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_CYC_RD_DMY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CYC_RD_DMY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_rd_dmy` writer - "]
pub struct EF_IF_CYC_RD_DMY_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_RD_DMY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_s(&self) -> EF_IF_CYC_PD_CS_S_R {
        EF_IF_CYC_PD_CS_S_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn ef_if_cyc_cs(&self) -> EF_IF_CYC_CS_R {
        EF_IF_CYC_CS_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_adr(&self) -> EF_IF_CYC_RD_ADR_R {
        EF_IF_CYC_RD_ADR_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dat(&self) -> EF_IF_CYC_RD_DAT_R {
        EF_IF_CYC_RD_DAT_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dmy(&self) -> EF_IF_CYC_RD_DMY_R {
        EF_IF_CYC_RD_DMY_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_cyc_pd_cs_s(&mut self) -> EF_IF_CYC_PD_CS_S_W {
        EF_IF_CYC_PD_CS_S_W { w: self }
    }
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn ef_if_cyc_cs(&mut self) -> EF_IF_CYC_CS_W {
        EF_IF_CYC_CS_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_adr(&mut self) -> EF_IF_CYC_RD_ADR_W {
        EF_IF_CYC_RD_ADR_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dat(&mut self) -> EF_IF_CYC_RD_DAT_W {
        EF_IF_CYC_RD_DAT_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn ef_if_cyc_rd_dmy(&mut self) -> EF_IF_CYC_RD_DMY_W {
        EF_IF_CYC_RD_DMY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_if_cyc_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_cyc_0](index.html) module"]
pub struct EF_IF_CYC_0_SPEC;
impl crate::RegisterSpec for EF_IF_CYC_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_cyc_0::R](R) reader structure"]
impl crate::Readable for EF_IF_CYC_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_cyc_0::W](W) writer structure"]
impl crate::Writable for EF_IF_CYC_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_if_cyc_0 to value 0"]
impl crate::Resettable for EF_IF_CYC_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
