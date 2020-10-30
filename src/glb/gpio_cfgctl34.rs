#[doc = "Register `GPIO_CFGCTL34` reader"]
pub struct R(crate::R<GPIO_CFGCTL34_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL34_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_CFGCTL34_SPEC>> for R {
    fn from(reader: crate::R<GPIO_CFGCTL34_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL34` writer"]
pub struct W(crate::W<GPIO_CFGCTL34_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL34_SPEC>;
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
impl core::convert::From<crate::W<GPIO_CFGCTL34_SPEC>> for W {
    fn from(writer: crate::W<GPIO_CFGCTL34_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_gpio_22_oe` reader - "]
pub struct REG_GPIO_22_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_22_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_22_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_oe` writer - "]
pub struct REG_GPIO_22_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_OE_W<'a> {
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
#[doc = "Field `reg_gpio_21_oe` reader - "]
pub struct REG_GPIO_21_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_21_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_21_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_21_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_21_oe` writer - "]
pub struct REG_GPIO_21_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_21_OE_W<'a> {
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
#[doc = "Field `reg_gpio_20_oe` reader - "]
pub struct REG_GPIO_20_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_20_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_20_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_20_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_20_oe` writer - "]
pub struct REG_GPIO_20_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_20_OE_W<'a> {
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
#[doc = "Field `reg_gpio_19_oe` reader - "]
pub struct REG_GPIO_19_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_19_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_19_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_19_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_19_oe` writer - "]
pub struct REG_GPIO_19_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_19_OE_W<'a> {
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
#[doc = "Field `reg_gpio_18_oe` reader - "]
pub struct REG_GPIO_18_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_18_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_18_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_18_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_18_oe` writer - "]
pub struct REG_GPIO_18_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_18_OE_W<'a> {
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
#[doc = "Field `reg_gpio_17_oe` reader - "]
pub struct REG_GPIO_17_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_17_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_17_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_17_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_17_oe` writer - "]
pub struct REG_GPIO_17_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_OE_W<'a> {
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
#[doc = "Field `reg_gpio_16_oe` reader - "]
pub struct REG_GPIO_16_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_16_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_16_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_16_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_16_oe` writer - "]
pub struct REG_GPIO_16_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_OE_W<'a> {
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
#[doc = "Field `reg_gpio_15_oe` reader - "]
pub struct REG_GPIO_15_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_15_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_15_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_15_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_15_oe` writer - "]
pub struct REG_GPIO_15_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_OE_W<'a> {
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
#[doc = "Field `reg_gpio_14_oe` reader - "]
pub struct REG_GPIO_14_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_14_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_14_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_14_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_14_oe` writer - "]
pub struct REG_GPIO_14_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_OE_W<'a> {
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
#[doc = "Field `reg_gpio_13_oe` reader - "]
pub struct REG_GPIO_13_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_13_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_13_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_13_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_13_oe` writer - "]
pub struct REG_GPIO_13_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_13_OE_W<'a> {
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
#[doc = "Field `reg_gpio_12_oe` reader - "]
pub struct REG_GPIO_12_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_12_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_12_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_12_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_12_oe` writer - "]
pub struct REG_GPIO_12_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_12_OE_W<'a> {
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
#[doc = "Field `reg_gpio_11_oe` reader - "]
pub struct REG_GPIO_11_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_11_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_11_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_11_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_11_oe` writer - "]
pub struct REG_GPIO_11_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_OE_W<'a> {
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
#[doc = "Field `reg_gpio_10_oe` reader - "]
pub struct REG_GPIO_10_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_10_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_10_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_10_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_10_oe` writer - "]
pub struct REG_GPIO_10_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_OE_W<'a> {
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
#[doc = "Field `reg_gpio_9_oe` reader - "]
pub struct REG_GPIO_9_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_9_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_9_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_9_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_9_oe` writer - "]
pub struct REG_GPIO_9_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_9_OE_W<'a> {
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
#[doc = "Field `reg_gpio_8_oe` reader - "]
pub struct REG_GPIO_8_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_8_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_8_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_8_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_8_oe` writer - "]
pub struct REG_GPIO_8_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_8_OE_W<'a> {
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
#[doc = "Field `reg_gpio_7_oe` reader - "]
pub struct REG_GPIO_7_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_7_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_7_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_7_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_7_oe` writer - "]
pub struct REG_GPIO_7_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_7_OE_W<'a> {
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
#[doc = "Field `reg_gpio_6_oe` reader - "]
pub struct REG_GPIO_6_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_6_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_6_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_6_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_6_oe` writer - "]
pub struct REG_GPIO_6_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_6_OE_W<'a> {
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
#[doc = "Field `reg_gpio_5_oe` reader - "]
pub struct REG_GPIO_5_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_5_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_5_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_5_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_5_oe` writer - "]
pub struct REG_GPIO_5_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_5_OE_W<'a> {
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
#[doc = "Field `reg_gpio_4_oe` reader - "]
pub struct REG_GPIO_4_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_4_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_4_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_4_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_4_oe` writer - "]
pub struct REG_GPIO_4_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_4_OE_W<'a> {
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
#[doc = "Field `reg_gpio_3_oe` reader - "]
pub struct REG_GPIO_3_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_3_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_3_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_3_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_3_oe` writer - "]
pub struct REG_GPIO_3_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_3_OE_W<'a> {
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
#[doc = "Field `reg_gpio_2_oe` reader - "]
pub struct REG_GPIO_2_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_2_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_2_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_2_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_2_oe` writer - "]
pub struct REG_GPIO_2_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_2_OE_W<'a> {
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
#[doc = "Field `reg_gpio_1_oe` reader - "]
pub struct REG_GPIO_1_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_1_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_1_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_1_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_1_oe` writer - "]
pub struct REG_GPIO_1_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_OE_W<'a> {
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
#[doc = "Field `reg_gpio_0_oe` reader - "]
pub struct REG_GPIO_0_OE_R(crate::FieldReader<bool, bool>);
impl REG_GPIO_0_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_0_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_GPIO_0_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_0_oe` writer - "]
pub struct REG_GPIO_0_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_OE_W<'a> {
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
    pub fn reg_gpio_22_oe(&self) -> REG_GPIO_22_OE_R {
        REG_GPIO_22_OE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_oe(&self) -> REG_GPIO_21_OE_R {
        REG_GPIO_21_OE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_oe(&self) -> REG_GPIO_20_OE_R {
        REG_GPIO_20_OE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_oe(&self) -> REG_GPIO_19_OE_R {
        REG_GPIO_19_OE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_oe(&self) -> REG_GPIO_18_OE_R {
        REG_GPIO_18_OE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_oe(&self) -> REG_GPIO_17_OE_R {
        REG_GPIO_17_OE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_oe(&self) -> REG_GPIO_16_OE_R {
        REG_GPIO_16_OE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_oe(&self) -> REG_GPIO_15_OE_R {
        REG_GPIO_15_OE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_oe(&self) -> REG_GPIO_14_OE_R {
        REG_GPIO_14_OE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_oe(&self) -> REG_GPIO_13_OE_R {
        REG_GPIO_13_OE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_oe(&self) -> REG_GPIO_12_OE_R {
        REG_GPIO_12_OE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_oe(&self) -> REG_GPIO_11_OE_R {
        REG_GPIO_11_OE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_oe(&self) -> REG_GPIO_10_OE_R {
        REG_GPIO_10_OE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_oe(&self) -> REG_GPIO_9_OE_R {
        REG_GPIO_9_OE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_oe(&self) -> REG_GPIO_8_OE_R {
        REG_GPIO_8_OE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_oe(&self) -> REG_GPIO_7_OE_R {
        REG_GPIO_7_OE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_oe(&self) -> REG_GPIO_6_OE_R {
        REG_GPIO_6_OE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_oe(&self) -> REG_GPIO_5_OE_R {
        REG_GPIO_5_OE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_oe(&self) -> REG_GPIO_4_OE_R {
        REG_GPIO_4_OE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_oe(&self) -> REG_GPIO_3_OE_R {
        REG_GPIO_3_OE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_oe(&self) -> REG_GPIO_2_OE_R {
        REG_GPIO_2_OE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_oe(&self) -> REG_GPIO_1_OE_R {
        REG_GPIO_1_OE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_oe(&self) -> REG_GPIO_0_OE_R {
        REG_GPIO_0_OE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_gpio_22_oe(&mut self) -> REG_GPIO_22_OE_W {
        REG_GPIO_22_OE_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_gpio_21_oe(&mut self) -> REG_GPIO_21_OE_W {
        REG_GPIO_21_OE_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_gpio_20_oe(&mut self) -> REG_GPIO_20_OE_W {
        REG_GPIO_20_OE_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_gpio_19_oe(&mut self) -> REG_GPIO_19_OE_W {
        REG_GPIO_19_OE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_gpio_18_oe(&mut self) -> REG_GPIO_18_OE_W {
        REG_GPIO_18_OE_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_gpio_17_oe(&mut self) -> REG_GPIO_17_OE_W {
        REG_GPIO_17_OE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_gpio_16_oe(&mut self) -> REG_GPIO_16_OE_W {
        REG_GPIO_16_OE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_gpio_15_oe(&mut self) -> REG_GPIO_15_OE_W {
        REG_GPIO_15_OE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn reg_gpio_14_oe(&mut self) -> REG_GPIO_14_OE_W {
        REG_GPIO_14_OE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_gpio_13_oe(&mut self) -> REG_GPIO_13_OE_W {
        REG_GPIO_13_OE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_gpio_12_oe(&mut self) -> REG_GPIO_12_OE_W {
        REG_GPIO_12_OE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_gpio_11_oe(&mut self) -> REG_GPIO_11_OE_W {
        REG_GPIO_11_OE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_gpio_10_oe(&mut self) -> REG_GPIO_10_OE_W {
        REG_GPIO_10_OE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_gpio_9_oe(&mut self) -> REG_GPIO_9_OE_W {
        REG_GPIO_9_OE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_gpio_8_oe(&mut self) -> REG_GPIO_8_OE_W {
        REG_GPIO_8_OE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_gpio_7_oe(&mut self) -> REG_GPIO_7_OE_W {
        REG_GPIO_7_OE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_gpio_6_oe(&mut self) -> REG_GPIO_6_OE_W {
        REG_GPIO_6_OE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_gpio_5_oe(&mut self) -> REG_GPIO_5_OE_W {
        REG_GPIO_5_OE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_gpio_4_oe(&mut self) -> REG_GPIO_4_OE_W {
        REG_GPIO_4_OE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_gpio_3_oe(&mut self) -> REG_GPIO_3_OE_W {
        REG_GPIO_3_OE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_gpio_2_oe(&mut self) -> REG_GPIO_2_OE_W {
        REG_GPIO_2_OE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_gpio_1_oe(&mut self) -> REG_GPIO_1_OE_W {
        REG_GPIO_1_OE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_gpio_0_oe(&mut self) -> REG_GPIO_0_OE_W {
        REG_GPIO_0_OE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO_CFGCTL34.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl34](index.html) module"]
pub struct GPIO_CFGCTL34_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl34::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL34_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl34::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL34_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_CFGCTL34 to value 0"]
impl crate::Resettable for GPIO_CFGCTL34_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
