#[doc = "Register `rf_fsm_ctrl_hw` reader"]
pub struct R(crate::R<RF_FSM_CTRL_HW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_FSM_CTRL_HW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RF_FSM_CTRL_HW_SPEC>> for R {
    fn from(reader: crate::R<RF_FSM_CTRL_HW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_fsm_ctrl_hw` writer"]
pub struct W(crate::W<RF_FSM_CTRL_HW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_FSM_CTRL_HW_SPEC>;
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
impl core::convert::From<crate::W<RF_FSM_CTRL_HW_SPEC>> for W {
    fn from(writer: crate::W<RF_FSM_CTRL_HW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_rc_state_value` reader - "]
pub struct RF_RC_STATE_VALUE_R(crate::FieldReader<u8, u8>);
impl RF_RC_STATE_VALUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_RC_STATE_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RC_STATE_VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_rc_state_value` writer - "]
pub struct RF_RC_STATE_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RC_STATE_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `rf_fsm_st_int_set` reader - "]
pub struct RF_FSM_ST_INT_SET_R(crate::FieldReader<bool, bool>);
impl RF_FSM_ST_INT_SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_ST_INT_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_ST_INT_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_st_int_set` writer - "]
pub struct RF_FSM_ST_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_INT_SET_W<'a> {
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
#[doc = "Field `rf_fsm_st_int_clr` reader - "]
pub struct RF_FSM_ST_INT_CLR_R(crate::FieldReader<bool, bool>);
impl RF_FSM_ST_INT_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_ST_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_ST_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_st_int_clr` writer - "]
pub struct RF_FSM_ST_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_INT_CLR_W<'a> {
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
#[doc = "Field `rf_fsm_st_int` reader - "]
pub struct RF_FSM_ST_INT_R(crate::FieldReader<bool, bool>);
impl RF_FSM_ST_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_ST_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_ST_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_st_int` writer - "]
pub struct RF_FSM_ST_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_INT_W<'a> {
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
#[doc = "Field `rf_fsm_st_int_sel` reader - "]
pub struct RF_FSM_ST_INT_SEL_R(crate::FieldReader<u8, u8>);
impl RF_FSM_ST_INT_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_FSM_ST_INT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_ST_INT_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_st_int_sel` writer - "]
pub struct RF_FSM_ST_INT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_ST_INT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `rf_rc_state_dbg_en` reader - "]
pub struct RF_RC_STATE_DBG_EN_R(crate::FieldReader<bool, bool>);
impl RF_RC_STATE_DBG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_RC_STATE_DBG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RC_STATE_DBG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_rc_state_dbg_en` writer - "]
pub struct RF_RC_STATE_DBG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RC_STATE_DBG_EN_W<'a> {
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
#[doc = "Field `rf_rc_state_dbg` reader - "]
pub struct RF_RC_STATE_DBG_R(crate::FieldReader<u8, u8>);
impl RF_RC_STATE_DBG_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_RC_STATE_DBG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_RC_STATE_DBG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_rc_state_dbg` writer - "]
pub struct RF_RC_STATE_DBG_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_RC_STATE_DBG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `rf_fsm_state` reader - "]
pub struct RF_FSM_STATE_R(crate::FieldReader<u8, u8>);
impl RF_FSM_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_FSM_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_state` writer - "]
pub struct RF_FSM_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `rf_fsm_t2r_cal_mode` reader - "]
pub struct RF_FSM_T2R_CAL_MODE_R(crate::FieldReader<u8, u8>);
impl RF_FSM_T2R_CAL_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_FSM_T2R_CAL_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_T2R_CAL_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_t2r_cal_mode` writer - "]
pub struct RF_FSM_T2R_CAL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_T2R_CAL_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `rf_fsm_ctrl_en` reader - "]
pub struct RF_FSM_CTRL_EN_R(crate::FieldReader<bool, bool>);
impl RF_FSM_CTRL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_CTRL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_CTRL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_ctrl_en` writer - "]
pub struct RF_FSM_CTRL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_CTRL_EN_W<'a> {
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
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_rc_state_value(&self) -> RF_RC_STATE_VALUE_R {
        RF_RC_STATE_VALUE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rf_fsm_st_int_set(&self) -> RF_FSM_ST_INT_SET_R {
        RF_FSM_ST_INT_SET_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rf_fsm_st_int_clr(&self) -> RF_FSM_ST_INT_CLR_R {
        RF_FSM_ST_INT_CLR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_st_int(&self) -> RF_FSM_ST_INT_R {
        RF_FSM_ST_INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_fsm_st_int_sel(&self) -> RF_FSM_ST_INT_SEL_R {
        RF_FSM_ST_INT_SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rf_rc_state_dbg_en(&self) -> RF_RC_STATE_DBG_EN_R {
        RF_RC_STATE_DBG_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_rc_state_dbg(&self) -> RF_RC_STATE_DBG_R {
        RF_RC_STATE_DBG_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_fsm_state(&self) -> RF_FSM_STATE_R {
        RF_FSM_STATE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rf_fsm_t2r_cal_mode(&self) -> RF_FSM_T2R_CAL_MODE_R {
        RF_FSM_T2R_CAL_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_fsm_ctrl_en(&self) -> RF_FSM_CTRL_EN_R {
        RF_FSM_CTRL_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_rc_state_value(&mut self) -> RF_RC_STATE_VALUE_W {
        RF_RC_STATE_VALUE_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn rf_fsm_st_int_set(&mut self) -> RF_FSM_ST_INT_SET_W {
        RF_FSM_ST_INT_SET_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rf_fsm_st_int_clr(&mut self) -> RF_FSM_ST_INT_CLR_W {
        RF_FSM_ST_INT_CLR_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_st_int(&mut self) -> RF_FSM_ST_INT_W {
        RF_FSM_ST_INT_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_fsm_st_int_sel(&mut self) -> RF_FSM_ST_INT_SEL_W {
        RF_FSM_ST_INT_SEL_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rf_rc_state_dbg_en(&mut self) -> RF_RC_STATE_DBG_EN_W {
        RF_RC_STATE_DBG_EN_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_rc_state_dbg(&mut self) -> RF_RC_STATE_DBG_W {
        RF_RC_STATE_DBG_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_fsm_state(&mut self) -> RF_FSM_STATE_W {
        RF_FSM_STATE_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn rf_fsm_t2r_cal_mode(&mut self) -> RF_FSM_T2R_CAL_MODE_W {
        RF_FSM_T2R_CAL_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rf_fsm_ctrl_en(&mut self) -> RF_FSM_CTRL_EN_W {
        RF_FSM_CTRL_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl_hw](index.html) module"]
pub struct RF_FSM_CTRL_HW_SPEC;
impl crate::RegisterSpec for RF_FSM_CTRL_HW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_fsm_ctrl_hw::R](R) reader structure"]
impl crate::Readable for RF_FSM_CTRL_HW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl_hw::W](W) writer structure"]
impl crate::Writable for RF_FSM_CTRL_HW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_fsm_ctrl_hw to value 0"]
impl crate::Resettable for RF_FSM_CTRL_HW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
