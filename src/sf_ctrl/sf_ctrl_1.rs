#[doc = "Register `sf_ctrl_1` reader"]
pub struct R(crate::R<SF_CTRL_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SF_CTRL_1_SPEC>> for R {
    fn from(reader: crate::R<SF_CTRL_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_1` writer"]
pub struct W(crate::W<SF_CTRL_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_1_SPEC>;
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
impl core::convert::From<crate::W<SF_CTRL_1_SPEC>> for W {
    fn from(writer: crate::W<SF_CTRL_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_ahb2sram_en` reader - "]
pub struct SF_AHB2SRAM_EN_R(crate::FieldReader<bool, bool>);
impl SF_AHB2SRAM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AHB2SRAM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AHB2SRAM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_ahb2sram_en` writer - "]
pub struct SF_AHB2SRAM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AHB2SRAM_EN_W<'a> {
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
#[doc = "Field `sf_ahb2sif_en` reader - "]
pub struct SF_AHB2SIF_EN_R(crate::FieldReader<bool, bool>);
impl SF_AHB2SIF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AHB2SIF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AHB2SIF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_ahb2sif_en` writer - "]
pub struct SF_AHB2SIF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AHB2SIF_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `sf_if_en` reader - "]
pub struct SF_IF_EN_R(crate::FieldReader<bool, bool>);
impl SF_IF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_en` writer - "]
pub struct SF_IF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_EN_W<'a> {
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
#[doc = "Field `sf_if_fn_sel` reader - "]
pub struct SF_IF_FN_SEL_R(crate::FieldReader<bool, bool>);
impl SF_IF_FN_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_FN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_FN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_fn_sel` writer - "]
pub struct SF_IF_FN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_FN_SEL_W<'a> {
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
#[doc = "Field `sf_ahb2sif_stop` reader - "]
pub struct SF_AHB2SIF_STOP_R(crate::FieldReader<bool, bool>);
impl SF_AHB2SIF_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AHB2SIF_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AHB2SIF_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_ahb2sif_stop` writer - "]
pub struct SF_AHB2SIF_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AHB2SIF_STOP_W<'a> {
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
#[doc = "Field `sf_ahb2sif_stopped` reader - "]
pub struct SF_AHB2SIF_STOPPED_R(crate::FieldReader<bool, bool>);
impl SF_AHB2SIF_STOPPED_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AHB2SIF_STOPPED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AHB2SIF_STOPPED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_reg_wp` reader - "]
pub struct SF_IF_REG_WP_R(crate::FieldReader<bool, bool>);
impl SF_IF_REG_WP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_REG_WP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_REG_WP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_reg_wp` writer - "]
pub struct SF_IF_REG_WP_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_REG_WP_W<'a> {
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
#[doc = "Field `sf_if_reg_hold` reader - "]
pub struct SF_IF_REG_HOLD_R(crate::FieldReader<bool, bool>);
impl SF_IF_REG_HOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_REG_HOLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_REG_HOLD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_reg_hold` writer - "]
pub struct SF_IF_REG_HOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_REG_HOLD_W<'a> {
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
#[doc = "Field `sf_if_0_ack_lat` reader - "]
pub struct SF_IF_0_ACK_LAT_R(crate::FieldReader<u8, u8>);
impl SF_IF_0_ACK_LAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IF_0_ACK_LAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_0_ACK_LAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_0_ack_lat` writer - "]
pub struct SF_IF_0_ACK_LAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_0_ACK_LAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `sf_if_sr_int_set` reader - "]
pub struct SF_IF_SR_INT_SET_R(crate::FieldReader<bool, bool>);
impl SF_IF_SR_INT_SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_SR_INT_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_SR_INT_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_sr_int_set` writer - "]
pub struct SF_IF_SR_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_SR_INT_SET_W<'a> {
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
#[doc = "Field `sf_if_sr_int_en` reader - "]
pub struct SF_IF_SR_INT_EN_R(crate::FieldReader<bool, bool>);
impl SF_IF_SR_INT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_SR_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_SR_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_sr_int_en` writer - "]
pub struct SF_IF_SR_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_SR_INT_EN_W<'a> {
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
#[doc = "Field `sf_if_sr_int` reader - "]
pub struct SF_IF_SR_INT_R(crate::FieldReader<bool, bool>);
impl SF_IF_SR_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_SR_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_SR_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_sr_pat` reader - "]
pub struct SF_IF_SR_PAT_R(crate::FieldReader<u8, u8>);
impl SF_IF_SR_PAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IF_SR_PAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_SR_PAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_sr_pat` writer - "]
pub struct SF_IF_SR_PAT_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_SR_PAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `sf_if_sr_pat_mask` reader - "]
pub struct SF_IF_SR_PAT_MASK_R(crate::FieldReader<u8, u8>);
impl SF_IF_SR_PAT_MASK_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IF_SR_PAT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_SR_PAT_MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_sr_pat_mask` writer - "]
pub struct SF_IF_SR_PAT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_SR_PAT_MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_ahb2sram_en(&self) -> SF_AHB2SRAM_EN_R {
        SF_AHB2SRAM_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_ahb2sif_en(&self) -> SF_AHB2SIF_EN_R {
        SF_AHB2SIF_EN_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if_en(&self) -> SF_IF_EN_R {
        SF_IF_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if_fn_sel(&self) -> SF_IF_FN_SEL_R {
        SF_IF_FN_SEL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_ahb2sif_stop(&self) -> SF_AHB2SIF_STOP_R {
        SF_AHB2SIF_STOP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn sf_ahb2sif_stopped(&self) -> SF_AHB2SIF_STOPPED_R {
        SF_AHB2SIF_STOPPED_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_reg_wp(&self) -> SF_IF_REG_WP_R {
        SF_IF_REG_WP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_reg_hold(&self) -> SF_IF_REG_HOLD_R {
        SF_IF_REG_HOLD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_0_ack_lat(&self) -> SF_IF_0_ACK_LAT_R {
        SF_IF_0_ACK_LAT_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_sr_int_set(&self) -> SF_IF_SR_INT_SET_R {
        SF_IF_SR_INT_SET_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_sr_int_en(&self) -> SF_IF_SR_INT_EN_R {
        SF_IF_SR_INT_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if_sr_int(&self) -> SF_IF_SR_INT_R {
        SF_IF_SR_INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sf_if_sr_pat(&self) -> SF_IF_SR_PAT_R {
        SF_IF_SR_PAT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sf_if_sr_pat_mask(&self) -> SF_IF_SR_PAT_MASK_R {
        SF_IF_SR_PAT_MASK_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sf_ahb2sram_en(&mut self) -> SF_AHB2SRAM_EN_W {
        SF_AHB2SRAM_EN_W { w: self }
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sf_ahb2sif_en(&mut self) -> SF_AHB2SIF_EN_W {
        SF_AHB2SIF_EN_W { w: self }
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sf_if_en(&mut self) -> SF_IF_EN_W {
        SF_IF_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn sf_if_fn_sel(&mut self) -> SF_IF_FN_SEL_W {
        SF_IF_FN_SEL_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sf_ahb2sif_stop(&mut self) -> SF_AHB2SIF_STOP_W {
        SF_AHB2SIF_STOP_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn sf_if_reg_wp(&mut self) -> SF_IF_REG_WP_W {
        SF_IF_REG_WP_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sf_if_reg_hold(&mut self) -> SF_IF_REG_HOLD_W {
        SF_IF_REG_HOLD_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn sf_if_0_ack_lat(&mut self) -> SF_IF_0_ACK_LAT_W {
        SF_IF_0_ACK_LAT_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_sr_int_set(&mut self) -> SF_IF_SR_INT_SET_W {
        SF_IF_SR_INT_SET_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_sr_int_en(&mut self) -> SF_IF_SR_INT_EN_W {
        SF_IF_SR_INT_EN_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sf_if_sr_pat(&mut self) -> SF_IF_SR_PAT_W {
        SF_IF_SR_PAT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sf_if_sr_pat_mask(&mut self) -> SF_IF_SR_PAT_MASK_W {
        SF_IF_SR_PAT_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_1](index.html) module"]
pub struct SF_CTRL_1_SPEC;
impl crate::RegisterSpec for SF_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_1::R](R) reader structure"]
impl crate::Readable for SF_CTRL_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_1::W](W) writer structure"]
impl crate::Writable for SF_CTRL_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_ctrl_1 to value 0xf360_0000"]
impl crate::Resettable for SF_CTRL_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf360_0000
    }
}
