#[doc = "Register `spi_prd_0` reader"]
pub struct R(crate::R<SPI_PRD_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_PRD_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPI_PRD_0_SPEC>> for R {
    fn from(reader: crate::R<SPI_PRD_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_prd_0` writer"]
pub struct W(crate::W<SPI_PRD_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_PRD_0_SPEC>;
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
impl core::convert::From<crate::W<SPI_PRD_0_SPEC>> for W {
    fn from(writer: crate::W<SPI_PRD_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_spi_prd_d_ph_1` reader - "]
pub struct CR_SPI_PRD_D_PH_1_R(crate::FieldReader<u8, u8>);
impl CR_SPI_PRD_D_PH_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_SPI_PRD_D_PH_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_PRD_D_PH_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_prd_d_ph_1` writer - "]
pub struct CR_SPI_PRD_D_PH_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_PRD_D_PH_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `cr_spi_prd_d_ph_0` reader - "]
pub struct CR_SPI_PRD_D_PH_0_R(crate::FieldReader<u8, u8>);
impl CR_SPI_PRD_D_PH_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_SPI_PRD_D_PH_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_PRD_D_PH_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_prd_d_ph_0` writer - "]
pub struct CR_SPI_PRD_D_PH_0_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_PRD_D_PH_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `cr_spi_prd_p` reader - "]
pub struct CR_SPI_PRD_P_R(crate::FieldReader<u8, u8>);
impl CR_SPI_PRD_P_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_SPI_PRD_P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_PRD_P_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_prd_p` writer - "]
pub struct CR_SPI_PRD_P_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_PRD_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `cr_spi_prd_s` reader - "]
pub struct CR_SPI_PRD_S_R(crate::FieldReader<u8, u8>);
impl CR_SPI_PRD_S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_SPI_PRD_S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_PRD_S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_prd_s` writer - "]
pub struct CR_SPI_PRD_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_PRD_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_1(&self) -> CR_SPI_PRD_D_PH_1_R {
        CR_SPI_PRD_D_PH_1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_0(&self) -> CR_SPI_PRD_D_PH_0_R {
        CR_SPI_PRD_D_PH_0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_spi_prd_p(&self) -> CR_SPI_PRD_P_R {
        CR_SPI_PRD_P_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_spi_prd_s(&self) -> CR_SPI_PRD_S_R {
        CR_SPI_PRD_S_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_1(&mut self) -> CR_SPI_PRD_D_PH_1_W {
        CR_SPI_PRD_D_PH_1_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_0(&mut self) -> CR_SPI_PRD_D_PH_0_W {
        CR_SPI_PRD_D_PH_0_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_spi_prd_p(&mut self) -> CR_SPI_PRD_P_W {
        CR_SPI_PRD_P_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_spi_prd_s(&mut self) -> CR_SPI_PRD_S_W {
        CR_SPI_PRD_S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_prd_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_prd_0](index.html) module"]
pub struct SPI_PRD_0_SPEC;
impl crate::RegisterSpec for SPI_PRD_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_prd_0::R](R) reader structure"]
impl crate::Readable for SPI_PRD_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_prd_0::W](W) writer structure"]
impl crate::Writable for SPI_PRD_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets spi_prd_0 to value 0"]
impl crate::Resettable for SPI_PRD_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
