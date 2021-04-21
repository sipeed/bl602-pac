#[doc = "Register `i2c_int_sts` reader"]
pub struct R(crate::R<I2C_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<I2C_INT_STS_SPEC>> for R {
    fn from(reader: crate::R<I2C_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2c_int_sts` writer"]
pub struct W(crate::W<I2C_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_INT_STS_SPEC>;
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
impl core::convert::From<crate::W<I2C_INT_STS_SPEC>> for W {
    fn from(writer: crate::W<I2C_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_i2c_fer_en` reader - "]
pub struct CR_I2C_FER_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2C_FER_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_FER_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_FER_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_fer_en` writer - "]
pub struct CR_I2C_FER_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_FER_EN_W<'a> {
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
#[doc = "Field `cr_i2c_arb_en` reader - "]
pub struct CR_I2C_ARB_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2C_ARB_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_ARB_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_ARB_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_arb_en` writer - "]
pub struct CR_I2C_ARB_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_ARB_EN_W<'a> {
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
#[doc = "Field `cr_i2c_nak_en` reader - "]
pub struct CR_I2C_NAK_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2C_NAK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_NAK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_NAK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_nak_en` writer - "]
pub struct CR_I2C_NAK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_NAK_EN_W<'a> {
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
#[doc = "Field `cr_i2c_rxf_en` reader - "]
pub struct CR_I2C_RXF_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2C_RXF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_RXF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_RXF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_rxf_en` writer - "]
pub struct CR_I2C_RXF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_RXF_EN_W<'a> {
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
#[doc = "Field `cr_i2c_txf_en` reader - "]
pub struct CR_I2C_TXF_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2C_TXF_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_TXF_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_TXF_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_txf_en` writer - "]
pub struct CR_I2C_TXF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_TXF_EN_W<'a> {
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
#[doc = "Field `cr_i2c_end_en` reader - "]
pub struct CR_I2C_END_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2C_END_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_END_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_END_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_end_en` writer - "]
pub struct CR_I2C_END_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_END_EN_W<'a> {
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
#[doc = "Field `rsvd_21` reader - "]
pub struct RSVD_21_R(crate::FieldReader<bool, bool>);
impl RSVD_21_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSVD_21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsvd_21` writer - "]
pub struct RSVD_21_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_21_W<'a> {
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
#[doc = "Field `cr_i2c_arb_clr` reader - "]
pub struct CR_I2C_ARB_CLR_R(crate::FieldReader<bool, bool>);
impl CR_I2C_ARB_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_ARB_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_ARB_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_arb_clr` writer - "]
pub struct CR_I2C_ARB_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_ARB_CLR_W<'a> {
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
#[doc = "Field `cr_i2c_nak_clr` reader - "]
pub struct CR_I2C_NAK_CLR_R(crate::FieldReader<bool, bool>);
impl CR_I2C_NAK_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_NAK_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_NAK_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_nak_clr` writer - "]
pub struct CR_I2C_NAK_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_NAK_CLR_W<'a> {
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
#[doc = "Field `rsvd_18` reader - "]
pub struct RSVD_18_R(crate::FieldReader<bool, bool>);
impl RSVD_18_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSVD_18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsvd_18` writer - "]
pub struct RSVD_18_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_18_W<'a> {
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
#[doc = "Field `rsvd_17` reader - "]
pub struct RSVD_17_R(crate::FieldReader<bool, bool>);
impl RSVD_17_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSVD_17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsvd_17` writer - "]
pub struct RSVD_17_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_17_W<'a> {
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
#[doc = "Field `cr_i2c_end_clr` reader - "]
pub struct CR_I2C_END_CLR_R(crate::FieldReader<bool, bool>);
impl CR_I2C_END_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_END_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_END_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_end_clr` writer - "]
pub struct CR_I2C_END_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_END_CLR_W<'a> {
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
#[doc = "Field `cr_i2c_fer_mask` reader - "]
pub struct CR_I2C_FER_MASK_R(crate::FieldReader<bool, bool>);
impl CR_I2C_FER_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_FER_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_FER_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_fer_mask` writer - "]
pub struct CR_I2C_FER_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_FER_MASK_W<'a> {
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
#[doc = "Field `cr_i2c_arb_mask` reader - "]
pub struct CR_I2C_ARB_MASK_R(crate::FieldReader<bool, bool>);
impl CR_I2C_ARB_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_ARB_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_ARB_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_arb_mask` writer - "]
pub struct CR_I2C_ARB_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_ARB_MASK_W<'a> {
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
#[doc = "Field `cr_i2c_nak_mask` reader - "]
pub struct CR_I2C_NAK_MASK_R(crate::FieldReader<bool, bool>);
impl CR_I2C_NAK_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_NAK_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_NAK_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_nak_mask` writer - "]
pub struct CR_I2C_NAK_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_NAK_MASK_W<'a> {
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
#[doc = "Field `cr_i2c_rxf_mask` reader - "]
pub struct CR_I2C_RXF_MASK_R(crate::FieldReader<bool, bool>);
impl CR_I2C_RXF_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_RXF_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_RXF_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_rxf_mask` writer - "]
pub struct CR_I2C_RXF_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_RXF_MASK_W<'a> {
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
#[doc = "Field `cr_i2c_txf_mask` reader - "]
pub struct CR_I2C_TXF_MASK_R(crate::FieldReader<bool, bool>);
impl CR_I2C_TXF_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_TXF_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_TXF_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_txf_mask` writer - "]
pub struct CR_I2C_TXF_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_TXF_MASK_W<'a> {
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
#[doc = "Field `cr_i2c_end_mask` reader - "]
pub struct CR_I2C_END_MASK_R(crate::FieldReader<bool, bool>);
impl CR_I2C_END_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_END_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_END_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_end_mask` writer - "]
pub struct CR_I2C_END_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_END_MASK_W<'a> {
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
#[doc = "Field `i2c_fer_int` reader - "]
pub struct I2C_FER_INT_R(crate::FieldReader<bool, bool>);
impl I2C_FER_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_FER_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_FER_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c_arb_int` reader - "]
pub struct I2C_ARB_INT_R(crate::FieldReader<bool, bool>);
impl I2C_ARB_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_ARB_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_ARB_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c_nak_int` reader - "]
pub struct I2C_NAK_INT_R(crate::FieldReader<bool, bool>);
impl I2C_NAK_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_NAK_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_NAK_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c_rxf_int` reader - "]
pub struct I2C_RXF_INT_R(crate::FieldReader<bool, bool>);
impl I2C_RXF_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_RXF_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_RXF_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c_txf_int` reader - "]
pub struct I2C_TXF_INT_R(crate::FieldReader<bool, bool>);
impl I2C_TXF_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_TXF_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_TXF_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2c_end_int` reader - "]
pub struct I2C_END_INT_R(crate::FieldReader<bool, bool>);
impl I2C_END_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_END_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2C_END_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_i2c_fer_en(&self) -> CR_I2C_FER_EN_R {
        CR_I2C_FER_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_i2c_arb_en(&self) -> CR_I2C_ARB_EN_R {
        CR_I2C_ARB_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_i2c_nak_en(&self) -> CR_I2C_NAK_EN_R {
        CR_I2C_NAK_EN_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_i2c_rxf_en(&self) -> CR_I2C_RXF_EN_R {
        CR_I2C_RXF_EN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_i2c_txf_en(&self) -> CR_I2C_TXF_EN_R {
        CR_I2C_TXF_EN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_i2c_end_en(&self) -> CR_I2C_END_EN_R {
        CR_I2C_END_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rsvd_21(&self) -> RSVD_21_R {
        RSVD_21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_i2c_arb_clr(&self) -> CR_I2C_ARB_CLR_R {
        CR_I2C_ARB_CLR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_i2c_nak_clr(&self) -> CR_I2C_NAK_CLR_R {
        CR_I2C_NAK_CLR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rsvd_18(&self) -> RSVD_18_R {
        RSVD_18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rsvd_17(&self) -> RSVD_17_R {
        RSVD_17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_i2c_end_clr(&self) -> CR_I2C_END_CLR_R {
        CR_I2C_END_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_i2c_fer_mask(&self) -> CR_I2C_FER_MASK_R {
        CR_I2C_FER_MASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_i2c_arb_mask(&self) -> CR_I2C_ARB_MASK_R {
        CR_I2C_ARB_MASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_i2c_nak_mask(&self) -> CR_I2C_NAK_MASK_R {
        CR_I2C_NAK_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_i2c_rxf_mask(&self) -> CR_I2C_RXF_MASK_R {
        CR_I2C_RXF_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_i2c_txf_mask(&self) -> CR_I2C_TXF_MASK_R {
        CR_I2C_TXF_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_i2c_end_mask(&self) -> CR_I2C_END_MASK_R {
        CR_I2C_END_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn i2c_fer_int(&self) -> I2C_FER_INT_R {
        I2C_FER_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn i2c_arb_int(&self) -> I2C_ARB_INT_R {
        I2C_ARB_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn i2c_nak_int(&self) -> I2C_NAK_INT_R {
        I2C_NAK_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn i2c_rxf_int(&self) -> I2C_RXF_INT_R {
        I2C_RXF_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn i2c_txf_int(&self) -> I2C_TXF_INT_R {
        I2C_TXF_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn i2c_end_int(&self) -> I2C_END_INT_R {
        I2C_END_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn cr_i2c_fer_en(&mut self) -> CR_I2C_FER_EN_W {
        CR_I2C_FER_EN_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn cr_i2c_arb_en(&mut self) -> CR_I2C_ARB_EN_W {
        CR_I2C_ARB_EN_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_i2c_nak_en(&mut self) -> CR_I2C_NAK_EN_W {
        CR_I2C_NAK_EN_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_i2c_rxf_en(&mut self) -> CR_I2C_RXF_EN_W {
        CR_I2C_RXF_EN_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_i2c_txf_en(&mut self) -> CR_I2C_TXF_EN_W {
        CR_I2C_TXF_EN_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_i2c_end_en(&mut self) -> CR_I2C_END_EN_W {
        CR_I2C_END_EN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rsvd_21(&mut self) -> RSVD_21_W {
        RSVD_21_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn cr_i2c_arb_clr(&mut self) -> CR_I2C_ARB_CLR_W {
        CR_I2C_ARB_CLR_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_i2c_nak_clr(&mut self) -> CR_I2C_NAK_CLR_W {
        CR_I2C_NAK_CLR_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rsvd_18(&mut self) -> RSVD_18_W {
        RSVD_18_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn rsvd_17(&mut self) -> RSVD_17_W {
        RSVD_17_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_i2c_end_clr(&mut self) -> CR_I2C_END_CLR_W {
        CR_I2C_END_CLR_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_i2c_fer_mask(&mut self) -> CR_I2C_FER_MASK_W {
        CR_I2C_FER_MASK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_i2c_arb_mask(&mut self) -> CR_I2C_ARB_MASK_W {
        CR_I2C_ARB_MASK_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_i2c_nak_mask(&mut self) -> CR_I2C_NAK_MASK_W {
        CR_I2C_NAK_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_i2c_rxf_mask(&mut self) -> CR_I2C_RXF_MASK_W {
        CR_I2C_RXF_MASK_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_i2c_txf_mask(&mut self) -> CR_I2C_TXF_MASK_W {
        CR_I2C_TXF_MASK_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_i2c_end_mask(&mut self) -> CR_I2C_END_MASK_W {
        CR_I2C_END_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c_int_sts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_int_sts](index.html) module"]
pub struct I2C_INT_STS_SPEC;
impl crate::RegisterSpec for I2C_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_int_sts::R](R) reader structure"]
impl crate::Readable for I2C_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_int_sts::W](W) writer structure"]
impl crate::Writable for I2C_INT_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2c_int_sts to value 0x3f00_3f00"]
impl crate::Resettable for I2C_INT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f00_3f00
    }
}
