#[doc = "Register `spi_sto_value` reader"]
pub struct R(crate::R<SPI_STO_VALUE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_STO_VALUE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SPI_STO_VALUE_SPEC>> for R {
    fn from(reader: crate::R<SPI_STO_VALUE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_sto_value` writer"]
pub struct W(crate::W<SPI_STO_VALUE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_STO_VALUE_SPEC>;
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
impl core::convert::From<crate::W<SPI_STO_VALUE_SPEC>> for W {
    fn from(writer: crate::W<SPI_STO_VALUE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_spi_sto_value` reader - "]
pub struct CR_SPI_STO_VALUE_R(crate::FieldReader<u16, u16>);
impl CR_SPI_STO_VALUE_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_SPI_STO_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_STO_VALUE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_sto_value` writer - "]
pub struct CR_SPI_STO_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_STO_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_spi_sto_value(&self) -> CR_SPI_STO_VALUE_R {
        CR_SPI_STO_VALUE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn cr_spi_sto_value(&mut self) -> CR_SPI_STO_VALUE_W {
        CR_SPI_STO_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_sto_value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_sto_value](index.html) module"]
pub struct SPI_STO_VALUE_SPEC;
impl crate::RegisterSpec for SPI_STO_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_sto_value::R](R) reader structure"]
impl crate::Readable for SPI_STO_VALUE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_sto_value::W](W) writer structure"]
impl crate::Writable for SPI_STO_VALUE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets spi_sto_value to value 0x0fff"]
impl crate::Resettable for SPI_STO_VALUE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff
    }
}
