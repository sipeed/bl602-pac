#[doc = "Register `ten_dc` reader"]
pub struct R(crate::R<TEN_DC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEN_DC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TEN_DC_SPEC>> for R {
    fn from(reader: crate::R<TEN_DC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ten_dc` writer"]
pub struct W(crate::W<TEN_DC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEN_DC_SPEC>;
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
impl core::convert::From<crate::W<TEN_DC_SPEC>> for W {
    fn from(writer: crate::W<TEN_DC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ten_lodist` reader - "]
pub struct TEN_LODIST_R(crate::FieldReader<bool, bool>);
impl TEN_LODIST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_LODIST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_LODIST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_lodist` writer - "]
pub struct TEN_LODIST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_LODIST_W<'a> {
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
#[doc = "Field `ten_lf` reader - "]
pub struct TEN_LF_R(crate::FieldReader<bool, bool>);
impl TEN_LF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_LF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_LF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_lf` writer - "]
pub struct TEN_LF_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_LF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `ten_pfdcp` reader - "]
pub struct TEN_PFDCP_R(crate::FieldReader<bool, bool>);
impl TEN_PFDCP_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_PFDCP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_PFDCP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_pfdcp` writer - "]
pub struct TEN_PFDCP_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_PFDCP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `ten_vco` reader - "]
pub struct TEN_VCO_R(crate::FieldReader<bool, bool>);
impl TEN_VCO_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_VCO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_VCO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_vco` writer - "]
pub struct TEN_VCO_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_VCO_W<'a> {
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
#[doc = "Field `ten_dac_q` reader - "]
pub struct TEN_DAC_Q_R(crate::FieldReader<bool, bool>);
impl TEN_DAC_Q_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_DAC_Q_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_DAC_Q_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_dac_q` writer - "]
pub struct TEN_DAC_Q_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_DAC_Q_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `ten_dac_i` reader - "]
pub struct TEN_DAC_I_R(crate::FieldReader<bool, bool>);
impl TEN_DAC_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_DAC_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_DAC_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_dac_i` writer - "]
pub struct TEN_DAC_I_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_DAC_I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `ten_adc` reader - "]
pub struct TEN_ADC_R(crate::FieldReader<bool, bool>);
impl TEN_ADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_adc` writer - "]
pub struct TEN_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_ADC_W<'a> {
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
#[doc = "Field `ten_tbb` reader - "]
pub struct TEN_TBB_R(crate::FieldReader<bool, bool>);
impl TEN_TBB_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_TBB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_TBB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_tbb` writer - "]
pub struct TEN_TBB_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_TBB_W<'a> {
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
#[doc = "Field `ten_atest` reader - "]
pub struct TEN_ATEST_R(crate::FieldReader<bool, bool>);
impl TEN_ATEST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_ATEST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_ATEST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_atest` writer - "]
pub struct TEN_ATEST_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_ATEST_W<'a> {
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
#[doc = "Field `ten_bq` reader - "]
pub struct TEN_BQ_R(crate::FieldReader<bool, bool>);
impl TEN_BQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_BQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_BQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_bq` writer - "]
pub struct TEN_BQ_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_BQ_W<'a> {
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
#[doc = "Field `ten_tia` reader - "]
pub struct TEN_TIA_R(crate::FieldReader<bool, bool>);
impl TEN_TIA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_TIA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_TIA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_tia` writer - "]
pub struct TEN_TIA_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_TIA_W<'a> {
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
#[doc = "Field `ten_tmx` reader - "]
pub struct TEN_TMX_R(crate::FieldReader<bool, bool>);
impl TEN_TMX_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_TMX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_TMX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_tmx` writer - "]
pub struct TEN_TMX_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_TMX_W<'a> {
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
#[doc = "Field `ten_pa` reader - "]
pub struct TEN_PA_R(crate::FieldReader<bool, bool>);
impl TEN_PA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_PA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_PA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_pa` writer - "]
pub struct TEN_PA_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_PA_W<'a> {
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
#[doc = "Field `ten_rrf_1` reader - "]
pub struct TEN_RRF_1_R(crate::FieldReader<bool, bool>);
impl TEN_RRF_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_RRF_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_RRF_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_rrf_1` writer - "]
pub struct TEN_RRF_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_RRF_1_W<'a> {
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
#[doc = "Field `ten_rrf_0` reader - "]
pub struct TEN_RRF_0_R(crate::FieldReader<bool, bool>);
impl TEN_RRF_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_RRF_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_RRF_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_rrf_0` writer - "]
pub struct TEN_RRF_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_RRF_0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `ten_clkpll_sfreg` reader - "]
pub struct TEN_CLKPLL_SFREG_R(crate::FieldReader<bool, bool>);
impl TEN_CLKPLL_SFREG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_CLKPLL_SFREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_CLKPLL_SFREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_clkpll_sfreg` writer - "]
pub struct TEN_CLKPLL_SFREG_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_CLKPLL_SFREG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ten_clkpll` reader - "]
pub struct TEN_CLKPLL_R(crate::FieldReader<bool, bool>);
impl TEN_CLKPLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEN_CLKPLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_CLKPLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_clkpll` writer - "]
pub struct TEN_CLKPLL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_CLKPLL_W<'a> {
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
#[doc = "Field `dc_tp_clkpll_en` reader - "]
pub struct DC_TP_CLKPLL_EN_R(crate::FieldReader<bool, bool>);
impl DC_TP_CLKPLL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC_TP_CLKPLL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_TP_CLKPLL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dc_tp_clkpll_en` writer - "]
pub struct DC_TP_CLKPLL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_TP_CLKPLL_EN_W<'a> {
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
#[doc = "Field `dc_tp_en` reader - "]
pub struct DC_TP_EN_R(crate::FieldReader<bool, bool>);
impl DC_TP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DC_TP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DC_TP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dc_tp_en` writer - "]
pub struct DC_TP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DC_TP_EN_W<'a> {
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
#[doc = "Field `tmux` reader - "]
pub struct TMUX_R(crate::FieldReader<u8, u8>);
impl TMUX_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmux` writer - "]
pub struct TMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> TMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_lodist(&self) -> TEN_LODIST_R {
        TEN_LODIST_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_lf(&self) -> TEN_LF_R {
        TEN_LF_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ten_pfdcp(&self) -> TEN_PFDCP_R {
        TEN_PFDCP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ten_vco(&self) -> TEN_VCO_R {
        TEN_VCO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ten_dac_q(&self) -> TEN_DAC_Q_R {
        TEN_DAC_Q_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_dac_i(&self) -> TEN_DAC_I_R {
        TEN_DAC_I_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_adc(&self) -> TEN_ADC_R {
        TEN_ADC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_tbb(&self) -> TEN_TBB_R {
        TEN_TBB_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_atest(&self) -> TEN_ATEST_R {
        TEN_ATEST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_bq(&self) -> TEN_BQ_R {
        TEN_BQ_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_tia(&self) -> TEN_TIA_R {
        TEN_TIA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ten_tmx(&self) -> TEN_TMX_R {
        TEN_TMX_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ten_pa(&self) -> TEN_PA_R {
        TEN_PA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ten_rrf_1(&self) -> TEN_RRF_1_R {
        TEN_RRF_1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_rrf_0(&self) -> TEN_RRF_0_R {
        TEN_RRF_0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_clkpll_sfreg(&self) -> TEN_CLKPLL_SFREG_R {
        TEN_CLKPLL_SFREG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_clkpll(&self) -> TEN_CLKPLL_R {
        TEN_CLKPLL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dc_tp_clkpll_en(&self) -> DC_TP_CLKPLL_EN_R {
        DC_TP_CLKPLL_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dc_tp_en(&self) -> DC_TP_EN_R {
        DC_TP_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux(&self) -> TMUX_R {
        TMUX_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn ten_lodist(&mut self) -> TEN_LODIST_W {
        TEN_LODIST_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn ten_lf(&mut self) -> TEN_LF_W {
        TEN_LF_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn ten_pfdcp(&mut self) -> TEN_PFDCP_W {
        TEN_PFDCP_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn ten_vco(&mut self) -> TEN_VCO_W {
        TEN_VCO_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ten_dac_q(&mut self) -> TEN_DAC_Q_W {
        TEN_DAC_Q_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ten_dac_i(&mut self) -> TEN_DAC_I_W {
        TEN_DAC_I_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ten_adc(&mut self) -> TEN_ADC_W {
        TEN_ADC_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ten_tbb(&mut self) -> TEN_TBB_W {
        TEN_TBB_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten_atest(&mut self) -> TEN_ATEST_W {
        TEN_ATEST_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ten_bq(&mut self) -> TEN_BQ_W {
        TEN_BQ_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ten_tia(&mut self) -> TEN_TIA_W {
        TEN_TIA_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ten_tmx(&mut self) -> TEN_TMX_W {
        TEN_TMX_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ten_pa(&mut self) -> TEN_PA_W {
        TEN_PA_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ten_rrf_1(&mut self) -> TEN_RRF_1_W {
        TEN_RRF_1_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ten_rrf_0(&mut self) -> TEN_RRF_0_W {
        TEN_RRF_0_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ten_clkpll_sfreg(&mut self) -> TEN_CLKPLL_SFREG_W {
        TEN_CLKPLL_SFREG_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ten_clkpll(&mut self) -> TEN_CLKPLL_W {
        TEN_CLKPLL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dc_tp_clkpll_en(&mut self) -> DC_TP_CLKPLL_EN_W {
        DC_TP_CLKPLL_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dc_tp_en(&mut self) -> DC_TP_EN_W {
        DC_TP_EN_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tmux(&mut self) -> TMUX_W {
        TMUX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dc test register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ten_dc](index.html) module"]
pub struct TEN_DC_SPEC;
impl crate::RegisterSpec for TEN_DC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ten_dc::R](R) reader structure"]
impl crate::Readable for TEN_DC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ten_dc::W](W) writer structure"]
impl crate::Writable for TEN_DC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ten_dc to value 0"]
impl crate::Resettable for TEN_DC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
