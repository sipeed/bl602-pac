#[doc = "Register `clk_cfg3` reader"]
pub struct R(crate::R<CLK_CFG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CFG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLK_CFG3_SPEC>> for R {
    fn from(reader: crate::R<CLK_CFG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clk_cfg3` writer"]
pub struct W(crate::W<CLK_CFG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CFG3_SPEC>;
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
impl core::convert::From<crate::W<CLK_CFG3_SPEC>> for W {
    fn from(writer: crate::W<CLK_CFG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `i2c_clk_en` reader - "]
pub struct I2C_CLK_EN_R(crate::FieldReader<bool, bool>);
impl I2C_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c_clk_en` writer - "]
pub struct I2C_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `i2c_clk_div` reader - "]
pub struct I2C_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl I2C_CLK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c_clk_div` writer - "]
pub struct I2C_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `spi_clk_en` reader - "]
pub struct SPI_CLK_EN_R(crate::FieldReader<bool, bool>);
impl SPI_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPI_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spi_clk_en` writer - "]
pub struct SPI_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLK_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `spi_clk_div` reader - "]
pub struct SPI_CLK_DIV_R(crate::FieldReader<u8, u8>);
impl SPI_CLK_DIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI_CLK_DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_CLK_DIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spi_clk_div` writer - "]
pub struct SPI_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&self) -> I2C_CLK_DIV_R {
        I2C_CLK_DIV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&self) -> SPI_CLK_EN_R {
        SPI_CLK_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&self) -> SPI_CLK_DIV_R {
        SPI_CLK_DIV_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W {
        I2C_CLK_EN_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2c_clk_div(&mut self) -> I2C_CLK_DIV_W {
        I2C_CLK_DIV_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn spi_clk_en(&mut self) -> SPI_CLK_EN_W {
        SPI_CLK_EN_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn spi_clk_div(&mut self) -> SPI_CLK_DIV_W {
        SPI_CLK_DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clk_cfg3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_cfg3](index.html) module"]
pub struct CLK_CFG3_SPEC;
impl crate::RegisterSpec for CLK_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_cfg3::R](R) reader structure"]
impl crate::Readable for CLK_CFG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_cfg3::W](W) writer structure"]
impl crate::Writable for CLK_CFG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clk_cfg3 to value 0"]
impl crate::Resettable for CLK_CFG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
