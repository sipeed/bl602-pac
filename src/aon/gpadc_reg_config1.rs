#[doc = "Register `gpadc_reg_config1` reader"]
pub struct R(crate::R<GPADC_REG_CONFIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_CONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPADC_REG_CONFIG1_SPEC>> for R {
    fn from(reader: crate::R<GPADC_REG_CONFIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_config1` writer"]
pub struct W(crate::W<GPADC_REG_CONFIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_CONFIG1_SPEC>;
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
impl core::convert::From<crate::W<GPADC_REG_CONFIG1_SPEC>> for W {
    fn from(writer: crate::W<GPADC_REG_CONFIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_v18_sel` reader - "]
pub struct GPADC_V18_SEL_R(crate::FieldReader<u8, u8>);
impl GPADC_V18_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_V18_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_V18_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_v18_sel` writer - "]
pub struct GPADC_V18_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_V18_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
#[doc = "Field `gpadc_v11_sel` reader - "]
pub struct GPADC_V11_SEL_R(crate::FieldReader<u8, u8>);
impl GPADC_V11_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_V11_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_V11_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_v11_sel` writer - "]
pub struct GPADC_V11_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_V11_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Field `gpadc_dither_en` reader - "]
pub struct GPADC_DITHER_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_DITHER_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_DITHER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_DITHER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_dither_en` writer - "]
pub struct GPADC_DITHER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DITHER_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `gpadc_scan_en` reader - "]
pub struct GPADC_SCAN_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_SCAN_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_SCAN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_SCAN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_scan_en` writer - "]
pub struct GPADC_SCAN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `gpadc_scan_length` reader - "]
pub struct GPADC_SCAN_LENGTH_R(crate::FieldReader<u8, u8>);
impl GPADC_SCAN_LENGTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_SCAN_LENGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_SCAN_LENGTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_scan_length` writer - "]
pub struct GPADC_SCAN_LENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_LENGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 21)) | ((value as u32 & 0x0f) << 21);
        self.w
    }
}
#[doc = "Field `gpadc_clk_div_ratio` reader - "]
pub struct GPADC_CLK_DIV_RATIO_R(crate::FieldReader<u8, u8>);
impl GPADC_CLK_DIV_RATIO_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_CLK_DIV_RATIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_CLK_DIV_RATIO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_clk_div_ratio` writer - "]
pub struct GPADC_CLK_DIV_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CLK_DIV_RATIO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `gpadc_clk_ana_inv` reader - "]
pub struct GPADC_CLK_ANA_INV_R(crate::FieldReader<bool, bool>);
impl GPADC_CLK_ANA_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_CLK_ANA_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_CLK_ANA_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_clk_ana_inv` writer - "]
pub struct GPADC_CLK_ANA_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CLK_ANA_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `gpadc_res_sel` reader - "]
pub struct GPADC_RES_SEL_R(crate::FieldReader<u8, u8>);
impl GPADC_RES_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_RES_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_RES_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_res_sel` writer - "]
pub struct GPADC_RES_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RES_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `gpadc_cont_conv_en` reader - "]
pub struct GPADC_CONT_CONV_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_CONT_CONV_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_CONT_CONV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_CONT_CONV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_cont_conv_en` writer - "]
pub struct GPADC_CONT_CONV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CONT_CONV_EN_W<'a> {
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
#[doc = "Field `gpadc_cal_os_en` reader - "]
pub struct GPADC_CAL_OS_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_CAL_OS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_CAL_OS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_CAL_OS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_cal_os_en` writer - "]
pub struct GPADC_CAL_OS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CAL_OS_EN_W<'a> {
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
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn gpadc_v18_sel(&self) -> GPADC_V18_SEL_R {
        GPADC_V18_SEL_R::new(((self.bits >> 29) & 0x03) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn gpadc_v11_sel(&self) -> GPADC_V11_SEL_R {
        GPADC_V11_SEL_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpadc_dither_en(&self) -> GPADC_DITHER_EN_R {
        GPADC_DITHER_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpadc_scan_en(&self) -> GPADC_SCAN_EN_R {
        GPADC_SCAN_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn gpadc_scan_length(&self) -> GPADC_SCAN_LENGTH_R {
        GPADC_SCAN_LENGTH_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn gpadc_clk_div_ratio(&self) -> GPADC_CLK_DIV_RATIO_R {
        GPADC_CLK_DIV_RATIO_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_clk_ana_inv(&self) -> GPADC_CLK_ANA_INV_R {
        GPADC_CLK_ANA_INV_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gpadc_res_sel(&self) -> GPADC_RES_SEL_R {
        GPADC_RES_SEL_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_cont_conv_en(&self) -> GPADC_CONT_CONV_EN_R {
        GPADC_CONT_CONV_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_cal_os_en(&self) -> GPADC_CAL_OS_EN_R {
        GPADC_CAL_OS_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn gpadc_v18_sel(&mut self) -> GPADC_V18_SEL_W {
        GPADC_V18_SEL_W { w: self }
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn gpadc_v11_sel(&mut self) -> GPADC_V11_SEL_W {
        GPADC_V11_SEL_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn gpadc_dither_en(&mut self) -> GPADC_DITHER_EN_W {
        GPADC_DITHER_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn gpadc_scan_en(&mut self) -> GPADC_SCAN_EN_W {
        GPADC_SCAN_EN_W { w: self }
    }
    #[doc = "Bits 21:24"]
    #[inline(always)]
    pub fn gpadc_scan_length(&mut self) -> GPADC_SCAN_LENGTH_W {
        GPADC_SCAN_LENGTH_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn gpadc_clk_div_ratio(&mut self) -> GPADC_CLK_DIV_RATIO_W {
        GPADC_CLK_DIV_RATIO_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_clk_ana_inv(&mut self) -> GPADC_CLK_ANA_INV_W {
        GPADC_CLK_ANA_INV_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn gpadc_res_sel(&mut self) -> GPADC_RES_SEL_W {
        GPADC_RES_SEL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_cont_conv_en(&mut self) -> GPADC_CONT_CONV_EN_W {
        GPADC_CONT_CONV_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_cal_os_en(&mut self) -> GPADC_CAL_OS_EN_W {
        GPADC_CAL_OS_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_config1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_config1](index.html) module"]
pub struct GPADC_REG_CONFIG1_SPEC;
impl crate::RegisterSpec for GPADC_REG_CONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_config1::R](R) reader structure"]
impl crate::Readable for GPADC_REG_CONFIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_config1::W](W) writer structure"]
impl crate::Writable for GPADC_REG_CONFIG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_reg_config1 to value 0x000c_0002"]
impl crate::Resettable for GPADC_REG_CONFIG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000c_0002
    }
}
