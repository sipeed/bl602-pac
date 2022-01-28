#[doc = "Register `spi_rxd_ignr` reader"]
pub struct R(crate::R<SPI_RXD_IGNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_RXD_IGNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_RXD_IGNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_RXD_IGNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_rxd_ignr` writer"]
pub struct W(crate::W<SPI_RXD_IGNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_RXD_IGNR_SPEC>;
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
impl From<crate::W<SPI_RXD_IGNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_RXD_IGNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_spi_rxd_ignr_s` reader - "]
pub struct CR_SPI_RXD_IGNR_S_R(crate::FieldReader<u8, u8>);
impl CR_SPI_RXD_IGNR_S_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_SPI_RXD_IGNR_S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_RXD_IGNR_S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_rxd_ignr_s` writer - "]
pub struct CR_SPI_RXD_IGNR_S_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_RXD_IGNR_S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `cr_spi_rxd_ignr_p` reader - "]
pub struct CR_SPI_RXD_IGNR_P_R(crate::FieldReader<u8, u8>);
impl CR_SPI_RXD_IGNR_P_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_SPI_RXD_IGNR_P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_RXD_IGNR_P_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_rxd_ignr_p` writer - "]
pub struct CR_SPI_RXD_IGNR_P_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_RXD_IGNR_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_s(&self) -> CR_SPI_RXD_IGNR_S_R {
        CR_SPI_RXD_IGNR_S_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_p(&self) -> CR_SPI_RXD_IGNR_P_R {
        CR_SPI_RXD_IGNR_P_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:20"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_s(&mut self) -> CR_SPI_RXD_IGNR_S_W {
        CR_SPI_RXD_IGNR_S_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_p(&mut self) -> CR_SPI_RXD_IGNR_P_W {
        CR_SPI_RXD_IGNR_P_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_rxd_ignr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_rxd_ignr](index.html) module"]
pub struct SPI_RXD_IGNR_SPEC;
impl crate::RegisterSpec for SPI_RXD_IGNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_rxd_ignr::R](R) reader structure"]
impl crate::Readable for SPI_RXD_IGNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_rxd_ignr::W](W) writer structure"]
impl crate::Writable for SPI_RXD_IGNR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets spi_rxd_ignr to value 0"]
impl crate::Resettable for SPI_RXD_IGNR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
