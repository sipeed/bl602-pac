#[doc = "Register `spi_prd_1` reader"]
pub struct R(crate::R<SPI_PRD_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_PRD_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_PRD_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_PRD_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_prd_1` writer"]
pub struct W(crate::W<SPI_PRD_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_PRD_1_SPEC>;
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
impl From<crate::W<SPI_PRD_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_PRD_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_spi_prd_i` reader - "]
pub struct CR_SPI_PRD_I_R(crate::FieldReader<u8, u8>);
impl CR_SPI_PRD_I_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_SPI_PRD_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_PRD_I_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_prd_i` writer - "]
pub struct CR_SPI_PRD_I_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_PRD_I_W<'a> {
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
    pub fn cr_spi_prd_i(&self) -> CR_SPI_PRD_I_R {
        CR_SPI_PRD_I_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_spi_prd_i(&mut self) -> CR_SPI_PRD_I_W {
        CR_SPI_PRD_I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_prd_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_prd_1](index.html) module"]
pub struct SPI_PRD_1_SPEC;
impl crate::RegisterSpec for SPI_PRD_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_prd_1::R](R) reader structure"]
impl crate::Readable for SPI_PRD_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_prd_1::W](W) writer structure"]
impl crate::Writable for SPI_PRD_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets spi_prd_1 to value 0x0f"]
impl crate::Resettable for SPI_PRD_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
