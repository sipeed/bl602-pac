#[doc = "Register `rf_fsm_ctrl2` reader"]
pub struct R(crate::R<RF_FSM_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_FSM_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RF_FSM_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RF_FSM_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_fsm_ctrl2` writer"]
pub struct W(crate::W<RF_FSM_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_FSM_CTRL2_SPEC>;
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
impl From<crate::W<RF_FSM_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RF_FSM_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_fsm_dfe_rx_dly_n` reader - "]
pub struct RF_FSM_DFE_RX_DLY_N_R(crate::FieldReader<u16, u16>);
impl RF_FSM_DFE_RX_DLY_N_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_FSM_DFE_RX_DLY_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_DFE_RX_DLY_N_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_dfe_rx_dly_n` writer - "]
pub struct RF_FSM_DFE_RX_DLY_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_DFE_RX_DLY_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Field `rf_fsm_dfe_tx_dly_n` reader - "]
pub struct RF_FSM_DFE_TX_DLY_N_R(crate::FieldReader<u16, u16>);
impl RF_FSM_DFE_TX_DLY_N_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_FSM_DFE_TX_DLY_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_DFE_TX_DLY_N_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_dfe_tx_dly_n` writer - "]
pub struct RF_FSM_DFE_TX_DLY_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_DFE_TX_DLY_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `rf_trx_ble_4s_en` reader - "]
pub struct RF_TRX_BLE_4S_EN_R(crate::FieldReader<bool, bool>);
impl RF_TRX_BLE_4S_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_TRX_BLE_4S_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TRX_BLE_4S_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_trx_ble_4s_en` writer - "]
pub struct RF_TRX_BLE_4S_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TRX_BLE_4S_EN_W<'a> {
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
#[doc = "Field `rf_trx_sw_ble_4s` reader - "]
pub struct RF_TRX_SW_BLE_4S_R(crate::FieldReader<bool, bool>);
impl RF_TRX_SW_BLE_4S_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_TRX_SW_BLE_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TRX_SW_BLE_4S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_trx_sw_ble_4s` writer - "]
pub struct RF_TRX_SW_BLE_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TRX_SW_BLE_4S_W<'a> {
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
#[doc = "Field `rf_trx_en_ble_4s` reader - "]
pub struct RF_TRX_EN_BLE_4S_R(crate::FieldReader<bool, bool>);
impl RF_TRX_EN_BLE_4S_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_TRX_EN_BLE_4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TRX_EN_BLE_4S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_trx_en_ble_4s` writer - "]
pub struct RF_TRX_EN_BLE_4S_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TRX_EN_BLE_4S_W<'a> {
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
#[doc = "Field `rf_fsm_st_dbg_en` reader - "]
pub struct RF_FSM_ST_DBG_EN_R(crate::FieldReader<bool, bool>);
impl RF_FSM_ST_DBG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_ST_DBG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_ST_DBG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_st_dbg_en` writer - "]
pub struct RF_FSM_ST_DBG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_DBG_EN_W<'a> {
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
#[doc = "Field `rf_fsm_st_dbg` reader - "]
pub struct RF_FSM_ST_DBG_R(crate::FieldReader<u8, u8>);
impl RF_FSM_ST_DBG_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_FSM_ST_DBG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_ST_DBG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_st_dbg` writer - "]
pub struct RF_FSM_ST_DBG_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_DBG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_dfe_rx_dly_n(&self) -> RF_FSM_DFE_RX_DLY_N_R {
        RF_FSM_DFE_RX_DLY_N_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_fsm_dfe_tx_dly_n(&self) -> RF_FSM_DFE_TX_DLY_N_R {
        RF_FSM_DFE_TX_DLY_N_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rf_trx_ble_4s_en(&self) -> RF_TRX_BLE_4S_EN_R {
        RF_TRX_BLE_4S_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rf_trx_sw_ble_4s(&self) -> RF_TRX_SW_BLE_4S_R {
        RF_TRX_SW_BLE_4S_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rf_trx_en_ble_4s(&self) -> RF_TRX_EN_BLE_4S_R {
        RF_TRX_EN_BLE_4S_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg_en(&self) -> RF_FSM_ST_DBG_EN_R {
        RF_FSM_ST_DBG_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg(&self) -> RF_FSM_ST_DBG_R {
        RF_FSM_ST_DBG_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_dfe_rx_dly_n(&mut self) -> RF_FSM_DFE_RX_DLY_N_W {
        RF_FSM_DFE_RX_DLY_N_W { w: self }
    }
    #[doc = "Bits 10:19"]
    #[inline(always)]
    pub fn rf_fsm_dfe_tx_dly_n(&mut self) -> RF_FSM_DFE_TX_DLY_N_W {
        RF_FSM_DFE_TX_DLY_N_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rf_trx_ble_4s_en(&mut self) -> RF_TRX_BLE_4S_EN_W {
        RF_TRX_BLE_4S_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rf_trx_sw_ble_4s(&mut self) -> RF_TRX_SW_BLE_4S_W {
        RF_TRX_SW_BLE_4S_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rf_trx_en_ble_4s(&mut self) -> RF_TRX_EN_BLE_4S_W {
        RF_TRX_EN_BLE_4S_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg_en(&mut self) -> RF_FSM_ST_DBG_EN_W {
        RF_FSM_ST_DBG_EN_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_fsm_st_dbg(&mut self) -> RF_FSM_ST_DBG_W {
        RF_FSM_ST_DBG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_fsm_ctrl2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl2](index.html) module"]
pub struct RF_FSM_CTRL2_SPEC;
impl crate::RegisterSpec for RF_FSM_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_fsm_ctrl2::R](R) reader structure"]
impl crate::Readable for RF_FSM_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl2::W](W) writer structure"]
impl crate::Writable for RF_FSM_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_fsm_ctrl2 to value 0"]
impl crate::Resettable for RF_FSM_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
