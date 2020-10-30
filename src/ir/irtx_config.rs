#[doc = "Register `irtx_config` reader"]
pub struct R(crate::R<IRTX_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRTX_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IRTX_CONFIG_SPEC>> for R {
    fn from(reader: crate::R<IRTX_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irtx_config` writer"]
pub struct W(crate::W<IRTX_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRTX_CONFIG_SPEC>;
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
impl core::convert::From<crate::W<IRTX_CONFIG_SPEC>> for W {
    fn from(writer: crate::W<IRTX_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irtx_data_num` reader - "]
pub struct CR_IRTX_DATA_NUM_R(crate::FieldReader<u8, u8>);
impl CR_IRTX_DATA_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRTX_DATA_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_DATA_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_data_num` writer - "]
pub struct CR_IRTX_DATA_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_DATA_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Field `cr_irtx_tail_hl_inv` reader - "]
pub struct CR_IRTX_TAIL_HL_INV_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_TAIL_HL_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_TAIL_HL_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_TAIL_HL_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_tail_hl_inv` writer - "]
pub struct CR_IRTX_TAIL_HL_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_TAIL_HL_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `cr_irtx_tail_en` reader - "]
pub struct CR_IRTX_TAIL_EN_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_TAIL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_TAIL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_TAIL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_tail_en` writer - "]
pub struct CR_IRTX_TAIL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_TAIL_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `cr_irtx_head_hl_inv` reader - "]
pub struct CR_IRTX_HEAD_HL_INV_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_HEAD_HL_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_HEAD_HL_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_HEAD_HL_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_head_hl_inv` writer - "]
pub struct CR_IRTX_HEAD_HL_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_HEAD_HL_INV_W<'a> {
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
#[doc = "Field `cr_irtx_head_en` reader - "]
pub struct CR_IRTX_HEAD_EN_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_HEAD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_HEAD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_HEAD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_head_en` writer - "]
pub struct CR_IRTX_HEAD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_HEAD_EN_W<'a> {
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
#[doc = "Field `cr_irtx_logic1_hl_inv` reader - "]
pub struct CR_IRTX_LOGIC1_HL_INV_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_LOGIC1_HL_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_LOGIC1_HL_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_LOGIC1_HL_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_logic1_hl_inv` writer - "]
pub struct CR_IRTX_LOGIC1_HL_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC1_HL_INV_W<'a> {
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
#[doc = "Field `cr_irtx_logic0_hl_inv` reader - "]
pub struct CR_IRTX_LOGIC0_HL_INV_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_LOGIC0_HL_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_LOGIC0_HL_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_LOGIC0_HL_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_logic0_hl_inv` writer - "]
pub struct CR_IRTX_LOGIC0_HL_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC0_HL_INV_W<'a> {
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
#[doc = "Field `cr_irtx_data_en` reader - "]
pub struct CR_IRTX_DATA_EN_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_DATA_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_DATA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_DATA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_data_en` writer - "]
pub struct CR_IRTX_DATA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_DATA_EN_W<'a> {
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
#[doc = "Field `cr_irtx_swm_en` reader - "]
pub struct CR_IRTX_SWM_EN_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_SWM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_SWM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_SWM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_swm_en` writer - "]
pub struct CR_IRTX_SWM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_SWM_EN_W<'a> {
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
#[doc = "Field `cr_irtx_mod_en` reader - "]
pub struct CR_IRTX_MOD_EN_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_MOD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_MOD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_MOD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_mod_en` writer - "]
pub struct CR_IRTX_MOD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_MOD_EN_W<'a> {
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
#[doc = "Field `cr_irtx_out_inv` reader - "]
pub struct CR_IRTX_OUT_INV_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_OUT_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_OUT_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_OUT_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_out_inv` writer - "]
pub struct CR_IRTX_OUT_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_OUT_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `cr_irtx_en` reader - "]
pub struct CR_IRTX_EN_R(crate::FieldReader<bool, bool>);
impl CR_IRTX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRTX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_en` writer - "]
pub struct CR_IRTX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn cr_irtx_data_num(&self) -> CR_IRTX_DATA_NUM_R {
        CR_IRTX_DATA_NUM_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_irtx_tail_hl_inv(&self) -> CR_IRTX_TAIL_HL_INV_R {
        CR_IRTX_TAIL_HL_INV_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_irtx_tail_en(&self) -> CR_IRTX_TAIL_EN_R {
        CR_IRTX_TAIL_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_irtx_head_hl_inv(&self) -> CR_IRTX_HEAD_HL_INV_R {
        CR_IRTX_HEAD_HL_INV_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_head_en(&self) -> CR_IRTX_HEAD_EN_R {
        CR_IRTX_HEAD_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_irtx_logic1_hl_inv(&self) -> CR_IRTX_LOGIC1_HL_INV_R {
        CR_IRTX_LOGIC1_HL_INV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_irtx_logic0_hl_inv(&self) -> CR_IRTX_LOGIC0_HL_INV_R {
        CR_IRTX_LOGIC0_HL_INV_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irtx_data_en(&self) -> CR_IRTX_DATA_EN_R {
        CR_IRTX_DATA_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_irtx_swm_en(&self) -> CR_IRTX_SWM_EN_R {
        CR_IRTX_SWM_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_irtx_mod_en(&self) -> CR_IRTX_MOD_EN_R {
        CR_IRTX_MOD_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irtx_out_inv(&self) -> CR_IRTX_OUT_INV_R {
        CR_IRTX_OUT_INV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irtx_en(&self) -> CR_IRTX_EN_R {
        CR_IRTX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn cr_irtx_data_num(&mut self) -> CR_IRTX_DATA_NUM_W {
        CR_IRTX_DATA_NUM_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_irtx_tail_hl_inv(&mut self) -> CR_IRTX_TAIL_HL_INV_W {
        CR_IRTX_TAIL_HL_INV_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_irtx_tail_en(&mut self) -> CR_IRTX_TAIL_EN_W {
        CR_IRTX_TAIL_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_irtx_head_hl_inv(&mut self) -> CR_IRTX_HEAD_HL_INV_W {
        CR_IRTX_HEAD_HL_INV_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_irtx_head_en(&mut self) -> CR_IRTX_HEAD_EN_W {
        CR_IRTX_HEAD_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_irtx_logic1_hl_inv(&mut self) -> CR_IRTX_LOGIC1_HL_INV_W {
        CR_IRTX_LOGIC1_HL_INV_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_irtx_logic0_hl_inv(&mut self) -> CR_IRTX_LOGIC0_HL_INV_W {
        CR_IRTX_LOGIC0_HL_INV_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irtx_data_en(&mut self) -> CR_IRTX_DATA_EN_W {
        CR_IRTX_DATA_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_irtx_swm_en(&mut self) -> CR_IRTX_SWM_EN_W {
        CR_IRTX_SWM_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_irtx_mod_en(&mut self) -> CR_IRTX_MOD_EN_W {
        CR_IRTX_MOD_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irtx_out_inv(&mut self) -> CR_IRTX_OUT_INV_W {
        CR_IRTX_OUT_INV_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irtx_en(&mut self) -> CR_IRTX_EN_W {
        CR_IRTX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irtx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_config](index.html) module"]
pub struct IRTX_CONFIG_SPEC;
impl crate::RegisterSpec for IRTX_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irtx_config::R](R) reader structure"]
impl crate::Readable for IRTX_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irtx_config::W](W) writer structure"]
impl crate::Writable for IRTX_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irtx_config to value 0"]
impl crate::Resettable for IRTX_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
