#[doc = "Register `GPIO_CFGCTL30` reader"]
pub struct R(crate::R<GPIO_CFGCTL30_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL30_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_CFGCTL30_SPEC>> for R {
    fn from(reader: crate::R<GPIO_CFGCTL30_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL30` writer"]
pub struct W(crate::W<GPIO_CFGCTL30_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL30_SPEC>;
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
impl core::convert::From<crate::W<GPIO_CFGCTL30_SPEC>> for W {
    fn from(writer: crate::W<GPIO_CFGCTL30_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_22_i` reader - "]
pub struct REG_GPIO_22_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_22_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_22_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_i` writer - "]
pub struct REG_GPIO_22_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `reg_gpio_21_i` reader - "]
pub struct REG_GPIO_21_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_21_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_21_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_21_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_21_i` writer - "]
pub struct REG_GPIO_21_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_21_I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `reg_gpio_20_i` reader - "]
pub struct REG_GPIO_20_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_20_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_20_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_20_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_20_i` writer - "]
pub struct REG_GPIO_20_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_20_I_W<'a> {
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
#[doc = "Field `reg_gpio_19_i` reader - "]
pub struct REG_GPIO_19_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_19_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_19_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_19_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_19_i` writer - "]
pub struct REG_GPIO_19_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_19_I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `reg_gpio_18_i` reader - "]
pub struct REG_GPIO_18_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_18_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_18_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_18_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_18_i` writer - "]
pub struct REG_GPIO_18_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_18_I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `reg_gpio_17_i` reader - "]
pub struct REG_GPIO_17_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_17_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_17_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_17_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_17_i` writer - "]
pub struct REG_GPIO_17_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `reg_gpio_16_i` reader - "]
pub struct REG_GPIO_16_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_16_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_16_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_16_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_16_i` writer - "]
pub struct REG_GPIO_16_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_I_W<'a> {
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
#[doc = "Field `reg_gpio_15_i` reader - "]
pub struct REG_GPIO_15_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_15_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_15_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_15_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_15_i` writer - "]
pub struct REG_GPIO_15_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_I_W<'a> {
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
#[doc = "Field `reg_gpio_14_i` reader - "]
pub struct REG_GPIO_14_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_14_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_14_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_14_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_14_i` writer - "]
pub struct REG_GPIO_14_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_I_W<'a> {
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
#[doc = "Field `reg_gpio_13_i` reader - "]
pub struct REG_GPIO_13_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_13_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_13_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_13_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_13_i` writer - "]
pub struct REG_GPIO_13_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_13_I_W<'a> {
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
#[doc = "Field `reg_gpio_12_i` reader - "]
pub struct REG_GPIO_12_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_12_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_12_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_12_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_12_i` writer - "]
pub struct REG_GPIO_12_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_12_I_W<'a> {
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
#[doc = "Field `reg_gpio_11_i` reader - "]
pub struct REG_GPIO_11_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_11_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_11_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_11_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_11_i` writer - "]
pub struct REG_GPIO_11_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_I_W<'a> {
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
#[doc = "Field `reg_gpio_10_i` reader - "]
pub struct REG_GPIO_10_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_10_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_10_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_10_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_10_i` writer - "]
pub struct REG_GPIO_10_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_I_W<'a> {
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
#[doc = "Field `reg_gpio_9_i` reader - "]
pub struct REG_GPIO_9_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_9_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_9_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_9_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_9_i` writer - "]
pub struct REG_GPIO_9_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_9_I_W<'a> {
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
#[doc = "Field `reg_gpio_8_i` reader - "]
pub struct REG_GPIO_8_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_8_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_8_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_8_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_8_i` writer - "]
pub struct REG_GPIO_8_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_8_I_W<'a> {
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
#[doc = "Field `reg_gpio_7_i` reader - "]
pub struct REG_GPIO_7_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_7_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_7_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_7_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_7_i` writer - "]
pub struct REG_GPIO_7_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_7_I_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `reg_gpio_6_i` reader - "]
pub struct REG_GPIO_6_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_6_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_6_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_6_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_6_i` writer - "]
pub struct REG_GPIO_6_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_6_I_W<'a> {
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
#[doc = "Field `reg_gpio_5_i` reader - "]
pub struct REG_GPIO_5_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_5_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_5_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_5_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_5_i` writer - "]
pub struct REG_GPIO_5_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_5_I_W<'a> {
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
#[doc = "Field `reg_gpio_4_i` reader - "]
pub struct REG_GPIO_4_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_4_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_4_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_4_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_4_i` writer - "]
pub struct REG_GPIO_4_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_4_I_W<'a> {
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
#[doc = "Field `reg_gpio_3_i` reader - "]
pub struct REG_GPIO_3_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_3_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_3_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_3_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_3_i` writer - "]
pub struct REG_GPIO_3_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_3_I_W<'a> {
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
#[doc = "Field `reg_gpio_2_i` reader - "]
pub struct REG_GPIO_2_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_2_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_2_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_2_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_2_i` writer - "]
pub struct REG_GPIO_2_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_2_I_W<'a> {
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
#[doc = "Field `reg_gpio_1_i` reader - "]
pub struct REG_GPIO_1_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_1_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_1_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_1_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_1_i` writer - "]
pub struct REG_GPIO_1_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_I_W<'a> {
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
#[doc = "Field `reg_gpio_0_i` reader - "]
pub struct REG_GPIO_0_I_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_0_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_0_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_0_I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_0_i` writer - "]
pub struct REG_GPIO_0_I_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_I_W<'a> {
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
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_i(&self) -> REG_GPIO_22_I_R {
        REG_GPIO_22_I_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_i(&self) -> REG_GPIO_21_I_R {
        REG_GPIO_21_I_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_i(&self) -> REG_GPIO_20_I_R {
        REG_GPIO_20_I_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_i(&self) -> REG_GPIO_19_I_R {
        REG_GPIO_19_I_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_i(&self) -> REG_GPIO_18_I_R {
        REG_GPIO_18_I_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_i(&self) -> REG_GPIO_17_I_R {
        REG_GPIO_17_I_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_i(&self) -> REG_GPIO_16_I_R {
        REG_GPIO_16_I_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_i(&self) -> REG_GPIO_15_I_R {
        REG_GPIO_15_I_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_i(&self) -> REG_GPIO_14_I_R {
        REG_GPIO_14_I_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_i(&self) -> REG_GPIO_13_I_R {
        REG_GPIO_13_I_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_i(&self) -> REG_GPIO_12_I_R {
        REG_GPIO_12_I_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_i(&self) -> REG_GPIO_11_I_R {
        REG_GPIO_11_I_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_i(&self) -> REG_GPIO_10_I_R {
        REG_GPIO_10_I_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_i(&self) -> REG_GPIO_9_I_R {
        REG_GPIO_9_I_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_i(&self) -> REG_GPIO_8_I_R {
        REG_GPIO_8_I_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_i(&self) -> REG_GPIO_7_I_R {
        REG_GPIO_7_I_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_i(&self) -> REG_GPIO_6_I_R {
        REG_GPIO_6_I_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_i(&self) -> REG_GPIO_5_I_R {
        REG_GPIO_5_I_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_i(&self) -> REG_GPIO_4_I_R {
        REG_GPIO_4_I_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_i(&self) -> REG_GPIO_3_I_R {
        REG_GPIO_3_I_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_i(&self) -> REG_GPIO_2_I_R {
        REG_GPIO_2_I_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_i(&self) -> REG_GPIO_1_I_R {
        REG_GPIO_1_I_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_i(&self) -> REG_GPIO_0_I_R {
        REG_GPIO_0_I_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_i(&mut self) -> REG_GPIO_22_I_W {
        REG_GPIO_22_I_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_i(&mut self) -> REG_GPIO_21_I_W {
        REG_GPIO_21_I_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_i(&mut self) -> REG_GPIO_20_I_W {
        REG_GPIO_20_I_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_i(&mut self) -> REG_GPIO_19_I_W {
        REG_GPIO_19_I_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_i(&mut self) -> REG_GPIO_18_I_W {
        REG_GPIO_18_I_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_i(&mut self) -> REG_GPIO_17_I_W {
        REG_GPIO_17_I_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_i(&mut self) -> REG_GPIO_16_I_W {
        REG_GPIO_16_I_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_i(&mut self) -> REG_GPIO_15_I_W {
        REG_GPIO_15_I_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_i(&mut self) -> REG_GPIO_14_I_W {
        REG_GPIO_14_I_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_i(&mut self) -> REG_GPIO_13_I_W {
        REG_GPIO_13_I_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_i(&mut self) -> REG_GPIO_12_I_W {
        REG_GPIO_12_I_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_i(&mut self) -> REG_GPIO_11_I_W {
        REG_GPIO_11_I_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_i(&mut self) -> REG_GPIO_10_I_W {
        REG_GPIO_10_I_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_i(&mut self) -> REG_GPIO_9_I_W {
        REG_GPIO_9_I_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_i(&mut self) -> REG_GPIO_8_I_W {
        REG_GPIO_8_I_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_i(&mut self) -> REG_GPIO_7_I_W {
        REG_GPIO_7_I_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_i(&mut self) -> REG_GPIO_6_I_W {
        REG_GPIO_6_I_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_i(&mut self) -> REG_GPIO_5_I_W {
        REG_GPIO_5_I_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_i(&mut self) -> REG_GPIO_4_I_W {
        REG_GPIO_4_I_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_i(&mut self) -> REG_GPIO_3_I_W {
        REG_GPIO_3_I_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_i(&mut self) -> REG_GPIO_2_I_W {
        REG_GPIO_2_I_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_i(&mut self) -> REG_GPIO_1_I_W {
        REG_GPIO_1_I_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_i(&mut self) -> REG_GPIO_0_I_W {
        REG_GPIO_0_I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_CFGCTL30.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl30](index.html) module"]
pub struct GPIO_CFGCTL30_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl30::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL30_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl30::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL30_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_CFGCTL30 to value 0"]
impl crate::Resettable for GPIO_CFGCTL30_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
