#[doc = "Register `tzc_glb_ctrl_2` reader"]
pub struct R(crate::R<TZC_GLB_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_GLB_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TZC_GLB_CTRL_2_SPEC>> for R {
    fn from(reader: crate::R<TZC_GLB_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_glb_ctrl_2` writer"]
pub struct W(crate::W<TZC_GLB_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_GLB_CTRL_2_SPEC>;
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
impl core::convert::From<crate::W<TZC_GLB_CTRL_2_SPEC>> for W {
    fn from(writer: crate::W<TZC_GLB_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_glb_gpio_28_lock` reader - "]
pub struct TZC_GLB_GPIO_28_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_28_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_28_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_28_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_28_lock` writer - "]
pub struct TZC_GLB_GPIO_28_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_28_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `tzc_glb_gpio_27_lock` reader - "]
pub struct TZC_GLB_GPIO_27_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_27_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_27_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_27_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_27_lock` writer - "]
pub struct TZC_GLB_GPIO_27_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_27_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `tzc_glb_gpio_26_lock` reader - "]
pub struct TZC_GLB_GPIO_26_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_26_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_26_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_26_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_26_lock` writer - "]
pub struct TZC_GLB_GPIO_26_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_26_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `tzc_glb_gpio_25_lock` reader - "]
pub struct TZC_GLB_GPIO_25_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_25_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_25_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_25_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_25_lock` writer - "]
pub struct TZC_GLB_GPIO_25_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_25_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `tzc_glb_gpio_24_lock` reader - "]
pub struct TZC_GLB_GPIO_24_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_24_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_24_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_24_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_24_lock` writer - "]
pub struct TZC_GLB_GPIO_24_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_24_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `tzc_glb_gpio_23_lock` reader - "]
pub struct TZC_GLB_GPIO_23_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_23_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_23_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_23_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_23_lock` writer - "]
pub struct TZC_GLB_GPIO_23_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_23_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `tzc_glb_gpio_22_lock` reader - "]
pub struct TZC_GLB_GPIO_22_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_22_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_22_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_22_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_22_lock` writer - "]
pub struct TZC_GLB_GPIO_22_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_22_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_21_lock` reader - "]
pub struct TZC_GLB_GPIO_21_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_21_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_21_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_21_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_21_lock` writer - "]
pub struct TZC_GLB_GPIO_21_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_21_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_20_lock` reader - "]
pub struct TZC_GLB_GPIO_20_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_20_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_20_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_20_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_20_lock` writer - "]
pub struct TZC_GLB_GPIO_20_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_20_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_19_lock` reader - "]
pub struct TZC_GLB_GPIO_19_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_19_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_19_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_19_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_19_lock` writer - "]
pub struct TZC_GLB_GPIO_19_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_19_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_18_lock` reader - "]
pub struct TZC_GLB_GPIO_18_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_18_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_18_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_18_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_18_lock` writer - "]
pub struct TZC_GLB_GPIO_18_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_18_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_17_lock` reader - "]
pub struct TZC_GLB_GPIO_17_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_17_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_17_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_17_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_17_lock` writer - "]
pub struct TZC_GLB_GPIO_17_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_17_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_16_lock` reader - "]
pub struct TZC_GLB_GPIO_16_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_16_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_16_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_16_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_16_lock` writer - "]
pub struct TZC_GLB_GPIO_16_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_16_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_15_lock` reader - "]
pub struct TZC_GLB_GPIO_15_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_15_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_15_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_15_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_15_lock` writer - "]
pub struct TZC_GLB_GPIO_15_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_15_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_14_lock` reader - "]
pub struct TZC_GLB_GPIO_14_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_14_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_14_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_14_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_14_lock` writer - "]
pub struct TZC_GLB_GPIO_14_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_14_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_13_lock` reader - "]
pub struct TZC_GLB_GPIO_13_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_13_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_13_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_13_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_13_lock` writer - "]
pub struct TZC_GLB_GPIO_13_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_13_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_12_lock` reader - "]
pub struct TZC_GLB_GPIO_12_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_12_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_12_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_12_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_12_lock` writer - "]
pub struct TZC_GLB_GPIO_12_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_12_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_11_lock` reader - "]
pub struct TZC_GLB_GPIO_11_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_11_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_11_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_11_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_11_lock` writer - "]
pub struct TZC_GLB_GPIO_11_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_11_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_10_lock` reader - "]
pub struct TZC_GLB_GPIO_10_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_10_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_10_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_10_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_10_lock` writer - "]
pub struct TZC_GLB_GPIO_10_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_10_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_9_lock` reader - "]
pub struct TZC_GLB_GPIO_9_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_9_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_9_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_9_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_9_lock` writer - "]
pub struct TZC_GLB_GPIO_9_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_9_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_8_lock` reader - "]
pub struct TZC_GLB_GPIO_8_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_8_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_8_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_8_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_8_lock` writer - "]
pub struct TZC_GLB_GPIO_8_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_8_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_7_lock` reader - "]
pub struct TZC_GLB_GPIO_7_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_7_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_7_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_7_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_7_lock` writer - "]
pub struct TZC_GLB_GPIO_7_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_7_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_6_lock` reader - "]
pub struct TZC_GLB_GPIO_6_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_6_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_6_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_6_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_6_lock` writer - "]
pub struct TZC_GLB_GPIO_6_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_6_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_5_lock` reader - "]
pub struct TZC_GLB_GPIO_5_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_5_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_5_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_5_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_5_lock` writer - "]
pub struct TZC_GLB_GPIO_5_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_5_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_4_lock` reader - "]
pub struct TZC_GLB_GPIO_4_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_4_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_4_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_4_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_4_lock` writer - "]
pub struct TZC_GLB_GPIO_4_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_4_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_3_lock` reader - "]
pub struct TZC_GLB_GPIO_3_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_3_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_3_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_3_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_3_lock` writer - "]
pub struct TZC_GLB_GPIO_3_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_3_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_2_lock` reader - "]
pub struct TZC_GLB_GPIO_2_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_2_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_2_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_2_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_2_lock` writer - "]
pub struct TZC_GLB_GPIO_2_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_2_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_1_lock` reader - "]
pub struct TZC_GLB_GPIO_1_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_1_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_1_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_1_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_1_lock` writer - "]
pub struct TZC_GLB_GPIO_1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_1_LOCK_W<'a> {
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
#[doc = "Field `tzc_glb_gpio_0_lock` reader - "]
pub struct TZC_GLB_GPIO_0_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_GLB_GPIO_0_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_GLB_GPIO_0_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_GLB_GPIO_0_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_glb_gpio_0_lock` writer - "]
pub struct TZC_GLB_GPIO_0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_GLB_GPIO_0_LOCK_W<'a> {
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
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_gpio_28_lock(&self) -> TZC_GLB_GPIO_28_LOCK_R {
        TZC_GLB_GPIO_28_LOCK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_gpio_27_lock(&self) -> TZC_GLB_GPIO_27_LOCK_R {
        TZC_GLB_GPIO_27_LOCK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_gpio_26_lock(&self) -> TZC_GLB_GPIO_26_LOCK_R {
        TZC_GLB_GPIO_26_LOCK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_gpio_25_lock(&self) -> TZC_GLB_GPIO_25_LOCK_R {
        TZC_GLB_GPIO_25_LOCK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_gpio_24_lock(&self) -> TZC_GLB_GPIO_24_LOCK_R {
        TZC_GLB_GPIO_24_LOCK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_glb_gpio_23_lock(&self) -> TZC_GLB_GPIO_23_LOCK_R {
        TZC_GLB_GPIO_23_LOCK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_gpio_22_lock(&self) -> TZC_GLB_GPIO_22_LOCK_R {
        TZC_GLB_GPIO_22_LOCK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_gpio_21_lock(&self) -> TZC_GLB_GPIO_21_LOCK_R {
        TZC_GLB_GPIO_21_LOCK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_glb_gpio_20_lock(&self) -> TZC_GLB_GPIO_20_LOCK_R {
        TZC_GLB_GPIO_20_LOCK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_glb_gpio_19_lock(&self) -> TZC_GLB_GPIO_19_LOCK_R {
        TZC_GLB_GPIO_19_LOCK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_glb_gpio_18_lock(&self) -> TZC_GLB_GPIO_18_LOCK_R {
        TZC_GLB_GPIO_18_LOCK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_glb_gpio_17_lock(&self) -> TZC_GLB_GPIO_17_LOCK_R {
        TZC_GLB_GPIO_17_LOCK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_glb_gpio_16_lock(&self) -> TZC_GLB_GPIO_16_LOCK_R {
        TZC_GLB_GPIO_16_LOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_gpio_15_lock(&self) -> TZC_GLB_GPIO_15_LOCK_R {
        TZC_GLB_GPIO_15_LOCK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_gpio_14_lock(&self) -> TZC_GLB_GPIO_14_LOCK_R {
        TZC_GLB_GPIO_14_LOCK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_gpio_13_lock(&self) -> TZC_GLB_GPIO_13_LOCK_R {
        TZC_GLB_GPIO_13_LOCK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_gpio_12_lock(&self) -> TZC_GLB_GPIO_12_LOCK_R {
        TZC_GLB_GPIO_12_LOCK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_glb_gpio_11_lock(&self) -> TZC_GLB_GPIO_11_LOCK_R {
        TZC_GLB_GPIO_11_LOCK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_glb_gpio_10_lock(&self) -> TZC_GLB_GPIO_10_LOCK_R {
        TZC_GLB_GPIO_10_LOCK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_glb_gpio_9_lock(&self) -> TZC_GLB_GPIO_9_LOCK_R {
        TZC_GLB_GPIO_9_LOCK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_gpio_8_lock(&self) -> TZC_GLB_GPIO_8_LOCK_R {
        TZC_GLB_GPIO_8_LOCK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_glb_gpio_7_lock(&self) -> TZC_GLB_GPIO_7_LOCK_R {
        TZC_GLB_GPIO_7_LOCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_glb_gpio_6_lock(&self) -> TZC_GLB_GPIO_6_LOCK_R {
        TZC_GLB_GPIO_6_LOCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_gpio_5_lock(&self) -> TZC_GLB_GPIO_5_LOCK_R {
        TZC_GLB_GPIO_5_LOCK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_gpio_4_lock(&self) -> TZC_GLB_GPIO_4_LOCK_R {
        TZC_GLB_GPIO_4_LOCK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_gpio_3_lock(&self) -> TZC_GLB_GPIO_3_LOCK_R {
        TZC_GLB_GPIO_3_LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_gpio_2_lock(&self) -> TZC_GLB_GPIO_2_LOCK_R {
        TZC_GLB_GPIO_2_LOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_gpio_1_lock(&self) -> TZC_GLB_GPIO_1_LOCK_R {
        TZC_GLB_GPIO_1_LOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_gpio_0_lock(&self) -> TZC_GLB_GPIO_0_LOCK_R {
        TZC_GLB_GPIO_0_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn tzc_glb_gpio_28_lock(&mut self) -> TZC_GLB_GPIO_28_LOCK_W {
        TZC_GLB_GPIO_28_LOCK_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_glb_gpio_27_lock(&mut self) -> TZC_GLB_GPIO_27_LOCK_W {
        TZC_GLB_GPIO_27_LOCK_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_glb_gpio_26_lock(&mut self) -> TZC_GLB_GPIO_26_LOCK_W {
        TZC_GLB_GPIO_26_LOCK_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_glb_gpio_25_lock(&mut self) -> TZC_GLB_GPIO_25_LOCK_W {
        TZC_GLB_GPIO_25_LOCK_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_glb_gpio_24_lock(&mut self) -> TZC_GLB_GPIO_24_LOCK_W {
        TZC_GLB_GPIO_24_LOCK_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn tzc_glb_gpio_23_lock(&mut self) -> TZC_GLB_GPIO_23_LOCK_W {
        TZC_GLB_GPIO_23_LOCK_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tzc_glb_gpio_22_lock(&mut self) -> TZC_GLB_GPIO_22_LOCK_W {
        TZC_GLB_GPIO_22_LOCK_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn tzc_glb_gpio_21_lock(&mut self) -> TZC_GLB_GPIO_21_LOCK_W {
        TZC_GLB_GPIO_21_LOCK_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn tzc_glb_gpio_20_lock(&mut self) -> TZC_GLB_GPIO_20_LOCK_W {
        TZC_GLB_GPIO_20_LOCK_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_glb_gpio_19_lock(&mut self) -> TZC_GLB_GPIO_19_LOCK_W {
        TZC_GLB_GPIO_19_LOCK_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_glb_gpio_18_lock(&mut self) -> TZC_GLB_GPIO_18_LOCK_W {
        TZC_GLB_GPIO_18_LOCK_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_glb_gpio_17_lock(&mut self) -> TZC_GLB_GPIO_17_LOCK_W {
        TZC_GLB_GPIO_17_LOCK_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_glb_gpio_16_lock(&mut self) -> TZC_GLB_GPIO_16_LOCK_W {
        TZC_GLB_GPIO_16_LOCK_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn tzc_glb_gpio_15_lock(&mut self) -> TZC_GLB_GPIO_15_LOCK_W {
        TZC_GLB_GPIO_15_LOCK_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn tzc_glb_gpio_14_lock(&mut self) -> TZC_GLB_GPIO_14_LOCK_W {
        TZC_GLB_GPIO_14_LOCK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn tzc_glb_gpio_13_lock(&mut self) -> TZC_GLB_GPIO_13_LOCK_W {
        TZC_GLB_GPIO_13_LOCK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn tzc_glb_gpio_12_lock(&mut self) -> TZC_GLB_GPIO_12_LOCK_W {
        TZC_GLB_GPIO_12_LOCK_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_glb_gpio_11_lock(&mut self) -> TZC_GLB_GPIO_11_LOCK_W {
        TZC_GLB_GPIO_11_LOCK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_glb_gpio_10_lock(&mut self) -> TZC_GLB_GPIO_10_LOCK_W {
        TZC_GLB_GPIO_10_LOCK_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_glb_gpio_9_lock(&mut self) -> TZC_GLB_GPIO_9_LOCK_W {
        TZC_GLB_GPIO_9_LOCK_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_glb_gpio_8_lock(&mut self) -> TZC_GLB_GPIO_8_LOCK_W {
        TZC_GLB_GPIO_8_LOCK_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tzc_glb_gpio_7_lock(&mut self) -> TZC_GLB_GPIO_7_LOCK_W {
        TZC_GLB_GPIO_7_LOCK_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn tzc_glb_gpio_6_lock(&mut self) -> TZC_GLB_GPIO_6_LOCK_W {
        TZC_GLB_GPIO_6_LOCK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tzc_glb_gpio_5_lock(&mut self) -> TZC_GLB_GPIO_5_LOCK_W {
        TZC_GLB_GPIO_5_LOCK_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tzc_glb_gpio_4_lock(&mut self) -> TZC_GLB_GPIO_4_LOCK_W {
        TZC_GLB_GPIO_4_LOCK_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_glb_gpio_3_lock(&mut self) -> TZC_GLB_GPIO_3_LOCK_W {
        TZC_GLB_GPIO_3_LOCK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_glb_gpio_2_lock(&mut self) -> TZC_GLB_GPIO_2_LOCK_W {
        TZC_GLB_GPIO_2_LOCK_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_glb_gpio_1_lock(&mut self) -> TZC_GLB_GPIO_1_LOCK_W {
        TZC_GLB_GPIO_1_LOCK_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_glb_gpio_0_lock(&mut self) -> TZC_GLB_GPIO_0_LOCK_W {
        TZC_GLB_GPIO_0_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_glb_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_glb_ctrl_2](index.html) module"]
pub struct TZC_GLB_CTRL_2_SPEC;
impl crate::RegisterSpec for TZC_GLB_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_glb_ctrl_2::R](R) reader structure"]
impl crate::Readable for TZC_GLB_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_glb_ctrl_2::W](W) writer structure"]
impl crate::Writable for TZC_GLB_CTRL_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tzc_glb_ctrl_2 to value 0"]
impl crate::Resettable for TZC_GLB_CTRL_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
