#[doc = "Register `se_trng_0_ctrl_0` reader"]
pub struct R(crate::R<SE_TRNG_0_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_TRNG_0_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_TRNG_0_CTRL_0_SPEC>> for R {
    fn from(reader: crate::R<SE_TRNG_0_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_trng_0_ctrl_0` writer"]
pub struct W(crate::W<SE_TRNG_0_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_TRNG_0_CTRL_0_SPEC>;
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
impl core::convert::From<crate::W<SE_TRNG_0_CTRL_0_SPEC>> for W {
    fn from(writer: crate::W<SE_TRNG_0_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_trng_0_manual_en` reader - "]
pub struct SE_TRNG_0_MANUAL_EN_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_MANUAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_MANUAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_MANUAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_manual_en` writer - "]
pub struct SE_TRNG_0_MANUAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_MANUAL_EN_W<'a> {
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
#[doc = "Field `se_trng_0_manual_reseed` reader - "]
pub struct SE_TRNG_0_MANUAL_RESEED_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_MANUAL_RESEED_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_MANUAL_RESEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_MANUAL_RESEED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_manual_reseed` writer - "]
pub struct SE_TRNG_0_MANUAL_RESEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_MANUAL_RESEED_W<'a> {
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
#[doc = "Field `se_trng_0_manual_fun_sel` reader - "]
pub struct SE_TRNG_0_MANUAL_FUN_SEL_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_MANUAL_FUN_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_MANUAL_FUN_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_MANUAL_FUN_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_manual_fun_sel` writer - "]
pub struct SE_TRNG_0_MANUAL_FUN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_MANUAL_FUN_SEL_W<'a> {
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
#[doc = "Field `se_trng_0_int_mask` reader - "]
pub struct SE_TRNG_0_INT_MASK_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_INT_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_int_mask` writer - "]
pub struct SE_TRNG_0_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_INT_MASK_W<'a> {
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
#[doc = "Field `se_trng_0_int_set_1t` reader - "]
pub struct SE_TRNG_0_INT_SET_1T_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_INT_SET_1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_INT_SET_1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_INT_SET_1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_int_set_1t` writer - "]
pub struct SE_TRNG_0_INT_SET_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_INT_SET_1T_W<'a> {
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
#[doc = "Field `se_trng_0_int_clr_1t` reader - "]
pub struct SE_TRNG_0_INT_CLR_1T_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_INT_CLR_1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_INT_CLR_1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_INT_CLR_1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_int_clr_1t` writer - "]
pub struct SE_TRNG_0_INT_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_INT_CLR_1T_W<'a> {
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
#[doc = "Field `se_trng_0_int` reader - "]
pub struct SE_TRNG_0_INT_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_int` writer - "]
pub struct SE_TRNG_0_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_INT_W<'a> {
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
#[doc = "Field `se_trng_0_ht_error` reader - "]
pub struct SE_TRNG_0_HT_ERROR_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_HT_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_HT_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_HT_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_ht_error` writer - "]
pub struct SE_TRNG_0_HT_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_HT_ERROR_W<'a> {
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
#[doc = "Field `se_trng_0_dout_clr_1t` reader - "]
pub struct SE_TRNG_0_DOUT_CLR_1T_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_DOUT_CLR_1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_DOUT_CLR_1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_DOUT_CLR_1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_dout_clr_1t` writer - "]
pub struct SE_TRNG_0_DOUT_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_DOUT_CLR_1T_W<'a> {
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
#[doc = "Field `se_trng_0_en` reader - "]
pub struct SE_TRNG_0_EN_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_en` writer - "]
pub struct SE_TRNG_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_EN_W<'a> {
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
#[doc = "Field `se_trng_0_trig_1t` reader - "]
pub struct SE_TRNG_0_TRIG_1T_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_TRIG_1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_TRIG_1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_TRIG_1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_trig_1t` writer - "]
pub struct SE_TRNG_0_TRIG_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_TRIG_1T_W<'a> {
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
#[doc = "Field `se_trng_0_busy` reader - "]
pub struct SE_TRNG_0_BUSY_R(crate::FieldReader<bool, bool>);
impl SE_TRNG_0_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_TRNG_0_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_TRNG_0_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_trng_0_busy` writer - "]
pub struct SE_TRNG_0_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_TRNG_0_BUSY_W<'a> {
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
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_trng_0_manual_en(&self) -> SE_TRNG_0_MANUAL_EN_R {
        SE_TRNG_0_MANUAL_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_trng_0_manual_reseed(&self) -> SE_TRNG_0_MANUAL_RESEED_R {
        SE_TRNG_0_MANUAL_RESEED_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_trng_0_manual_fun_sel(&self) -> SE_TRNG_0_MANUAL_FUN_SEL_R {
        SE_TRNG_0_MANUAL_FUN_SEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_trng_0_int_mask(&self) -> SE_TRNG_0_INT_MASK_R {
        SE_TRNG_0_INT_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_0_int_set_1t(&self) -> SE_TRNG_0_INT_SET_1T_R {
        SE_TRNG_0_INT_SET_1T_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_0_int_clr_1t(&self) -> SE_TRNG_0_INT_CLR_1T_R {
        SE_TRNG_0_INT_CLR_1T_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_0_int(&self) -> SE_TRNG_0_INT_R {
        SE_TRNG_0_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_trng_0_ht_error(&self) -> SE_TRNG_0_HT_ERROR_R {
        SE_TRNG_0_HT_ERROR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_dout_clr_1t(&self) -> SE_TRNG_0_DOUT_CLR_1T_R {
        SE_TRNG_0_DOUT_CLR_1T_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_en(&self) -> SE_TRNG_0_EN_R {
        SE_TRNG_0_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_trig_1t(&self) -> SE_TRNG_0_TRIG_1T_R {
        SE_TRNG_0_TRIG_1T_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_busy(&self) -> SE_TRNG_0_BUSY_R {
        SE_TRNG_0_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_trng_0_manual_en(&mut self) -> SE_TRNG_0_MANUAL_EN_W {
        SE_TRNG_0_MANUAL_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn se_trng_0_manual_reseed(&mut self) -> SE_TRNG_0_MANUAL_RESEED_W {
        SE_TRNG_0_MANUAL_RESEED_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_trng_0_manual_fun_sel(&mut self) -> SE_TRNG_0_MANUAL_FUN_SEL_W {
        SE_TRNG_0_MANUAL_FUN_SEL_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_trng_0_int_mask(&mut self) -> SE_TRNG_0_INT_MASK_W {
        SE_TRNG_0_INT_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_trng_0_int_set_1t(&mut self) -> SE_TRNG_0_INT_SET_1T_W {
        SE_TRNG_0_INT_SET_1T_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_trng_0_int_clr_1t(&mut self) -> SE_TRNG_0_INT_CLR_1T_W {
        SE_TRNG_0_INT_CLR_1T_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_trng_0_int(&mut self) -> SE_TRNG_0_INT_W {
        SE_TRNG_0_INT_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn se_trng_0_ht_error(&mut self) -> SE_TRNG_0_HT_ERROR_W {
        SE_TRNG_0_HT_ERROR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_trng_0_dout_clr_1t(&mut self) -> SE_TRNG_0_DOUT_CLR_1T_W {
        SE_TRNG_0_DOUT_CLR_1T_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_trng_0_en(&mut self) -> SE_TRNG_0_EN_W {
        SE_TRNG_0_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_trng_0_trig_1t(&mut self) -> SE_TRNG_0_TRIG_1T_W {
        SE_TRNG_0_TRIG_1T_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_trng_0_busy(&mut self) -> SE_TRNG_0_BUSY_W {
        SE_TRNG_0_BUSY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_trng_0_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_trng_0_ctrl_0](index.html) module"]
pub struct SE_TRNG_0_CTRL_0_SPEC;
impl crate::RegisterSpec for SE_TRNG_0_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_trng_0_ctrl_0::R](R) reader structure"]
impl crate::Readable for SE_TRNG_0_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_trng_0_ctrl_0::W](W) writer structure"]
impl crate::Writable for SE_TRNG_0_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_trng_0_ctrl_0 to value 0"]
impl crate::Resettable for SE_TRNG_0_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
