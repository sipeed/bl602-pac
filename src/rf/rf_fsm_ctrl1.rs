#[doc = "Register `rf_fsm_ctrl1` reader"]
pub struct R(crate::R<RF_FSM_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RF_FSM_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RF_FSM_CTRL1_SPEC>> for R {
    fn from(reader: crate::R<RF_FSM_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `rf_fsm_ctrl1` writer"]
pub struct W(crate::W<RF_FSM_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RF_FSM_CTRL1_SPEC>;
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
impl core::convert::From<crate::W<RF_FSM_CTRL1_SPEC>> for W {
    fn from(writer: crate::W<RF_FSM_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_fsm_pu_pa_dly_n` reader - "]
pub struct RF_FSM_PU_PA_DLY_N_R(crate::FieldReader<u16, u16>);
impl RF_FSM_PU_PA_DLY_N_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_FSM_PU_PA_DLY_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_PU_PA_DLY_N_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_pu_pa_dly_n` writer - "]
pub struct RF_FSM_PU_PA_DLY_N_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_PU_PA_DLY_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Field `rf_fsm_lo_rdy_sbclr` reader - "]
pub struct RF_FSM_LO_RDY_SBCLR_R(crate::FieldReader<bool, bool>);
impl RF_FSM_LO_RDY_SBCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_LO_RDY_SBCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_LO_RDY_SBCLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_lo_rdy_sbclr` writer - "]
pub struct RF_FSM_LO_RDY_SBCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_LO_RDY_SBCLR_W<'a> {
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
#[doc = "Field `rf_fsm_lo_rdy_4s_1` reader - "]
pub struct RF_FSM_LO_RDY_4S_1_R(crate::FieldReader<bool, bool>);
impl RF_FSM_LO_RDY_4S_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_LO_RDY_4S_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_LO_RDY_4S_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_lo_rdy_4s_1` writer - "]
pub struct RF_FSM_LO_RDY_4S_1_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_LO_RDY_4S_1_W<'a> {
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
#[doc = "Field `rf_fsm_lo_rdy_rst` reader - "]
pub struct RF_FSM_LO_RDY_RST_R(crate::FieldReader<bool, bool>);
impl RF_FSM_LO_RDY_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_LO_RDY_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_LO_RDY_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_lo_rdy_rst` writer - "]
pub struct RF_FSM_LO_RDY_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_LO_RDY_RST_W<'a> {
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
#[doc = "Field `rf_fsm_lo_rdy` reader - "]
pub struct RF_FSM_LO_RDY_R(crate::FieldReader<bool, bool>);
impl RF_FSM_LO_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF_FSM_LO_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_LO_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_lo_rdy` writer - "]
pub struct RF_FSM_LO_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_LO_RDY_W<'a> {
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
#[doc = "Field `rf_fsm_lo_time` reader - "]
pub struct RF_FSM_LO_TIME_R(crate::FieldReader<u16, u16>);
impl RF_FSM_LO_TIME_R {
    pub(crate) fn new(bits: u16) -> Self {
        RF_FSM_LO_TIME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_FSM_LO_TIME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_fsm_lo_time` writer - "]
pub struct RF_FSM_LO_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_FSM_LO_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_pu_pa_dly_n(&self) -> RF_FSM_PU_PA_DLY_N_R {
        RF_FSM_PU_PA_DLY_N_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_sbclr(&self) -> RF_FSM_LO_RDY_SBCLR_R {
        RF_FSM_LO_RDY_SBCLR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_4s_1(&self) -> RF_FSM_LO_RDY_4S_1_R {
        RF_FSM_LO_RDY_4S_1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_rst(&self) -> RF_FSM_LO_RDY_RST_R {
        RF_FSM_LO_RDY_RST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy(&self) -> RF_FSM_LO_RDY_R {
        RF_FSM_LO_RDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_fsm_lo_time(&self) -> RF_FSM_LO_TIME_R {
        RF_FSM_LO_TIME_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 20:29"]
    #[inline(always)]
    pub fn rf_fsm_pu_pa_dly_n(&mut self) -> RF_FSM_PU_PA_DLY_N_W {
        RF_FSM_PU_PA_DLY_N_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_sbclr(&mut self) -> RF_FSM_LO_RDY_SBCLR_W {
        RF_FSM_LO_RDY_SBCLR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_4s_1(&mut self) -> RF_FSM_LO_RDY_4S_1_W {
        RF_FSM_LO_RDY_4S_1_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy_rst(&mut self) -> RF_FSM_LO_RDY_RST_W {
        RF_FSM_LO_RDY_RST_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn rf_fsm_lo_rdy(&mut self) -> RF_FSM_LO_RDY_W {
        RF_FSM_LO_RDY_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rf_fsm_lo_time(&mut self) -> RF_FSM_LO_TIME_W {
        RF_FSM_LO_TIME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rf_fsm_ctrl1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rf_fsm_ctrl1](index.html) module"]
pub struct RF_FSM_CTRL1_SPEC;
impl crate::RegisterSpec for RF_FSM_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rf_fsm_ctrl1::R](R) reader structure"]
impl crate::Readable for RF_FSM_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rf_fsm_ctrl1::W](W) writer structure"]
impl crate::Writable for RF_FSM_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets rf_fsm_ctrl1 to value 0"]
impl crate::Resettable for RF_FSM_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
