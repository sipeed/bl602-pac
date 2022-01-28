#[doc = "Register `spi_config` reader"]
pub struct R(crate::R<SPI_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_config` writer"]
pub struct W(crate::W<SPI_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_CONFIG_SPEC>;
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
impl From<crate::W<SPI_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_spi_deg_cnt` reader - "]
pub struct CR_SPI_DEG_CNT_R(crate::FieldReader<u8, u8>);
impl CR_SPI_DEG_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_SPI_DEG_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_DEG_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_deg_cnt` writer - "]
pub struct CR_SPI_DEG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_DEG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `cr_spi_deg_en` reader - "]
pub struct CR_SPI_DEG_EN_R(crate::FieldReader<bool, bool>);
impl CR_SPI_DEG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SPI_DEG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_DEG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_deg_en` writer - "]
pub struct CR_SPI_DEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_DEG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `cr_spi_m_cont_en` reader - "]
pub struct CR_SPI_M_CONT_EN_R(crate::FieldReader<bool, bool>);
impl CR_SPI_M_CONT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SPI_M_CONT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_M_CONT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_m_cont_en` writer - "]
pub struct CR_SPI_M_CONT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_M_CONT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `cr_spi_rxd_ignr_en` reader - "]
pub struct CR_SPI_RXD_IGNR_EN_R(crate::FieldReader<bool, bool>);
impl CR_SPI_RXD_IGNR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SPI_RXD_IGNR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_RXD_IGNR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_rxd_ignr_en` writer - "]
pub struct CR_SPI_RXD_IGNR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_RXD_IGNR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `cr_spi_byte_inv` reader - "]
pub struct CR_SPI_BYTE_INV_R(crate::FieldReader<bool, bool>);
impl CR_SPI_BYTE_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SPI_BYTE_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_BYTE_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_byte_inv` writer - "]
pub struct CR_SPI_BYTE_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_BYTE_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `cr_spi_bit_inv` reader - "]
pub struct CR_SPI_BIT_INV_R(crate::FieldReader<bool, bool>);
impl CR_SPI_BIT_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SPI_BIT_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_BIT_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_bit_inv` writer - "]
pub struct CR_SPI_BIT_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_BIT_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `cr_spi_sclk_ph` reader - "]
pub struct CR_SPI_SCLK_PH_R(crate::FieldReader<bool, bool>);
impl CR_SPI_SCLK_PH_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SPI_SCLK_PH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_SCLK_PH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_sclk_ph` writer - "]
pub struct CR_SPI_SCLK_PH_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_SCLK_PH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `cr_spi_sclk_pol` reader - "]
pub struct CR_SPI_SCLK_POL_R(crate::FieldReader<bool, bool>);
impl CR_SPI_SCLK_POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SPI_SCLK_POL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_SCLK_POL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_sclk_pol` writer - "]
pub struct CR_SPI_SCLK_POL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_SCLK_POL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `cr_spi_frame_size` reader - "]
pub struct CR_SPI_FRAME_SIZE_R(crate::FieldReader<u8, u8>);
impl CR_SPI_FRAME_SIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_SPI_FRAME_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_FRAME_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_frame_size` writer - "]
pub struct CR_SPI_FRAME_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_FRAME_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `cr_spi_s_en` reader - "]
pub struct CR_SPI_S_EN_R(crate::FieldReader<bool, bool>);
impl CR_SPI_S_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SPI_S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_s_en` writer - "]
pub struct CR_SPI_S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_S_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `cr_spi_m_en` reader - "]
pub struct CR_SPI_M_EN_R(crate::FieldReader<bool, bool>);
impl CR_SPI_M_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_SPI_M_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_SPI_M_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_spi_m_en` writer - "]
pub struct CR_SPI_M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_SPI_M_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_spi_deg_cnt(&self) -> CR_SPI_DEG_CNT_R {
        CR_SPI_DEG_CNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_spi_deg_en(&self) -> CR_SPI_DEG_EN_R {
        CR_SPI_DEG_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_spi_m_cont_en(&self) -> CR_SPI_M_CONT_EN_R {
        CR_SPI_M_CONT_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_en(&self) -> CR_SPI_RXD_IGNR_EN_R {
        CR_SPI_RXD_IGNR_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_spi_byte_inv(&self) -> CR_SPI_BYTE_INV_R {
        CR_SPI_BYTE_INV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_spi_bit_inv(&self) -> CR_SPI_BIT_INV_R {
        CR_SPI_BIT_INV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_spi_sclk_ph(&self) -> CR_SPI_SCLK_PH_R {
        CR_SPI_SCLK_PH_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_spi_sclk_pol(&self) -> CR_SPI_SCLK_POL_R {
        CR_SPI_SCLK_POL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_spi_frame_size(&self) -> CR_SPI_FRAME_SIZE_R {
        CR_SPI_FRAME_SIZE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_spi_s_en(&self) -> CR_SPI_S_EN_R {
        CR_SPI_S_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_spi_m_en(&self) -> CR_SPI_M_EN_R {
        CR_SPI_M_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_spi_deg_cnt(&mut self) -> CR_SPI_DEG_CNT_W {
        CR_SPI_DEG_CNT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_spi_deg_en(&mut self) -> CR_SPI_DEG_EN_W {
        CR_SPI_DEG_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_spi_m_cont_en(&mut self) -> CR_SPI_M_CONT_EN_W {
        CR_SPI_M_CONT_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_spi_rxd_ignr_en(&mut self) -> CR_SPI_RXD_IGNR_EN_W {
        CR_SPI_RXD_IGNR_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_spi_byte_inv(&mut self) -> CR_SPI_BYTE_INV_W {
        CR_SPI_BYTE_INV_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_spi_bit_inv(&mut self) -> CR_SPI_BIT_INV_W {
        CR_SPI_BIT_INV_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_spi_sclk_ph(&mut self) -> CR_SPI_SCLK_PH_W {
        CR_SPI_SCLK_PH_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_spi_sclk_pol(&mut self) -> CR_SPI_SCLK_POL_W {
        CR_SPI_SCLK_POL_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_spi_frame_size(&mut self) -> CR_SPI_FRAME_SIZE_W {
        CR_SPI_FRAME_SIZE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_spi_s_en(&mut self) -> CR_SPI_S_EN_W {
        CR_SPI_S_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_spi_m_en(&mut self) -> CR_SPI_M_EN_W {
        CR_SPI_M_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_config](index.html) module"]
pub struct SPI_CONFIG_SPEC;
impl crate::RegisterSpec for SPI_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_config::R](R) reader structure"]
impl crate::Readable for SPI_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_config::W](W) writer structure"]
impl crate::Writable for SPI_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets spi_config to value 0"]
impl crate::Resettable for SPI_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
