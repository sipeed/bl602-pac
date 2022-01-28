#[doc = "Register `GPIO_INT_MASK1` reader"]
pub struct R(crate::R<GPIO_INT_MASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_MASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_MASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_MASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_MASK1` writer"]
pub struct W(crate::W<GPIO_INT_MASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_MASK1_SPEC>;
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
impl From<crate::W<GPIO_INT_MASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_MASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Mask register for GPIO0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_0_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_0_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_0_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_mask` reader - Mask register for GPIO0."]
pub struct REG_GPIO_0_MASK_R(crate::FieldReader<bool, REG_GPIO_0_MASK_A>);
impl REG_GPIO_0_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_0_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_0_MASK_A {
        match self.bits {
            false => REG_GPIO_0_MASK_A::UNMASKED,
            true => REG_GPIO_0_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_0_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_0_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_0_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_0_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_0_mask` writer - Mask register for GPIO0."]
pub struct REG_GPIO_0_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_0_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_0_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_0_MASK_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Mask register for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_1_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_1_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_1_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_mask` reader - Mask register for GPIO1."]
pub struct REG_GPIO_1_MASK_R(crate::FieldReader<bool, REG_GPIO_1_MASK_A>);
impl REG_GPIO_1_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_1_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_1_MASK_A {
        match self.bits {
            false => REG_GPIO_1_MASK_A::UNMASKED,
            true => REG_GPIO_1_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_1_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_1_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_1_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_1_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_1_mask` writer - Mask register for GPIO1."]
pub struct REG_GPIO_1_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_1_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_1_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_1_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_2_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_2_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_2_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_mask` reader - Mask register for GPIO2."]
pub struct REG_GPIO_2_MASK_R(crate::FieldReader<bool, REG_GPIO_2_MASK_A>);
impl REG_GPIO_2_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_2_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_2_MASK_A {
        match self.bits {
            false => REG_GPIO_2_MASK_A::UNMASKED,
            true => REG_GPIO_2_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_2_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_2_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_2_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_2_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_2_mask` writer - Mask register for GPIO2."]
pub struct REG_GPIO_2_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_2_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_2_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_2_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_2_MASK_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Mask register for GPIO3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_3_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_3_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_3_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_mask` reader - Mask register for GPIO3."]
pub struct REG_GPIO_3_MASK_R(crate::FieldReader<bool, REG_GPIO_3_MASK_A>);
impl REG_GPIO_3_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_3_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_3_MASK_A {
        match self.bits {
            false => REG_GPIO_3_MASK_A::UNMASKED,
            true => REG_GPIO_3_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_3_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_3_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_3_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_3_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_3_mask` writer - Mask register for GPIO3."]
pub struct REG_GPIO_3_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_3_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_3_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_3_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_3_MASK_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Mask register for GPIO4.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_4_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_4_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_4_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_mask` reader - Mask register for GPIO4."]
pub struct REG_GPIO_4_MASK_R(crate::FieldReader<bool, REG_GPIO_4_MASK_A>);
impl REG_GPIO_4_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_4_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_4_MASK_A {
        match self.bits {
            false => REG_GPIO_4_MASK_A::UNMASKED,
            true => REG_GPIO_4_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_4_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_4_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_4_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_4_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_4_mask` writer - Mask register for GPIO4."]
pub struct REG_GPIO_4_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_4_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_4_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_4_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_4_MASK_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Mask register for GPIO5.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_5_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_5_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_5_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_mask` reader - Mask register for GPIO5."]
pub struct REG_GPIO_5_MASK_R(crate::FieldReader<bool, REG_GPIO_5_MASK_A>);
impl REG_GPIO_5_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_5_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_5_MASK_A {
        match self.bits {
            false => REG_GPIO_5_MASK_A::UNMASKED,
            true => REG_GPIO_5_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_5_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_5_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_5_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_5_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_5_mask` writer - Mask register for GPIO5."]
pub struct REG_GPIO_5_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_5_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_5_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_5_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_5_MASK_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Mask register for GPIO6.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_6_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_6_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_6_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_mask` reader - Mask register for GPIO6."]
pub struct REG_GPIO_6_MASK_R(crate::FieldReader<bool, REG_GPIO_6_MASK_A>);
impl REG_GPIO_6_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_6_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_6_MASK_A {
        match self.bits {
            false => REG_GPIO_6_MASK_A::UNMASKED,
            true => REG_GPIO_6_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_6_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_6_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_6_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_6_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_6_mask` writer - Mask register for GPIO6."]
pub struct REG_GPIO_6_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_6_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_6_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_6_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_6_MASK_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Mask register for GPIO7.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_7_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_7_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_7_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_mask` reader - Mask register for GPIO7."]
pub struct REG_GPIO_7_MASK_R(crate::FieldReader<bool, REG_GPIO_7_MASK_A>);
impl REG_GPIO_7_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_7_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_7_MASK_A {
        match self.bits {
            false => REG_GPIO_7_MASK_A::UNMASKED,
            true => REG_GPIO_7_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_7_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_7_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_7_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_7_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_7_mask` writer - Mask register for GPIO7."]
pub struct REG_GPIO_7_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_7_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_7_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_7_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_7_MASK_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Mask register for GPIO8.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_8_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_8_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_8_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_mask` reader - Mask register for GPIO8."]
pub struct REG_GPIO_8_MASK_R(crate::FieldReader<bool, REG_GPIO_8_MASK_A>);
impl REG_GPIO_8_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_8_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_8_MASK_A {
        match self.bits {
            false => REG_GPIO_8_MASK_A::UNMASKED,
            true => REG_GPIO_8_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_8_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_8_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_8_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_8_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_8_mask` writer - Mask register for GPIO8."]
pub struct REG_GPIO_8_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_8_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_8_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_8_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_8_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO9.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_9_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_9_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_9_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_mask` reader - Mask register for GPIO9."]
pub struct REG_GPIO_9_MASK_R(crate::FieldReader<bool, REG_GPIO_9_MASK_A>);
impl REG_GPIO_9_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_9_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_9_MASK_A {
        match self.bits {
            false => REG_GPIO_9_MASK_A::UNMASKED,
            true => REG_GPIO_9_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_9_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_9_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_9_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_9_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_9_mask` writer - Mask register for GPIO9."]
pub struct REG_GPIO_9_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_9_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_9_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_9_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_9_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO10.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_10_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_10_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_10_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_mask` reader - Mask register for GPIO10."]
pub struct REG_GPIO_10_MASK_R(crate::FieldReader<bool, REG_GPIO_10_MASK_A>);
impl REG_GPIO_10_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_10_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_10_MASK_A {
        match self.bits {
            false => REG_GPIO_10_MASK_A::UNMASKED,
            true => REG_GPIO_10_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_10_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_10_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_10_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_10_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_10_mask` writer - Mask register for GPIO10."]
pub struct REG_GPIO_10_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_10_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_10_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_10_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO11.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_11_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_11_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_11_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_mask` reader - Mask register for GPIO11."]
pub struct REG_GPIO_11_MASK_R(crate::FieldReader<bool, REG_GPIO_11_MASK_A>);
impl REG_GPIO_11_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_11_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_11_MASK_A {
        match self.bits {
            false => REG_GPIO_11_MASK_A::UNMASKED,
            true => REG_GPIO_11_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_11_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_11_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_11_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_11_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_11_mask` writer - Mask register for GPIO11."]
pub struct REG_GPIO_11_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_11_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_11_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_11_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO12.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_12_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_12_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_12_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_mask` reader - Mask register for GPIO12."]
pub struct REG_GPIO_12_MASK_R(crate::FieldReader<bool, REG_GPIO_12_MASK_A>);
impl REG_GPIO_12_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_12_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_12_MASK_A {
        match self.bits {
            false => REG_GPIO_12_MASK_A::UNMASKED,
            true => REG_GPIO_12_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_12_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_12_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_12_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_12_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_12_mask` writer - Mask register for GPIO12."]
pub struct REG_GPIO_12_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_12_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_12_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_12_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_12_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO13.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_13_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_13_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_13_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_mask` reader - Mask register for GPIO13."]
pub struct REG_GPIO_13_MASK_R(crate::FieldReader<bool, REG_GPIO_13_MASK_A>);
impl REG_GPIO_13_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_13_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_13_MASK_A {
        match self.bits {
            false => REG_GPIO_13_MASK_A::UNMASKED,
            true => REG_GPIO_13_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_13_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_13_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_13_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_13_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_13_mask` writer - Mask register for GPIO13."]
pub struct REG_GPIO_13_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_13_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_13_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_13_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_13_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO14.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_14_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_14_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_14_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_mask` reader - Mask register for GPIO14."]
pub struct REG_GPIO_14_MASK_R(crate::FieldReader<bool, REG_GPIO_14_MASK_A>);
impl REG_GPIO_14_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_14_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_14_MASK_A {
        match self.bits {
            false => REG_GPIO_14_MASK_A::UNMASKED,
            true => REG_GPIO_14_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_14_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_14_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_14_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_14_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_14_mask` writer - Mask register for GPIO14."]
pub struct REG_GPIO_14_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_14_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_14_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_14_MASK_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Mask register for GPIO15.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_15_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_15_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_15_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_mask` reader - Mask register for GPIO15."]
pub struct REG_GPIO_15_MASK_R(crate::FieldReader<bool, REG_GPIO_15_MASK_A>);
impl REG_GPIO_15_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_15_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_15_MASK_A {
        match self.bits {
            false => REG_GPIO_15_MASK_A::UNMASKED,
            true => REG_GPIO_15_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_15_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_15_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_15_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_15_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_15_mask` writer - Mask register for GPIO15."]
pub struct REG_GPIO_15_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_15_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_15_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_15_MASK_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Mask register for GPIO16.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_16_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_16_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_16_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_mask` reader - Mask register for GPIO16."]
pub struct REG_GPIO_16_MASK_R(crate::FieldReader<bool, REG_GPIO_16_MASK_A>);
impl REG_GPIO_16_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_16_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_16_MASK_A {
        match self.bits {
            false => REG_GPIO_16_MASK_A::UNMASKED,
            true => REG_GPIO_16_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_16_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_16_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_16_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_16_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_16_mask` writer - Mask register for GPIO16."]
pub struct REG_GPIO_16_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_16_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_16_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_16_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO17.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_17_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_17_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_17_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_mask` reader - Mask register for GPIO17."]
pub struct REG_GPIO_17_MASK_R(crate::FieldReader<bool, REG_GPIO_17_MASK_A>);
impl REG_GPIO_17_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_17_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_17_MASK_A {
        match self.bits {
            false => REG_GPIO_17_MASK_A::UNMASKED,
            true => REG_GPIO_17_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_17_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_17_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_17_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_17_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_17_mask` writer - Mask register for GPIO17."]
pub struct REG_GPIO_17_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_17_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_17_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_17_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO18.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_18_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_18_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_18_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_mask` reader - Mask register for GPIO18."]
pub struct REG_GPIO_18_MASK_R(crate::FieldReader<bool, REG_GPIO_18_MASK_A>);
impl REG_GPIO_18_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_18_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_18_MASK_A {
        match self.bits {
            false => REG_GPIO_18_MASK_A::UNMASKED,
            true => REG_GPIO_18_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_18_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_18_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_18_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_18_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_18_mask` writer - Mask register for GPIO18."]
pub struct REG_GPIO_18_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_18_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_18_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_18_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_18_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO19.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_19_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_19_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_19_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_mask` reader - Mask register for GPIO19."]
pub struct REG_GPIO_19_MASK_R(crate::FieldReader<bool, REG_GPIO_19_MASK_A>);
impl REG_GPIO_19_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_19_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_19_MASK_A {
        match self.bits {
            false => REG_GPIO_19_MASK_A::UNMASKED,
            true => REG_GPIO_19_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_19_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_19_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_19_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_19_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_19_mask` writer - Mask register for GPIO19."]
pub struct REG_GPIO_19_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_19_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_19_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_19_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_19_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO20.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_20_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_20_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_20_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_mask` reader - Mask register for GPIO20."]
pub struct REG_GPIO_20_MASK_R(crate::FieldReader<bool, REG_GPIO_20_MASK_A>);
impl REG_GPIO_20_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_20_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_20_MASK_A {
        match self.bits {
            false => REG_GPIO_20_MASK_A::UNMASKED,
            true => REG_GPIO_20_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_20_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_20_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_20_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_20_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_20_mask` writer - Mask register for GPIO20."]
pub struct REG_GPIO_20_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_20_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_20_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_20_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_20_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO21.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_21_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_21_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_21_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_mask` reader - Mask register for GPIO21."]
pub struct REG_GPIO_21_MASK_R(crate::FieldReader<bool, REG_GPIO_21_MASK_A>);
impl REG_GPIO_21_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_21_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_21_MASK_A {
        match self.bits {
            false => REG_GPIO_21_MASK_A::UNMASKED,
            true => REG_GPIO_21_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_21_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_21_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_21_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_21_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_21_mask` writer - Mask register for GPIO21."]
pub struct REG_GPIO_21_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_21_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_21_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_21_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_21_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO22.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_22_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_22_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_22_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_mask` reader - Mask register for GPIO22."]
pub struct REG_GPIO_22_MASK_R(crate::FieldReader<bool, REG_GPIO_22_MASK_A>);
impl REG_GPIO_22_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_22_MASK_A {
        match self.bits {
            false => REG_GPIO_22_MASK_A::UNMASKED,
            true => REG_GPIO_22_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_22_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_22_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_22_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_22_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_mask` writer - Mask register for GPIO22."]
pub struct REG_GPIO_22_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_22_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_22_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_22_MASK_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Mask register for GPIO23.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_23_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_23_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_23_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_mask` reader - Mask register for GPIO23."]
pub struct REG_GPIO_23_MASK_R(crate::FieldReader<bool, REG_GPIO_23_MASK_A>);
impl REG_GPIO_23_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_23_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_23_MASK_A {
        match self.bits {
            false => REG_GPIO_23_MASK_A::UNMASKED,
            true => REG_GPIO_23_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_23_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_23_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_23_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_23_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_23_mask` writer - Mask register for GPIO23."]
pub struct REG_GPIO_23_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_23_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_23_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_23_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_23_MASK_A::MASKED)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Mask register for GPIO24.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_24_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_24_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_24_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_mask` reader - Mask register for GPIO24."]
pub struct REG_GPIO_24_MASK_R(crate::FieldReader<bool, REG_GPIO_24_MASK_A>);
impl REG_GPIO_24_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_24_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_24_MASK_A {
        match self.bits {
            false => REG_GPIO_24_MASK_A::UNMASKED,
            true => REG_GPIO_24_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_24_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_24_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_24_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_24_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_24_mask` writer - Mask register for GPIO24."]
pub struct REG_GPIO_24_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_24_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_24_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_24_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_24_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO25.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_25_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_25_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_25_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_mask` reader - Mask register for GPIO25."]
pub struct REG_GPIO_25_MASK_R(crate::FieldReader<bool, REG_GPIO_25_MASK_A>);
impl REG_GPIO_25_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_25_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_25_MASK_A {
        match self.bits {
            false => REG_GPIO_25_MASK_A::UNMASKED,
            true => REG_GPIO_25_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_25_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_25_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_25_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_25_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_25_mask` writer - Mask register for GPIO25."]
pub struct REG_GPIO_25_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_25_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_25_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_25_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_25_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_26_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_26_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_26_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_mask` reader - Mask register for GPIO26."]
pub struct REG_GPIO_26_MASK_R(crate::FieldReader<bool, REG_GPIO_26_MASK_A>);
impl REG_GPIO_26_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_26_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_26_MASK_A {
        match self.bits {
            false => REG_GPIO_26_MASK_A::UNMASKED,
            true => REG_GPIO_26_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_26_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_26_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_26_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_26_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_26_mask` writer - Mask register for GPIO26."]
pub struct REG_GPIO_26_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_26_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_26_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_26_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_27_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_27_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_27_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_mask` reader - Mask register for GPIO27."]
pub struct REG_GPIO_27_MASK_R(crate::FieldReader<bool, REG_GPIO_27_MASK_A>);
impl REG_GPIO_27_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_27_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_27_MASK_A {
        match self.bits {
            false => REG_GPIO_27_MASK_A::UNMASKED,
            true => REG_GPIO_27_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_27_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_27_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_27_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_27_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_27_mask` writer - Mask register for GPIO27."]
pub struct REG_GPIO_27_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_27_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_27_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_27_MASK_A::MASKED)
    }
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
#[doc = "Mask register for GPIO28.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_28_MASK_A {
    #[doc = "0: `0`"]
    UNMASKED = 0,
    #[doc = "1: `1`"]
    MASKED = 1,
}
impl From<REG_GPIO_28_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_28_MASK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_mask` reader - Mask register for GPIO28."]
pub struct REG_GPIO_28_MASK_R(crate::FieldReader<bool, REG_GPIO_28_MASK_A>);
impl REG_GPIO_28_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_28_MASK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_28_MASK_A {
        match self.bits {
            false => REG_GPIO_28_MASK_A::UNMASKED,
            true => REG_GPIO_28_MASK_A::MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNMASKED`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        **self == REG_GPIO_28_MASK_A::UNMASKED
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        **self == REG_GPIO_28_MASK_A::MASKED
    }
}
impl core::ops::Deref for REG_GPIO_28_MASK_R {
    type Target = crate::FieldReader<bool, REG_GPIO_28_MASK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_28_mask` writer - Mask register for GPIO28."]
pub struct REG_GPIO_28_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_28_MASK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_28_MASK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(REG_GPIO_28_MASK_A::UNMASKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(REG_GPIO_28_MASK_A::MASKED)
    }
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
impl R {
    #[doc = "Bit 0 - Mask register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_mask(&self) -> REG_GPIO_0_MASK_R {
        REG_GPIO_0_MASK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_mask(&self) -> REG_GPIO_1_MASK_R {
        REG_GPIO_1_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_mask(&self) -> REG_GPIO_2_MASK_R {
        REG_GPIO_2_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_mask(&self) -> REG_GPIO_3_MASK_R {
        REG_GPIO_3_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_mask(&self) -> REG_GPIO_4_MASK_R {
        REG_GPIO_4_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_mask(&self) -> REG_GPIO_5_MASK_R {
        REG_GPIO_5_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mask register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_mask(&self) -> REG_GPIO_6_MASK_R {
        REG_GPIO_6_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mask register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_mask(&self) -> REG_GPIO_7_MASK_R {
        REG_GPIO_7_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mask register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_mask(&self) -> REG_GPIO_8_MASK_R {
        REG_GPIO_8_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mask register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_mask(&self) -> REG_GPIO_9_MASK_R {
        REG_GPIO_9_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Mask register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_mask(&self) -> REG_GPIO_10_MASK_R {
        REG_GPIO_10_MASK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mask register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_mask(&self) -> REG_GPIO_11_MASK_R {
        REG_GPIO_11_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Mask register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_mask(&self) -> REG_GPIO_12_MASK_R {
        REG_GPIO_12_MASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Mask register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_mask(&self) -> REG_GPIO_13_MASK_R {
        REG_GPIO_13_MASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Mask register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_mask(&self) -> REG_GPIO_14_MASK_R {
        REG_GPIO_14_MASK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mask register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_mask(&self) -> REG_GPIO_15_MASK_R {
        REG_GPIO_15_MASK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Mask register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_mask(&self) -> REG_GPIO_16_MASK_R {
        REG_GPIO_16_MASK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Mask register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_mask(&self) -> REG_GPIO_17_MASK_R {
        REG_GPIO_17_MASK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mask register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_mask(&self) -> REG_GPIO_18_MASK_R {
        REG_GPIO_18_MASK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Mask register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_mask(&self) -> REG_GPIO_19_MASK_R {
        REG_GPIO_19_MASK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Mask register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_mask(&self) -> REG_GPIO_20_MASK_R {
        REG_GPIO_20_MASK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Mask register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_mask(&self) -> REG_GPIO_21_MASK_R {
        REG_GPIO_21_MASK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Mask register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_mask(&self) -> REG_GPIO_22_MASK_R {
        REG_GPIO_22_MASK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Mask register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_mask(&self) -> REG_GPIO_23_MASK_R {
        REG_GPIO_23_MASK_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Mask register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_mask(&self) -> REG_GPIO_24_MASK_R {
        REG_GPIO_24_MASK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Mask register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_mask(&self) -> REG_GPIO_25_MASK_R {
        REG_GPIO_25_MASK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Mask register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_mask(&self) -> REG_GPIO_26_MASK_R {
        REG_GPIO_26_MASK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Mask register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_mask(&self) -> REG_GPIO_27_MASK_R {
        REG_GPIO_27_MASK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Mask register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_mask(&self) -> REG_GPIO_28_MASK_R {
        REG_GPIO_28_MASK_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_mask(&mut self) -> REG_GPIO_0_MASK_W {
        REG_GPIO_0_MASK_W { w: self }
    }
    #[doc = "Bit 1 - Mask register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_mask(&mut self) -> REG_GPIO_1_MASK_W {
        REG_GPIO_1_MASK_W { w: self }
    }
    #[doc = "Bit 2 - Mask register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_mask(&mut self) -> REG_GPIO_2_MASK_W {
        REG_GPIO_2_MASK_W { w: self }
    }
    #[doc = "Bit 3 - Mask register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_mask(&mut self) -> REG_GPIO_3_MASK_W {
        REG_GPIO_3_MASK_W { w: self }
    }
    #[doc = "Bit 4 - Mask register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_mask(&mut self) -> REG_GPIO_4_MASK_W {
        REG_GPIO_4_MASK_W { w: self }
    }
    #[doc = "Bit 5 - Mask register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_mask(&mut self) -> REG_GPIO_5_MASK_W {
        REG_GPIO_5_MASK_W { w: self }
    }
    #[doc = "Bit 6 - Mask register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_mask(&mut self) -> REG_GPIO_6_MASK_W {
        REG_GPIO_6_MASK_W { w: self }
    }
    #[doc = "Bit 7 - Mask register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_mask(&mut self) -> REG_GPIO_7_MASK_W {
        REG_GPIO_7_MASK_W { w: self }
    }
    #[doc = "Bit 8 - Mask register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_mask(&mut self) -> REG_GPIO_8_MASK_W {
        REG_GPIO_8_MASK_W { w: self }
    }
    #[doc = "Bit 9 - Mask register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_mask(&mut self) -> REG_GPIO_9_MASK_W {
        REG_GPIO_9_MASK_W { w: self }
    }
    #[doc = "Bit 10 - Mask register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_mask(&mut self) -> REG_GPIO_10_MASK_W {
        REG_GPIO_10_MASK_W { w: self }
    }
    #[doc = "Bit 11 - Mask register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_mask(&mut self) -> REG_GPIO_11_MASK_W {
        REG_GPIO_11_MASK_W { w: self }
    }
    #[doc = "Bit 12 - Mask register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_mask(&mut self) -> REG_GPIO_12_MASK_W {
        REG_GPIO_12_MASK_W { w: self }
    }
    #[doc = "Bit 13 - Mask register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_mask(&mut self) -> REG_GPIO_13_MASK_W {
        REG_GPIO_13_MASK_W { w: self }
    }
    #[doc = "Bit 14 - Mask register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_mask(&mut self) -> REG_GPIO_14_MASK_W {
        REG_GPIO_14_MASK_W { w: self }
    }
    #[doc = "Bit 15 - Mask register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_mask(&mut self) -> REG_GPIO_15_MASK_W {
        REG_GPIO_15_MASK_W { w: self }
    }
    #[doc = "Bit 16 - Mask register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_mask(&mut self) -> REG_GPIO_16_MASK_W {
        REG_GPIO_16_MASK_W { w: self }
    }
    #[doc = "Bit 17 - Mask register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_mask(&mut self) -> REG_GPIO_17_MASK_W {
        REG_GPIO_17_MASK_W { w: self }
    }
    #[doc = "Bit 18 - Mask register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_mask(&mut self) -> REG_GPIO_18_MASK_W {
        REG_GPIO_18_MASK_W { w: self }
    }
    #[doc = "Bit 19 - Mask register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_mask(&mut self) -> REG_GPIO_19_MASK_W {
        REG_GPIO_19_MASK_W { w: self }
    }
    #[doc = "Bit 20 - Mask register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_mask(&mut self) -> REG_GPIO_20_MASK_W {
        REG_GPIO_20_MASK_W { w: self }
    }
    #[doc = "Bit 21 - Mask register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_mask(&mut self) -> REG_GPIO_21_MASK_W {
        REG_GPIO_21_MASK_W { w: self }
    }
    #[doc = "Bit 22 - Mask register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_mask(&mut self) -> REG_GPIO_22_MASK_W {
        REG_GPIO_22_MASK_W { w: self }
    }
    #[doc = "Bit 23 - Mask register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_mask(&mut self) -> REG_GPIO_23_MASK_W {
        REG_GPIO_23_MASK_W { w: self }
    }
    #[doc = "Bit 24 - Mask register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_mask(&mut self) -> REG_GPIO_24_MASK_W {
        REG_GPIO_24_MASK_W { w: self }
    }
    #[doc = "Bit 25 - Mask register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_mask(&mut self) -> REG_GPIO_25_MASK_W {
        REG_GPIO_25_MASK_W { w: self }
    }
    #[doc = "Bit 26 - Mask register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_mask(&mut self) -> REG_GPIO_26_MASK_W {
        REG_GPIO_26_MASK_W { w: self }
    }
    #[doc = "Bit 27 - Mask register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_mask(&mut self) -> REG_GPIO_27_MASK_W {
        REG_GPIO_27_MASK_W { w: self }
    }
    #[doc = "Bit 28 - Mask register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_mask(&mut self) -> REG_GPIO_28_MASK_W {
        REG_GPIO_28_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt masking register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_mask1](index.html) module"]
pub struct GPIO_INT_MASK1_SPEC;
impl crate::RegisterSpec for GPIO_INT_MASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_mask1::R](R) reader structure"]
impl crate::Readable for GPIO_INT_MASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_mask1::W](W) writer structure"]
impl crate::Writable for GPIO_INT_MASK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INT_MASK1 to value 0xffff_ffff"]
impl crate::Resettable for GPIO_INT_MASK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
