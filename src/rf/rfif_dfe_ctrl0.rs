#[doc = "Register `rfif_dfe_ctrl0` reader"]
pub struct R(crate::R<RFIF_DFE_CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFIF_DFE_CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RFIF_DFE_CTRL0_SPEC>> for R {
    fn from(reader: crate::R<RFIF_DFE_CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfif_dfe_ctrl0` writer"]
pub struct W(crate::W<RFIF_DFE_CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFIF_DFE_CTRL0_SPEC>;
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
impl core::convert::From<crate::W<RFIF_DFE_CTRL0_SPEC>> for W {
    fn from(writer: crate::W<RFIF_DFE_CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `test_sel` reader - "]
pub struct TEST_SEL_R(crate::FieldReader<u8, u8>);
impl TEST_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEST_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `test_sel` writer - "]
pub struct TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `bbmode_4s_en` reader - "]
pub struct BBMODE_4S_EN_R(crate::FieldReader<bool, bool>);
impl BBMODE_4S_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BBMODE_4S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBMODE_4S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bbmode_4s_en` writer - "]
pub struct BBMODE_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BBMODE_4S_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `bbmode_4s` reader - "]
pub struct BBMODE_4S_R(crate::FieldReader<bool, bool>);
impl BBMODE_4S_R {
    pub(crate) fn new(bits: bool) -> Self {
        BBMODE_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BBMODE_4S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bbmode_4s` writer - "]
pub struct BBMODE_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> BBMODE_4S_W<'a> {
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
#[doc = "Field `wifimode_4s_en` reader - "]
pub struct WIFIMODE_4S_EN_R(crate::FieldReader<bool, bool>);
impl WIFIMODE_4S_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFIMODE_4S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFIMODE_4S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wifimode_4s_en` writer - "]
pub struct WIFIMODE_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFIMODE_4S_EN_W<'a> {
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
#[doc = "Field `wifimode_4s` reader - "]
pub struct WIFIMODE_4S_R(crate::FieldReader<u8, u8>);
impl WIFIMODE_4S_R {
    pub(crate) fn new(bits: u8) -> Self {
        WIFIMODE_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFIMODE_4S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wifimode_4s` writer - "]
pub struct WIFIMODE_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFIMODE_4S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 23)) | ((value as u32 & 0x03) << 23);
        self.w
    }
}
#[doc = "Field `rf_ch_ind_ble_4s_en` reader - "]
pub struct RF_CH_IND_BLE_4S_EN_R(crate::FieldReader<bool, bool>);
impl RF_CH_IND_BLE_4S_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_CH_IND_BLE_4S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_CH_IND_BLE_4S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ch_ind_ble_4s_en` writer - "]
pub struct RF_CH_IND_BLE_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_CH_IND_BLE_4S_EN_W<'a> {
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
#[doc = "Field `rf_ch_ind_ble_4s` reader - "]
pub struct RF_CH_IND_BLE_4S_R(crate::FieldReader<u8, u8>);
impl RF_CH_IND_BLE_4S_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_CH_IND_BLE_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_CH_IND_BLE_4S_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_ch_ind_ble_4s` writer - "]
pub struct RF_CH_IND_BLE_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_CH_IND_BLE_4S_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 15)) | ((value as u32 & 0x7f) << 15);
        self.w
    }
}
#[doc = "Field `pad_dac_clkout_inv_en` reader - "]
pub struct PAD_DAC_CLKOUT_INV_EN_R(crate::FieldReader<bool, bool>);
impl PAD_DAC_CLKOUT_INV_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD_DAC_CLKOUT_INV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_DAC_CLKOUT_INV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_dac_clkout_inv_en` writer - "]
pub struct PAD_DAC_CLKOUT_INV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_DAC_CLKOUT_INV_EN_W<'a> {
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
#[doc = "Field `pad_adc_clkout_inv_en` reader - "]
pub struct PAD_ADC_CLKOUT_INV_EN_R(crate::FieldReader<bool, bool>);
impl PAD_ADC_CLKOUT_INV_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PAD_ADC_CLKOUT_INV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_ADC_CLKOUT_INV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_adc_clkout_inv_en` writer - "]
pub struct PAD_ADC_CLKOUT_INV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_ADC_CLKOUT_INV_EN_W<'a> {
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
#[doc = "Field `tx_test_sel` reader - "]
pub struct TX_TEST_SEL_R(crate::FieldReader<u8, u8>);
impl TX_TEST_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_TEST_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TEST_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_test_sel` writer - "]
pub struct TX_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `rx_test_sel` reader - "]
pub struct RX_TEST_SEL_R(crate::FieldReader<u8, u8>);
impl RX_TEST_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RX_TEST_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_TEST_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_test_sel` writer - "]
pub struct RX_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Field `tx_dfe_en_4s_en` reader - "]
pub struct TX_DFE_EN_4S_EN_R(crate::FieldReader<bool, bool>);
impl TX_DFE_EN_4S_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DFE_EN_4S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DFE_EN_4S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dfe_en_4s_en` writer - "]
pub struct TX_DFE_EN_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DFE_EN_4S_EN_W<'a> {
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
#[doc = "Field `tx_dfe_en_4s` reader - "]
pub struct TX_DFE_EN_4S_R(crate::FieldReader<bool, bool>);
impl TX_DFE_EN_4S_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_DFE_EN_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DFE_EN_4S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dfe_en_4s` writer - "]
pub struct TX_DFE_EN_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DFE_EN_4S_W<'a> {
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
#[doc = "Field `rx_dfe_en_4s_en` reader - "]
pub struct RX_DFE_EN_4S_EN_R(crate::FieldReader<bool, bool>);
impl RX_DFE_EN_4S_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DFE_EN_4S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DFE_EN_4S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_dfe_en_4s_en` writer - "]
pub struct RX_DFE_EN_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DFE_EN_4S_EN_W<'a> {
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
#[doc = "Field `rx_dfe_en_4s` reader - "]
pub struct RX_DFE_EN_4S_R(crate::FieldReader<bool, bool>);
impl RX_DFE_EN_4S_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_DFE_EN_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DFE_EN_4S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_dfe_en_4s` writer - "]
pub struct RX_DFE_EN_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DFE_EN_4S_W<'a> {
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
#[doc = "Field `rfckg_dac_afifo_inv` reader - "]
pub struct RFCKG_DAC_AFIFO_INV_R(crate::FieldReader<bool, bool>);
impl RFCKG_DAC_AFIFO_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCKG_DAC_AFIFO_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCKG_DAC_AFIFO_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfckg_dac_afifo_inv` writer - "]
pub struct RFCKG_DAC_AFIFO_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_DAC_AFIFO_INV_W<'a> {
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
#[doc = "Field `rfckg_adc_clkout_sel` reader - "]
pub struct RFCKG_ADC_CLKOUT_SEL_R(crate::FieldReader<bool, bool>);
impl RFCKG_ADC_CLKOUT_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCKG_ADC_CLKOUT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCKG_ADC_CLKOUT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfckg_adc_clkout_sel` writer - "]
pub struct RFCKG_ADC_CLKOUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_ADC_CLKOUT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `rfckg_adc_afifo_inv` reader - "]
pub struct RFCKG_ADC_AFIFO_INV_R(crate::FieldReader<bool, bool>);
impl RFCKG_ADC_AFIFO_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCKG_ADC_AFIFO_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCKG_ADC_AFIFO_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfckg_adc_afifo_inv` writer - "]
pub struct RFCKG_ADC_AFIFO_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_ADC_AFIFO_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `rfckg_txclk_4s_on` reader - "]
pub struct RFCKG_TXCLK_4S_ON_R(crate::FieldReader<bool, bool>);
impl RFCKG_TXCLK_4S_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCKG_TXCLK_4S_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCKG_TXCLK_4S_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfckg_txclk_4s_on` writer - "]
pub struct RFCKG_TXCLK_4S_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_TXCLK_4S_ON_W<'a> {
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
#[doc = "Field `rfckg_rxclk_4s_on` reader - "]
pub struct RFCKG_RXCLK_4S_ON_R(crate::FieldReader<bool, bool>);
impl RFCKG_RXCLK_4S_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        RFCKG_RXCLK_4S_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFCKG_RXCLK_4S_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rfckg_rxclk_4s_on` writer - "]
pub struct RFCKG_RXCLK_4S_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> RFCKG_RXCLK_4S_ON_W<'a> {
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
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn test_sel(&self) -> TEST_SEL_R {
        TEST_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bbmode_4s_en(&self) -> BBMODE_4S_EN_R {
        BBMODE_4S_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bbmode_4s(&self) -> BBMODE_4S_R {
        BBMODE_4S_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wifimode_4s_en(&self) -> WIFIMODE_4S_EN_R {
        WIFIMODE_4S_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn wifimode_4s(&self) -> WIFIMODE_4S_R {
        WIFIMODE_4S_R::new(((self.bits >> 23) & 0x03) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s_en(&self) -> RF_CH_IND_BLE_4S_EN_R {
        RF_CH_IND_BLE_4S_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 15:21"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s(&self) -> RF_CH_IND_BLE_4S_R {
        RF_CH_IND_BLE_4S_R::new(((self.bits >> 15) & 0x7f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pad_dac_clkout_inv_en(&self) -> PAD_DAC_CLKOUT_INV_EN_R {
        PAD_DAC_CLKOUT_INV_EN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pad_adc_clkout_inv_en(&self) -> PAD_ADC_CLKOUT_INV_EN_R {
        PAD_ADC_CLKOUT_INV_EN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn tx_test_sel(&self) -> TX_TEST_SEL_R {
        TX_TEST_SEL_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rx_test_sel(&self) -> RX_TEST_SEL_R {
        RX_TEST_SEL_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_dfe_en_4s_en(&self) -> TX_DFE_EN_4S_EN_R {
        TX_DFE_EN_4S_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_dfe_en_4s(&self) -> TX_DFE_EN_4S_R {
        TX_DFE_EN_4S_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_dfe_en_4s_en(&self) -> RX_DFE_EN_4S_EN_R {
        RX_DFE_EN_4S_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_dfe_en_4s(&self) -> RX_DFE_EN_4S_R {
        RX_DFE_EN_4S_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rfckg_dac_afifo_inv(&self) -> RFCKG_DAC_AFIFO_INV_R {
        RFCKG_DAC_AFIFO_INV_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfckg_adc_clkout_sel(&self) -> RFCKG_ADC_CLKOUT_SEL_R {
        RFCKG_ADC_CLKOUT_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_adc_afifo_inv(&self) -> RFCKG_ADC_AFIFO_INV_R {
        RFCKG_ADC_AFIFO_INV_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfckg_txclk_4s_on(&self) -> RFCKG_TXCLK_4S_ON_R {
        RFCKG_TXCLK_4S_ON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfckg_rxclk_4s_on(&self) -> RFCKG_RXCLK_4S_ON_R {
        RFCKG_RXCLK_4S_ON_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn test_sel(&mut self) -> TEST_SEL_W {
        TEST_SEL_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn bbmode_4s_en(&mut self) -> BBMODE_4S_EN_W {
        BBMODE_4S_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn bbmode_4s(&mut self) -> BBMODE_4S_W {
        BBMODE_4S_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn wifimode_4s_en(&mut self) -> WIFIMODE_4S_EN_W {
        WIFIMODE_4S_EN_W { w: self }
    }
    #[doc = "Bits 23:24"]
    #[inline(always)]
    pub fn wifimode_4s(&mut self) -> WIFIMODE_4S_W {
        WIFIMODE_4S_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s_en(&mut self) -> RF_CH_IND_BLE_4S_EN_W {
        RF_CH_IND_BLE_4S_EN_W { w: self }
    }
    #[doc = "Bits 15:21"]
    #[inline(always)]
    pub fn rf_ch_ind_ble_4s(&mut self) -> RF_CH_IND_BLE_4S_W {
        RF_CH_IND_BLE_4S_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pad_dac_clkout_inv_en(&mut self) -> PAD_DAC_CLKOUT_INV_EN_W {
        PAD_DAC_CLKOUT_INV_EN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn pad_adc_clkout_inv_en(&mut self) -> PAD_ADC_CLKOUT_INV_EN_W {
        PAD_ADC_CLKOUT_INV_EN_W { w: self }
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn tx_test_sel(&mut self) -> TX_TEST_SEL_W {
        TX_TEST_SEL_W { w: self }
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn rx_test_sel(&mut self) -> RX_TEST_SEL_W {
        RX_TEST_SEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tx_dfe_en_4s_en(&mut self) -> TX_DFE_EN_4S_EN_W {
        TX_DFE_EN_4S_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tx_dfe_en_4s(&mut self) -> TX_DFE_EN_4S_W {
        TX_DFE_EN_4S_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rx_dfe_en_4s_en(&mut self) -> RX_DFE_EN_4S_EN_W {
        RX_DFE_EN_4S_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rx_dfe_en_4s(&mut self) -> RX_DFE_EN_4S_W {
        RX_DFE_EN_4S_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rfckg_dac_afifo_inv(&mut self) -> RFCKG_DAC_AFIFO_INV_W {
        RFCKG_DAC_AFIFO_INV_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rfckg_adc_clkout_sel(&mut self) -> RFCKG_ADC_CLKOUT_SEL_W {
        RFCKG_ADC_CLKOUT_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rfckg_adc_afifo_inv(&mut self) -> RFCKG_ADC_AFIFO_INV_W {
        RFCKG_ADC_AFIFO_INV_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rfckg_txclk_4s_on(&mut self) -> RFCKG_TXCLK_4S_ON_W {
        RFCKG_TXCLK_4S_ON_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rfckg_rxclk_4s_on(&mut self) -> RFCKG_RXCLK_4S_ON_W {
        RFCKG_RXCLK_4S_ON_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rfif_dfe_ctrl0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfif_dfe_ctrl0](index.html) module"]
pub struct RFIF_DFE_CTRL0_SPEC;
impl crate::RegisterSpec for RFIF_DFE_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfif_dfe_ctrl0::R](R) reader structure"]
impl crate::Readable for RFIF_DFE_CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfif_dfe_ctrl0::W](W) writer structure"]
impl crate::Writable for RFIF_DFE_CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rfif_dfe_ctrl0 to value 0"]
impl crate::Resettable for RFIF_DFE_CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
