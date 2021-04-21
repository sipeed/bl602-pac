#[doc = "Register `rfctrl_hw_en` reader"]
pub struct R(crate::R<RFCTRL_HW_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFCTRL_HW_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RFCTRL_HW_EN_SPEC>> for R {
    fn from(reader: crate::R<RFCTRL_HW_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rfctrl_hw_en` writer"]
pub struct W(crate::W<RFCTRL_HW_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFCTRL_HW_EN_SPEC>;
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
impl core::convert::From<crate::W<RFCTRL_HW_EN_SPEC>> for W {
    fn from(writer: crate::W<RFCTRL_HW_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adda_ctrl_hw` reader - "]
pub struct ADDA_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl ADDA_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDA_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDA_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adda_ctrl_hw` writer - "]
pub struct ADDA_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDA_CTRL_HW_W<'a> {
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
#[doc = "Field `rbb_pkdet_out_rstn_ctrl_hw` reader - "]
pub struct RBB_PKDET_OUT_RSTN_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_OUT_RSTN_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_OUT_RSTN_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_OUT_RSTN_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_out_rstn_ctrl_hw` writer - "]
pub struct RBB_PKDET_OUT_RSTN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_OUT_RSTN_CTRL_HW_W<'a> {
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
#[doc = "Field `rbb_pkdet_en_ctrl_hw` reader - "]
pub struct RBB_PKDET_EN_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl RBB_PKDET_EN_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_PKDET_EN_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_PKDET_EN_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_pkdet_en_ctrl_hw` writer - "]
pub struct RBB_PKDET_EN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_PKDET_EN_CTRL_HW_W<'a> {
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
#[doc = "Field `sdm_ctrl_hw` reader - "]
pub struct SDM_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl SDM_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDM_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDM_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdm_ctrl_hw` writer - "]
pub struct SDM_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> SDM_CTRL_HW_W<'a> {
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
#[doc = "Field `inc_fcal_ctrl_en_hw` reader - "]
pub struct INC_FCAL_CTRL_EN_HW_R(crate::FieldReader<bool, bool>);
impl INC_FCAL_CTRL_EN_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_FCAL_CTRL_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_FCAL_CTRL_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_fcal_ctrl_en_hw` writer - "]
pub struct INC_FCAL_CTRL_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_FCAL_CTRL_EN_HW_W<'a> {
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
#[doc = "Field `inc_acal_ctrl_en_hw` reader - "]
pub struct INC_ACAL_CTRL_EN_HW_R(crate::FieldReader<bool, bool>);
impl INC_ACAL_CTRL_EN_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_ACAL_CTRL_EN_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_ACAL_CTRL_EN_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_acal_ctrl_en_hw` writer - "]
pub struct INC_ACAL_CTRL_EN_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_ACAL_CTRL_EN_HW_W<'a> {
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
#[doc = "Field `lo_ctrl_hw` reader - "]
pub struct LO_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl LO_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_ctrl_hw` writer - "]
pub struct LO_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CTRL_HW_W<'a> {
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
#[doc = "Field `trxcal_ctrl_hw` reader - "]
pub struct TRXCAL_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl TRXCAL_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRXCAL_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRXCAL_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `trxcal_ctrl_hw` writer - "]
pub struct TRXCAL_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> TRXCAL_CTRL_HW_W<'a> {
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
#[doc = "Field `rbb_bw_ctrl_hw` reader - "]
pub struct RBB_BW_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl RBB_BW_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBB_BW_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBB_BW_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rbb_bw_ctrl_hw` writer - "]
pub struct RBB_BW_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RBB_BW_CTRL_HW_W<'a> {
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
#[doc = "Field `lna_ctrl_hw` reader - "]
pub struct LNA_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl LNA_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        LNA_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LNA_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lna_ctrl_hw` writer - "]
pub struct LNA_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LNA_CTRL_HW_W<'a> {
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
#[doc = "Field `tx_gain_ctrl_hw` reader - "]
pub struct TX_GAIN_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl TX_GAIN_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TX_GAIN_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_GAIN_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_gain_ctrl_hw` writer - "]
pub struct TX_GAIN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_GAIN_CTRL_HW_W<'a> {
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
#[doc = "Field `rx_gain_ctrl_hw` reader - "]
pub struct RX_GAIN_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl RX_GAIN_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        RX_GAIN_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_GAIN_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_gain_ctrl_hw` writer - "]
pub struct RX_GAIN_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_GAIN_CTRL_HW_W<'a> {
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
#[doc = "Field `pu_ctrl_hw` reader - "]
pub struct PU_CTRL_HW_R(crate::FieldReader<bool, bool>);
impl PU_CTRL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_CTRL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_CTRL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pu_ctrl_hw` writer - "]
pub struct PU_CTRL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_CTRL_HW_W<'a> {
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
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adda_ctrl_hw(&self) -> ADDA_CTRL_HW_R {
        ADDA_CTRL_HW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&self) -> RBB_PKDET_OUT_RSTN_CTRL_HW_R {
        RBB_PKDET_OUT_RSTN_CTRL_HW_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rbb_pkdet_en_ctrl_hw(&self) -> RBB_PKDET_EN_CTRL_HW_R {
        RBB_PKDET_EN_CTRL_HW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sdm_ctrl_hw(&self) -> SDM_CTRL_HW_R {
        SDM_CTRL_HW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn inc_fcal_ctrl_en_hw(&self) -> INC_FCAL_CTRL_EN_HW_R {
        INC_FCAL_CTRL_EN_HW_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inc_acal_ctrl_en_hw(&self) -> INC_ACAL_CTRL_EN_HW_R {
        INC_ACAL_CTRL_EN_HW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_ctrl_hw(&self) -> LO_CTRL_HW_R {
        LO_CTRL_HW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trxcal_ctrl_hw(&self) -> TRXCAL_CTRL_HW_R {
        TRXCAL_CTRL_HW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bw_ctrl_hw(&self) -> RBB_BW_CTRL_HW_R {
        RBB_BW_CTRL_HW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lna_ctrl_hw(&self) -> LNA_CTRL_HW_R {
        LNA_CTRL_HW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_gain_ctrl_hw(&self) -> TX_GAIN_CTRL_HW_R {
        TX_GAIN_CTRL_HW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_gain_ctrl_hw(&self) -> RX_GAIN_CTRL_HW_R {
        RX_GAIN_CTRL_HW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ctrl_hw(&self) -> PU_CTRL_HW_R {
        PU_CTRL_HW_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn adda_ctrl_hw(&mut self) -> ADDA_CTRL_HW_W {
        ADDA_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rbb_pkdet_out_rstn_ctrl_hw(&mut self) -> RBB_PKDET_OUT_RSTN_CTRL_HW_W {
        RBB_PKDET_OUT_RSTN_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rbb_pkdet_en_ctrl_hw(&mut self) -> RBB_PKDET_EN_CTRL_HW_W {
        RBB_PKDET_EN_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sdm_ctrl_hw(&mut self) -> SDM_CTRL_HW_W {
        SDM_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn inc_fcal_ctrl_en_hw(&mut self) -> INC_FCAL_CTRL_EN_HW_W {
        INC_FCAL_CTRL_EN_HW_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn inc_acal_ctrl_en_hw(&mut self) -> INC_ACAL_CTRL_EN_HW_W {
        INC_ACAL_CTRL_EN_HW_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lo_ctrl_hw(&mut self) -> LO_CTRL_HW_W {
        LO_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn trxcal_ctrl_hw(&mut self) -> TRXCAL_CTRL_HW_W {
        TRXCAL_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rbb_bw_ctrl_hw(&mut self) -> RBB_BW_CTRL_HW_W {
        RBB_BW_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lna_ctrl_hw(&mut self) -> LNA_CTRL_HW_W {
        LNA_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tx_gain_ctrl_hw(&mut self) -> TX_GAIN_CTRL_HW_W {
        TX_GAIN_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rx_gain_ctrl_hw(&mut self) -> RX_GAIN_CTRL_HW_W {
        RX_GAIN_CTRL_HW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pu_ctrl_hw(&mut self) -> PU_CTRL_HW_W {
        PU_CTRL_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control logic switch\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfctrl_hw_en](index.html) module"]
pub struct RFCTRL_HW_EN_SPEC;
impl crate::RegisterSpec for RFCTRL_HW_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfctrl_hw_en::R](R) reader structure"]
impl crate::Readable for RFCTRL_HW_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfctrl_hw_en::W](W) writer structure"]
impl crate::Writable for RFCTRL_HW_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rfctrl_hw_en to value 0"]
impl crate::Resettable for RFCTRL_HW_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
