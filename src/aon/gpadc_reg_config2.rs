#[doc = "Register `gpadc_reg_config2` reader"]
pub struct R(crate::R<GPADC_REG_CONFIG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_CONFIG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPADC_REG_CONFIG2_SPEC>> for R {
    fn from(reader: crate::R<GPADC_REG_CONFIG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_config2` writer"]
pub struct W(crate::W<GPADC_REG_CONFIG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_CONFIG2_SPEC>;
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
impl core::convert::From<crate::W<GPADC_REG_CONFIG2_SPEC>> for W {
    fn from(writer: crate::W<GPADC_REG_CONFIG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_tsvbe_low` reader - "]
pub struct GPADC_TSVBE_LOW_R(crate::FieldReader<bool, bool>);
impl GPADC_TSVBE_LOW_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_TSVBE_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_TSVBE_LOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_tsvbe_low` writer - "]
pub struct GPADC_TSVBE_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_TSVBE_LOW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `gpadc_dly_sel` reader - "]
pub struct GPADC_DLY_SEL_R(crate::FieldReader<u8, u8>);
impl GPADC_DLY_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_DLY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_DLY_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_dly_sel` writer - "]
pub struct GPADC_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `gpadc_pga1_gain` reader - "]
pub struct GPADC_PGA1_GAIN_R(crate::FieldReader<u8, u8>);
impl GPADC_PGA1_GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_PGA1_GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_PGA1_GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_pga1_gain` writer - "]
pub struct GPADC_PGA1_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA1_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 25)) | (((value as u32) & 0x07) << 25);
        self.w
    }
}
#[doc = "Field `gpadc_pga2_gain` reader - "]
pub struct GPADC_PGA2_GAIN_R(crate::FieldReader<u8, u8>);
impl GPADC_PGA2_GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_PGA2_GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_PGA2_GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_pga2_gain` writer - "]
pub struct GPADC_PGA2_GAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA2_GAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Field `gpadc_test_sel` reader - "]
pub struct GPADC_TEST_SEL_R(crate::FieldReader<u8, u8>);
impl GPADC_TEST_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_TEST_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_TEST_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_test_sel` writer - "]
pub struct GPADC_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 19)) | (((value as u32) & 0x07) << 19);
        self.w
    }
}
#[doc = "Field `gpadc_test_en` reader - "]
pub struct GPADC_TEST_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_TEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_test_en` writer - "]
pub struct GPADC_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_TEST_EN_W<'a> {
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
#[doc = "Field `gpadc_bias_sel` reader - "]
pub struct GPADC_BIAS_SEL_R(crate::FieldReader<bool, bool>);
impl GPADC_BIAS_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_BIAS_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_BIAS_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_bias_sel` writer - "]
pub struct GPADC_BIAS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_BIAS_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `gpadc_chop_mode` reader - "]
pub struct GPADC_CHOP_MODE_R(crate::FieldReader<u8, u8>);
impl GPADC_CHOP_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_CHOP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_CHOP_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_chop_mode` writer - "]
pub struct GPADC_CHOP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_CHOP_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | (((value as u32) & 0x03) << 15);
        self.w
    }
}
#[doc = "Field `gpadc_pga_vcmi_en` reader - "]
pub struct GPADC_PGA_VCMI_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_PGA_VCMI_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_PGA_VCMI_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_PGA_VCMI_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_pga_vcmi_en` writer - "]
pub struct GPADC_PGA_VCMI_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA_VCMI_EN_W<'a> {
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
#[doc = "Field `gpadc_pga_en` reader - "]
pub struct GPADC_PGA_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_PGA_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_PGA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_PGA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_pga_en` writer - "]
pub struct GPADC_PGA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA_EN_W<'a> {
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
#[doc = "Field `gpadc_pga_os_cal` reader - "]
pub struct GPADC_PGA_OS_CAL_R(crate::FieldReader<u8, u8>);
impl GPADC_PGA_OS_CAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_PGA_OS_CAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_PGA_OS_CAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_pga_os_cal` writer - "]
pub struct GPADC_PGA_OS_CAL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA_OS_CAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 9)) | (((value as u32) & 0x0f) << 9);
        self.w
    }
}
#[doc = "Field `gpadc_pga_vcm` reader - "]
pub struct GPADC_PGA_VCM_R(crate::FieldReader<u8, u8>);
impl GPADC_PGA_VCM_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_PGA_VCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_PGA_VCM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_pga_vcm` writer - "]
pub struct GPADC_PGA_VCM_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_PGA_VCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | (((value as u32) & 0x03) << 7);
        self.w
    }
}
#[doc = "Field `gpadc_ts_en` reader - "]
pub struct GPADC_TS_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_TS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_TS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_TS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_ts_en` writer - "]
pub struct GPADC_TS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_TS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `gpadc_tsext_sel` reader - "]
pub struct GPADC_TSEXT_SEL_R(crate::FieldReader<bool, bool>);
impl GPADC_TSEXT_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_TSEXT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_TSEXT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_tsext_sel` writer - "]
pub struct GPADC_TSEXT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_TSEXT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `gpadc_vbat_en` reader - "]
pub struct GPADC_VBAT_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_VBAT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_VBAT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_VBAT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_vbat_en` writer - "]
pub struct GPADC_VBAT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_VBAT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `gpadc_vref_sel` reader - "]
pub struct GPADC_VREF_SEL_R(crate::FieldReader<bool, bool>);
impl GPADC_VREF_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_VREF_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_VREF_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_vref_sel` writer - "]
pub struct GPADC_VREF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_VREF_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `gpadc_diff_mode` reader - "]
pub struct GPADC_DIFF_MODE_R(crate::FieldReader<bool, bool>);
impl GPADC_DIFF_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_DIFF_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_DIFF_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_diff_mode` writer - "]
pub struct GPADC_DIFF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DIFF_MODE_W<'a> {
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
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpadc_tsvbe_low(&self) -> GPADC_TSVBE_LOW_R {
        GPADC_TSVBE_LOW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gpadc_dly_sel(&self) -> GPADC_DLY_SEL_R {
        GPADC_DLY_SEL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn gpadc_pga1_gain(&self) -> GPADC_PGA1_GAIN_R {
        GPADC_PGA1_GAIN_R::new(((self.bits >> 25) & 0x07) as u8)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gpadc_pga2_gain(&self) -> GPADC_PGA2_GAIN_R {
        GPADC_PGA2_GAIN_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn gpadc_test_sel(&self) -> GPADC_TEST_SEL_R {
        GPADC_TEST_SEL_R::new(((self.bits >> 19) & 0x07) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_test_en(&self) -> GPADC_TEST_EN_R {
        GPADC_TEST_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_bias_sel(&self) -> GPADC_BIAS_SEL_R {
        GPADC_BIAS_SEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gpadc_chop_mode(&self) -> GPADC_CHOP_MODE_R {
        GPADC_CHOP_MODE_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_pga_vcmi_en(&self) -> GPADC_PGA_VCMI_EN_R {
        GPADC_PGA_VCMI_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_pga_en(&self) -> GPADC_PGA_EN_R {
        GPADC_PGA_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn gpadc_pga_os_cal(&self) -> GPADC_PGA_OS_CAL_R {
        GPADC_PGA_OS_CAL_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn gpadc_pga_vcm(&self) -> GPADC_PGA_VCM_R {
        GPADC_PGA_VCM_R::new(((self.bits >> 7) & 0x03) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_ts_en(&self) -> GPADC_TS_EN_R {
        GPADC_TS_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_tsext_sel(&self) -> GPADC_TSEXT_SEL_R {
        GPADC_TSEXT_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_vbat_en(&self) -> GPADC_VBAT_EN_R {
        GPADC_VBAT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_vref_sel(&self) -> GPADC_VREF_SEL_R {
        GPADC_VREF_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_diff_mode(&self) -> GPADC_DIFF_MODE_R {
        GPADC_DIFF_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn gpadc_tsvbe_low(&mut self) -> GPADC_TSVBE_LOW_W {
        GPADC_TSVBE_LOW_W { w: self }
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn gpadc_dly_sel(&mut self) -> GPADC_DLY_SEL_W {
        GPADC_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 25:27"]
    #[inline(always)]
    pub fn gpadc_pga1_gain(&mut self) -> GPADC_PGA1_GAIN_W {
        GPADC_PGA1_GAIN_W { w: self }
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn gpadc_pga2_gain(&mut self) -> GPADC_PGA2_GAIN_W {
        GPADC_PGA2_GAIN_W { w: self }
    }
    #[doc = "Bits 19:21"]
    #[inline(always)]
    pub fn gpadc_test_sel(&mut self) -> GPADC_TEST_SEL_W {
        GPADC_TEST_SEL_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn gpadc_test_en(&mut self) -> GPADC_TEST_EN_W {
        GPADC_TEST_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn gpadc_bias_sel(&mut self) -> GPADC_BIAS_SEL_W {
        GPADC_BIAS_SEL_W { w: self }
    }
    #[doc = "Bits 15:16"]
    #[inline(always)]
    pub fn gpadc_chop_mode(&mut self) -> GPADC_CHOP_MODE_W {
        GPADC_CHOP_MODE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_pga_vcmi_en(&mut self) -> GPADC_PGA_VCMI_EN_W {
        GPADC_PGA_VCMI_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_pga_en(&mut self) -> GPADC_PGA_EN_W {
        GPADC_PGA_EN_W { w: self }
    }
    #[doc = "Bits 9:12"]
    #[inline(always)]
    pub fn gpadc_pga_os_cal(&mut self) -> GPADC_PGA_OS_CAL_W {
        GPADC_PGA_OS_CAL_W { w: self }
    }
    #[doc = "Bits 7:8"]
    #[inline(always)]
    pub fn gpadc_pga_vcm(&mut self) -> GPADC_PGA_VCM_W {
        GPADC_PGA_VCM_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_ts_en(&mut self) -> GPADC_TS_EN_W {
        GPADC_TS_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_tsext_sel(&mut self) -> GPADC_TSEXT_SEL_W {
        GPADC_TSEXT_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_vbat_en(&mut self) -> GPADC_VBAT_EN_W {
        GPADC_VBAT_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_vref_sel(&mut self) -> GPADC_VREF_SEL_W {
        GPADC_VREF_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_diff_mode(&mut self) -> GPADC_DIFF_MODE_W {
        GPADC_DIFF_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_config2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_config2](index.html) module"]
pub struct GPADC_REG_CONFIG2_SPEC;
impl crate::RegisterSpec for GPADC_REG_CONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_config2::R](R) reader structure"]
impl crate::Readable for GPADC_REG_CONFIG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_config2::W](W) writer structure"]
impl crate::Writable for GPADC_REG_CONFIG2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_reg_config2 to value 0"]
impl crate::Resettable for GPADC_REG_CONFIG2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
