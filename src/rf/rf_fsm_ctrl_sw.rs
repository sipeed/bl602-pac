#[doc = "Register `rf_fsm_ctrl_sw` reader"]
pub struct R(crate::R<RF_FSM_CTRL_SW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_FSM_CTRL_SW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RF_FSM_CTRL_SW_SPEC>> for R {
    fn from(reader: crate::R<RF_FSM_CTRL_SW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_fsm_ctrl_sw` writer"]
pub struct W(crate::W<RF_FSM_CTRL_SW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_FSM_CTRL_SW_SPEC>;
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
impl core::convert::From<crate::W<RF_FSM_CTRL_SW_SPEC>> for W {
    fn from(writer: crate::W<RF_FSM_CTRL_SW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_unlocked` reader - "]
pub struct LO_UNLOCKED_R(crate::FieldReader<bool, bool>);
impl LO_UNLOCKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_UNLOCKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_UNLOCKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_unlocked` writer - "]
pub struct LO_UNLOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_UNLOCKED_W<'a> {
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
#[doc = "Field `inc_cal_timeout` reader - "]
pub struct INC_CAL_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl INC_CAL_TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INC_CAL_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INC_CAL_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inc_cal_timeout` writer - "]
pub struct INC_CAL_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> INC_CAL_TIMEOUT_W<'a> {
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
#[doc = "Field `full_cal_en` reader - "]
pub struct FULL_CAL_EN_R(crate::FieldReader<bool, bool>);
impl FULL_CAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FULL_CAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULL_CAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `full_cal_en` writer - "]
pub struct FULL_CAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FULL_CAL_EN_W<'a> {
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
#[doc = "Field `rf_fsm_sw_st_vld` reader - "]
pub struct RF_FSM_SW_ST_VLD_R(crate::FieldReader<bool, bool>);
impl RF_FSM_SW_ST_VLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_SW_ST_VLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_SW_ST_VLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_sw_st_vld` writer - "]
pub struct RF_FSM_SW_ST_VLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_SW_ST_VLD_W<'a> {
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
#[doc = "Field `rf_fsm_sw_st` reader - "]
pub struct RF_FSM_SW_ST_R(crate::FieldReader<u8, u8>);
impl RF_FSM_SW_ST_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_FSM_SW_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_SW_ST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_sw_st` writer - "]
pub struct RF_FSM_SW_ST_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_SW_ST_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_unlocked(&self) -> LO_UNLOCKED_R {
        LO_UNLOCKED_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn inc_cal_timeout(&self) -> INC_CAL_TIMEOUT_R {
        INC_CAL_TIMEOUT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn full_cal_en(&self) -> FULL_CAL_EN_R {
        FULL_CAL_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rf_fsm_sw_st_vld(&self) -> RF_FSM_SW_ST_VLD_R {
        RF_FSM_SW_ST_VLD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_fsm_sw_st(&self) -> RF_FSM_SW_ST_R {
        RF_FSM_SW_ST_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_unlocked(&mut self) -> LO_UNLOCKED_W {
        LO_UNLOCKED_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn inc_cal_timeout(&mut self) -> INC_CAL_TIMEOUT_W {
        INC_CAL_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn full_cal_en(&mut self) -> FULL_CAL_EN_W {
        FULL_CAL_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rf_fsm_sw_st_vld(&mut self) -> RF_FSM_SW_ST_VLD_W {
        RF_FSM_SW_ST_VLD_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn rf_fsm_sw_st(&mut self) -> RF_FSM_SW_ST_W {
        RF_FSM_SW_ST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rfsm status reg\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl_sw](index.html) module"]
pub struct RF_FSM_CTRL_SW_SPEC;
impl crate::RegisterSpec for RF_FSM_CTRL_SW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_fsm_ctrl_sw::R](R) reader structure"]
impl crate::Readable for RF_FSM_CTRL_SW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl_sw::W](W) writer structure"]
impl crate::Writable for RF_FSM_CTRL_SW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_fsm_ctrl_sw to value 0"]
impl crate::Resettable for RF_FSM_CTRL_SW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
