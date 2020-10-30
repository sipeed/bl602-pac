#[doc = "Register `gpadc_reg_cmd` reader"]
pub struct R(crate::R<GPADC_REG_CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPADC_REG_CMD_SPEC>> for R {
    fn from(reader: crate::R<GPADC_REG_CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_cmd` writer"]
pub struct W(crate::W<GPADC_REG_CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_CMD_SPEC>;
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
impl core::convert::From<crate::W<GPADC_REG_CMD_SPEC>> for W {
    fn from(writer: crate::W<GPADC_REG_CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_sen_test_en` reader - "]
pub struct GPADC_SEN_TEST_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_SEN_TEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_SEN_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_SEN_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_sen_test_en` writer - "]
pub struct GPADC_SEN_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SEN_TEST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `gpadc_sen_sel` reader - "]
pub struct GPADC_SEN_SEL_R(crate::FieldReader<u8, u8>);
impl GPADC_SEN_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_SEN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_SEN_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_sen_sel` writer - "]
pub struct GPADC_SEN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SEN_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Field `gpadc_chip_sen_pu` reader - "]
pub struct GPADC_CHIP_SEN_PU_R(crate::FieldReader<bool, bool>);
impl GPADC_CHIP_SEN_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_CHIP_SEN_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_CHIP_SEN_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_chip_sen_pu` writer - "]
pub struct GPADC_CHIP_SEN_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CHIP_SEN_PU_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `gpadc_micboost_32db_en` reader - "]
pub struct GPADC_MICBOOST_32DB_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_MICBOOST_32DB_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_MICBOOST_32DB_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_MICBOOST_32DB_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_micboost_32db_en` writer - "]
pub struct GPADC_MICBOOST_32DB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MICBOOST_32DB_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `gpadc_mic_pga2_gain` reader - "]
pub struct GPADC_MIC_PGA2_GAIN_R(crate::FieldReader<u8, u8>);
impl GPADC_MIC_PGA2_GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_MIC_PGA2_GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_MIC_PGA2_GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_mic_pga2_gain` writer - "]
pub struct GPADC_MIC_PGA2_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MIC_PGA2_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Field `gpadc_mic1_diff` reader - "]
pub struct GPADC_MIC1_DIFF_R(crate::FieldReader<bool, bool>);
impl GPADC_MIC1_DIFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_MIC1_DIFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_MIC1_DIFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_mic1_diff` writer - "]
pub struct GPADC_MIC1_DIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MIC1_DIFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `gpadc_mic2_diff` reader - "]
pub struct GPADC_MIC2_DIFF_R(crate::FieldReader<bool, bool>);
impl GPADC_MIC2_DIFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_MIC2_DIFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_MIC2_DIFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_mic2_diff` writer - "]
pub struct GPADC_MIC2_DIFF_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MIC2_DIFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `gpadc_dwa_en` reader - "]
pub struct GPADC_DWA_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_DWA_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_DWA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_DWA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_dwa_en` writer - "]
pub struct GPADC_DWA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DWA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `gpadc_byp_micboost` reader - "]
pub struct GPADC_BYP_MICBOOST_R(crate::FieldReader<bool, bool>);
impl GPADC_BYP_MICBOOST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_BYP_MICBOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_BYP_MICBOOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_byp_micboost` writer - "]
pub struct GPADC_BYP_MICBOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_BYP_MICBOOST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `gpadc_micpga_en` reader - "]
pub struct GPADC_MICPGA_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_MICPGA_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_MICPGA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_MICPGA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_micpga_en` writer - "]
pub struct GPADC_MICPGA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MICPGA_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `gpadc_micbias_en` reader - "]
pub struct GPADC_MICBIAS_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_MICBIAS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_MICBIAS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_MICBIAS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_micbias_en` writer - "]
pub struct GPADC_MICBIAS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_MICBIAS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `gpadc_neg_gnd` reader - "]
pub struct GPADC_NEG_GND_R(crate::FieldReader<bool, bool>);
impl GPADC_NEG_GND_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_NEG_GND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_NEG_GND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_neg_gnd` writer - "]
pub struct GPADC_NEG_GND_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_NEG_GND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `gpadc_pos_sel` reader - "]
pub struct GPADC_POS_SEL_R(crate::FieldReader<u8, u8>);
impl GPADC_POS_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_POS_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_POS_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_pos_sel` writer - "]
pub struct GPADC_POS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_POS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `gpadc_neg_sel` reader - "]
pub struct GPADC_NEG_SEL_R(crate::FieldReader<u8, u8>);
impl GPADC_NEG_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_NEG_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_NEG_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_neg_sel` writer - "]
pub struct GPADC_NEG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_NEG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | (((value as u32) & 0x1f) << 3);
        self.w
    }
}
#[doc = "Field `gpadc_soft_rst` reader - "]
pub struct GPADC_SOFT_RST_R(crate::FieldReader<bool, bool>);
impl GPADC_SOFT_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_SOFT_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_SOFT_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_soft_rst` writer - "]
pub struct GPADC_SOFT_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SOFT_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `gpadc_conv_start` reader - "]
pub struct GPADC_CONV_START_R(crate::FieldReader<bool, bool>);
impl GPADC_CONV_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_CONV_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_CONV_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_conv_start` writer - "]
pub struct GPADC_CONV_START_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CONV_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `gpadc_global_en` reader - "]
pub struct GPADC_GLOBAL_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_GLOBAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_GLOBAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_GLOBAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_global_en` writer - "]
pub struct GPADC_GLOBAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_GLOBAL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpadc_sen_test_en(&self) -> GPADC_SEN_TEST_EN_R {
        GPADC_SEN_TEST_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gpadc_sen_sel(&self) -> GPADC_SEN_SEL_R {
        GPADC_SEN_SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpadc_chip_sen_pu(&self) -> GPADC_CHIP_SEN_PU_R {
        GPADC_CHIP_SEN_PU_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpadc_micboost_32db_en(&self) -> GPADC_MICBOOST_32DB_EN_R {
        GPADC_MICBOOST_32DB_EN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn gpadc_mic_pga2_gain(&self) -> GPADC_MIC_PGA2_GAIN_R {
        GPADC_MIC_PGA2_GAIN_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpadc_mic1_diff(&self) -> GPADC_MIC1_DIFF_R {
        GPADC_MIC1_DIFF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpadc_mic2_diff(&self) -> GPADC_MIC2_DIFF_R {
        GPADC_MIC2_DIFF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_dwa_en(&self) -> GPADC_DWA_EN_R {
        GPADC_DWA_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpadc_byp_micboost(&self) -> GPADC_BYP_MICBOOST_R {
        GPADC_BYP_MICBOOST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpadc_micpga_en(&self) -> GPADC_MICPGA_EN_R {
        GPADC_MICPGA_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_micbias_en(&self) -> GPADC_MICBIAS_EN_R {
        GPADC_MICBIAS_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_neg_gnd(&self) -> GPADC_NEG_GND_R {
        GPADC_NEG_GND_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpadc_pos_sel(&self) -> GPADC_POS_SEL_R {
        GPADC_POS_SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn gpadc_neg_sel(&self) -> GPADC_NEG_SEL_R {
        GPADC_NEG_SEL_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_soft_rst(&self) -> GPADC_SOFT_RST_R {
        GPADC_SOFT_RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_conv_start(&self) -> GPADC_CONV_START_R {
        GPADC_CONV_START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_global_en(&self) -> GPADC_GLOBAL_EN_R {
        GPADC_GLOBAL_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn gpadc_sen_test_en(&mut self) -> GPADC_SEN_TEST_EN_W {
        GPADC_SEN_TEST_EN_W { w: self }
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn gpadc_sen_sel(&mut self) -> GPADC_SEN_SEL_W {
        GPADC_SEN_SEL_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn gpadc_chip_sen_pu(&mut self) -> GPADC_CHIP_SEN_PU_W {
        GPADC_CHIP_SEN_PU_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn gpadc_micboost_32db_en(&mut self) -> GPADC_MICBOOST_32DB_EN_W {
        GPADC_MICBOOST_32DB_EN_W { w: self }
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn gpadc_mic_pga2_gain(&mut self) -> GPADC_MIC_PGA2_GAIN_W {
        GPADC_MIC_PGA2_GAIN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn gpadc_mic1_diff(&mut self) -> GPADC_MIC1_DIFF_W {
        GPADC_MIC1_DIFF_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn gpadc_mic2_diff(&mut self) -> GPADC_MIC2_DIFF_W {
        GPADC_MIC2_DIFF_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_dwa_en(&mut self) -> GPADC_DWA_EN_W {
        GPADC_DWA_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn gpadc_byp_micboost(&mut self) -> GPADC_BYP_MICBOOST_W {
        GPADC_BYP_MICBOOST_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gpadc_micpga_en(&mut self) -> GPADC_MICPGA_EN_W {
        GPADC_MICPGA_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_micbias_en(&mut self) -> GPADC_MICBIAS_EN_W {
        GPADC_MICBIAS_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_neg_gnd(&mut self) -> GPADC_NEG_GND_W {
        GPADC_NEG_GND_W { w: self }
    }
    #[doc = "Bits 8:12"]
    #[inline(always)]
    pub fn gpadc_pos_sel(&mut self) -> GPADC_POS_SEL_W {
        GPADC_POS_SEL_W { w: self }
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn gpadc_neg_sel(&mut self) -> GPADC_NEG_SEL_W {
        GPADC_NEG_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_soft_rst(&mut self) -> GPADC_SOFT_RST_W {
        GPADC_SOFT_RST_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_conv_start(&mut self) -> GPADC_CONV_START_W {
        GPADC_CONV_START_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_global_en(&mut self) -> GPADC_GLOBAL_EN_W {
        GPADC_GLOBAL_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_cmd.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_cmd](index.html) module"]
pub struct GPADC_REG_CMD_SPEC;
impl crate::RegisterSpec for GPADC_REG_CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_cmd::R](R) reader structure"]
impl crate::Readable for GPADC_REG_CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_cmd::W](W) writer structure"]
impl crate::Writable for GPADC_REG_CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_reg_cmd to value 0"]
impl crate::Resettable for GPADC_REG_CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
