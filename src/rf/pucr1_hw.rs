#[doc = "Register `pucr1_hw` reader"]
pub struct R(crate::R<PUCR1_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCR1_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PUCR1_HW_SPEC>> for R {
    fn from(reader: crate::R<PUCR1_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pucr1_hw` writer"]
pub struct W(crate::W<PUCR1_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCR1_HW_SPEC>;
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
impl core::convert::From<crate::W<PUCR1_HW_SPEC>> for W {
    fn from(writer: crate::W<PUCR1_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pu_tosdac_hw` reader - "]
pub struct PU_TOSDAC_HW_R(crate::FieldReader<bool, bool>);
impl PU_TOSDAC_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_TOSDAC_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_TOSDAC_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_tosdac_hw` writer - "]
pub struct PU_TOSDAC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TOSDAC_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `pu_rosdac_hw` reader - "]
pub struct PU_ROSDAC_HW_R(crate::FieldReader<bool, bool>);
impl PU_ROSDAC_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_ROSDAC_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_ROSDAC_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rosdac_hw` writer - "]
pub struct PU_ROSDAC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ROSDAC_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `pu_pkdet_hw` reader - "]
pub struct PU_PKDET_HW_R(crate::FieldReader<bool, bool>);
impl PU_PKDET_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_PKDET_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_PKDET_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_pkdet_hw` writer - "]
pub struct PU_PKDET_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PKDET_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `trsw_en_hw` reader - "]
pub struct TRSW_EN_HW_R(crate::FieldReader<bool, bool>);
impl TRSW_EN_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRSW_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRSW_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `trsw_en_hw` writer - "]
pub struct TRSW_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> TRSW_EN_HW_W<'a> {
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
#[doc = "Field `pu_txbuf_hw` reader - "]
pub struct PU_TXBUF_HW_R(crate::FieldReader<bool, bool>);
impl PU_TXBUF_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_TXBUF_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_TXBUF_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_txbuf_hw` writer - "]
pub struct PU_TXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TXBUF_HW_W<'a> {
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
#[doc = "Field `pu_rxbuf_hw` reader - "]
pub struct PU_RXBUF_HW_R(crate::FieldReader<bool, bool>);
impl PU_RXBUF_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_RXBUF_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RXBUF_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rxbuf_hw` writer - "]
pub struct PU_RXBUF_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RXBUF_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `pu_osmx_hw` reader - "]
pub struct PU_OSMX_HW_R(crate::FieldReader<bool, bool>);
impl PU_OSMX_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_OSMX_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_OSMX_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_osmx_hw` writer - "]
pub struct PU_OSMX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_OSMX_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `pu_pfd_hw` reader - "]
pub struct PU_PFD_HW_R(crate::FieldReader<bool, bool>);
impl PU_PFD_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_PFD_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_PFD_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_pfd_hw` writer - "]
pub struct PU_PFD_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PFD_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `pu_fbdv_hw` reader - "]
pub struct PU_FBDV_HW_R(crate::FieldReader<bool, bool>);
impl PU_FBDV_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_FBDV_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_FBDV_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_fbdv_hw` writer - "]
pub struct PU_FBDV_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_FBDV_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `pu_vco_hw` reader - "]
pub struct PU_VCO_HW_R(crate::FieldReader<bool, bool>);
impl PU_VCO_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_VCO_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_VCO_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_vco_hw` writer - "]
pub struct PU_VCO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_VCO_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `pu_dac_hw` reader - "]
pub struct PU_DAC_HW_R(crate::FieldReader<bool, bool>);
impl PU_DAC_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_DAC_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_DAC_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_dac_hw` writer - "]
pub struct PU_DAC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_DAC_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `pu_tbb_hw` reader - "]
pub struct PU_TBB_HW_R(crate::FieldReader<bool, bool>);
impl PU_TBB_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_TBB_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_TBB_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_tbb_hw` writer - "]
pub struct PU_TBB_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TBB_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `pu_tmx_hw` reader - "]
pub struct PU_TMX_HW_R(crate::FieldReader<bool, bool>);
impl PU_TMX_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_TMX_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_TMX_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_tmx_hw` writer - "]
pub struct PU_TMX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_TMX_HW_W<'a> {
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
#[doc = "Field `pu_pa_hw` reader - "]
pub struct PU_PA_HW_R(crate::FieldReader<bool, bool>);
impl PU_PA_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_PA_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_PA_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_pa_hw` writer - "]
pub struct PU_PA_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_PA_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `pu_adc_hw` reader - "]
pub struct PU_ADC_HW_R(crate::FieldReader<bool, bool>);
impl PU_ADC_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_ADC_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_ADC_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_adc_hw` writer - "]
pub struct PU_ADC_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ADC_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `adc_clk_en_hw` reader - "]
pub struct ADC_CLK_EN_HW_R(crate::FieldReader<bool, bool>);
impl ADC_CLK_EN_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC_CLK_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_CLK_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_clk_en_hw` writer - "]
pub struct ADC_CLK_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_EN_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `pu_adda_ldo_hw` reader - "]
pub struct PU_ADDA_LDO_HW_R(crate::FieldReader<bool, bool>);
impl PU_ADDA_LDO_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_ADDA_LDO_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_ADDA_LDO_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_adda_ldo_hw` writer - "]
pub struct PU_ADDA_LDO_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_ADDA_LDO_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `pu_rbb_hw` reader - "]
pub struct PU_RBB_HW_R(crate::FieldReader<bool, bool>);
impl PU_RBB_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_RBB_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RBB_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rbb_hw` writer - "]
pub struct PU_RBB_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RBB_HW_W<'a> {
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
#[doc = "Field `pu_rmx_hw` reader - "]
pub struct PU_RMX_HW_R(crate::FieldReader<bool, bool>);
impl PU_RMX_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_RMX_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RMX_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rmx_hw` writer - "]
pub struct PU_RMX_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RMX_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `pu_rmxgm_hw` reader - "]
pub struct PU_RMXGM_HW_R(crate::FieldReader<bool, bool>);
impl PU_RMXGM_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_RMXGM_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_RMXGM_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_rmxgm_hw` writer - "]
pub struct PU_RMXGM_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_RMXGM_HW_W<'a> {
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
#[doc = "Field `pu_lna_hw` reader - "]
pub struct PU_LNA_HW_R(crate::FieldReader<bool, bool>);
impl PU_LNA_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_LNA_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_LNA_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_lna_hw` writer - "]
pub struct PU_LNA_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_LNA_HW_W<'a> {
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
#[doc = "Field `pu_sfreg_hw` reader - "]
pub struct PU_SFREG_HW_R(crate::FieldReader<bool, bool>);
impl PU_SFREG_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_SFREG_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_SFREG_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_sfreg_hw` writer - "]
pub struct PU_SFREG_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_SFREG_HW_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_tosdac_hw(&self) -> PU_TOSDAC_HW_R {
        PU_TOSDAC_HW_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_rosdac_hw(&self) -> PU_ROSDAC_HW_R {
        PU_ROSDAC_HW_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pu_pkdet_hw(&self) -> PU_PKDET_HW_R {
        PU_PKDET_HW_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn trsw_en_hw(&self) -> TRSW_EN_HW_R {
        TRSW_EN_HW_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pu_txbuf_hw(&self) -> PU_TXBUF_HW_R {
        PU_TXBUF_HW_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_rxbuf_hw(&self) -> PU_RXBUF_HW_R {
        PU_RXBUF_HW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_osmx_hw(&self) -> PU_OSMX_HW_R {
        PU_OSMX_HW_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pfd_hw(&self) -> PU_PFD_HW_R {
        PU_PFD_HW_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_fbdv_hw(&self) -> PU_FBDV_HW_R {
        PU_FBDV_HW_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_vco_hw(&self) -> PU_VCO_HW_R {
        PU_VCO_HW_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_dac_hw(&self) -> PU_DAC_HW_R {
        PU_DAC_HW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_tbb_hw(&self) -> PU_TBB_HW_R {
        PU_TBB_HW_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_tmx_hw(&self) -> PU_TMX_HW_R {
        PU_TMX_HW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_pa_hw(&self) -> PU_PA_HW_R {
        PU_PA_HW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_adc_hw(&self) -> PU_ADC_HW_R {
        PU_ADC_HW_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en_hw(&self) -> ADC_CLK_EN_HW_R {
        ADC_CLK_EN_HW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_adda_ldo_hw(&self) -> PU_ADDA_LDO_HW_R {
        PU_ADDA_LDO_HW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_rbb_hw(&self) -> PU_RBB_HW_R {
        PU_RBB_HW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_rmx_hw(&self) -> PU_RMX_HW_R {
        PU_RMX_HW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_rmxgm_hw(&self) -> PU_RMXGM_HW_R {
        PU_RMXGM_HW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_lna_hw(&self) -> PU_LNA_HW_R {
        PU_LNA_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_sfreg_hw(&self) -> PU_SFREG_HW_R {
        PU_SFREG_HW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pu_tosdac_hw(&mut self) -> PU_TOSDAC_HW_W {
        PU_TOSDAC_HW_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn pu_rosdac_hw(&mut self) -> PU_ROSDAC_HW_W {
        PU_ROSDAC_HW_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn pu_pkdet_hw(&mut self) -> PU_PKDET_HW_W {
        PU_PKDET_HW_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn trsw_en_hw(&mut self) -> TRSW_EN_HW_W {
        TRSW_EN_HW_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn pu_txbuf_hw(&mut self) -> PU_TXBUF_HW_W {
        PU_TXBUF_HW_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn pu_rxbuf_hw(&mut self) -> PU_RXBUF_HW_W {
        PU_RXBUF_HW_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pu_osmx_hw(&mut self) -> PU_OSMX_HW_W {
        PU_OSMX_HW_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pu_pfd_hw(&mut self) -> PU_PFD_HW_W {
        PU_PFD_HW_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pu_fbdv_hw(&mut self) -> PU_FBDV_HW_W {
        PU_FBDV_HW_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pu_vco_hw(&mut self) -> PU_VCO_HW_W {
        PU_VCO_HW_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pu_dac_hw(&mut self) -> PU_DAC_HW_W {
        PU_DAC_HW_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn pu_tbb_hw(&mut self) -> PU_TBB_HW_W {
        PU_TBB_HW_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pu_tmx_hw(&mut self) -> PU_TMX_HW_W {
        PU_TMX_HW_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn pu_pa_hw(&mut self) -> PU_PA_HW_W {
        PU_PA_HW_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pu_adc_hw(&mut self) -> PU_ADC_HW_W {
        PU_ADC_HW_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn adc_clk_en_hw(&mut self) -> ADC_CLK_EN_HW_W {
        ADC_CLK_EN_HW_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pu_adda_ldo_hw(&mut self) -> PU_ADDA_LDO_HW_W {
        PU_ADDA_LDO_HW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn pu_rbb_hw(&mut self) -> PU_RBB_HW_W {
        PU_RBB_HW_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn pu_rmx_hw(&mut self) -> PU_RMX_HW_W {
        PU_RMX_HW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pu_rmxgm_hw(&mut self) -> PU_RMXGM_HW_W {
        PU_RMXGM_HW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pu_lna_hw(&mut self) -> PU_LNA_HW_W {
        PU_LNA_HW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_sfreg_hw(&mut self) -> PU_SFREG_HW_W {
        PU_SFREG_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "read only from hardware logic\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucr1_hw](index.html) module"]
pub struct PUCR1_HW_SPEC;
impl crate::RegisterSpec for PUCR1_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucr1_hw::R](R) reader structure"]
impl crate::Readable for PUCR1_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucr1_hw::W](W) writer structure"]
impl crate::Writable for PUCR1_HW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pucr1_hw to value 0"]
impl crate::Resettable for PUCR1_HW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
