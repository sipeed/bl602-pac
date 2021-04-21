#[doc = "Register `GPIO_CFGCTL32` reader"]
pub struct R(crate::R<GPIO_CFGCTL32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_CFGCTL32_SPEC>> for R {
    fn from(reader: crate::R<GPIO_CFGCTL32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL32` writer"]
pub struct W(crate::W<GPIO_CFGCTL32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL32_SPEC>;
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
impl core::convert::From<crate::W<GPIO_CFGCTL32_SPEC>> for W {
    fn from(writer: crate::W<GPIO_CFGCTL32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Output register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_22_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_22_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_22_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_o` reader - Output register for GPIO22."]
pub struct REG_GPIO_22_O_R(crate::FieldReader<bool, REG_GPIO_22_O_A>);
impl REG_GPIO_22_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_22_O_A {
        match self.bits {
            false => REG_GPIO_22_O_A::DISABLED,
            true => REG_GPIO_22_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_22_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_22_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_22_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_22_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_o` writer - Output register for GPIO22."]
pub struct REG_GPIO_22_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_22_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_22_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_22_O_A::ENABLED)
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
#[doc = "Output register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_21_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_21_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_21_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_o` reader - Output register for GPIO21."]
pub struct REG_GPIO_21_O_R(crate::FieldReader<bool, REG_GPIO_21_O_A>);
impl REG_GPIO_21_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_21_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_21_O_A {
        match self.bits {
            false => REG_GPIO_21_O_A::DISABLED,
            true => REG_GPIO_21_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_21_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_21_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_21_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_21_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_21_o` writer - Output register for GPIO21."]
pub struct REG_GPIO_21_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_21_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_21_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_21_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_21_O_A::ENABLED)
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
#[doc = "Output register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_20_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_20_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_20_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_o` reader - Output register for GPIO20."]
pub struct REG_GPIO_20_O_R(crate::FieldReader<bool, REG_GPIO_20_O_A>);
impl REG_GPIO_20_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_20_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_20_O_A {
        match self.bits {
            false => REG_GPIO_20_O_A::DISABLED,
            true => REG_GPIO_20_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_20_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_20_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_20_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_20_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_20_o` writer - Output register for GPIO20."]
pub struct REG_GPIO_20_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_20_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_20_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_20_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_20_O_A::ENABLED)
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
#[doc = "Output register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_19_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_19_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_19_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_o` reader - Output register for GPIO19."]
pub struct REG_GPIO_19_O_R(crate::FieldReader<bool, REG_GPIO_19_O_A>);
impl REG_GPIO_19_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_19_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_19_O_A {
        match self.bits {
            false => REG_GPIO_19_O_A::DISABLED,
            true => REG_GPIO_19_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_19_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_19_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_19_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_19_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_19_o` writer - Output register for GPIO19."]
pub struct REG_GPIO_19_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_19_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_19_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_19_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_19_O_A::ENABLED)
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
#[doc = "Output register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_18_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_18_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_18_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_o` reader - Output register for GPIO18."]
pub struct REG_GPIO_18_O_R(crate::FieldReader<bool, REG_GPIO_18_O_A>);
impl REG_GPIO_18_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_18_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_18_O_A {
        match self.bits {
            false => REG_GPIO_18_O_A::DISABLED,
            true => REG_GPIO_18_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_18_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_18_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_18_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_18_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_18_o` writer - Output register for GPIO18."]
pub struct REG_GPIO_18_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_18_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_18_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_18_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_18_O_A::ENABLED)
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
#[doc = "Output register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_17_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_17_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_17_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_o` reader - Output register for GPIO17."]
pub struct REG_GPIO_17_O_R(crate::FieldReader<bool, REG_GPIO_17_O_A>);
impl REG_GPIO_17_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_17_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_17_O_A {
        match self.bits {
            false => REG_GPIO_17_O_A::DISABLED,
            true => REG_GPIO_17_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_17_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_17_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_17_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_17_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_17_o` writer - Output register for GPIO17."]
pub struct REG_GPIO_17_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_17_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_O_A::ENABLED)
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
#[doc = "Output register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_16_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_16_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_16_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_o` reader - Output register for GPIO16."]
pub struct REG_GPIO_16_O_R(crate::FieldReader<bool, REG_GPIO_16_O_A>);
impl REG_GPIO_16_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_16_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_16_O_A {
        match self.bits {
            false => REG_GPIO_16_O_A::DISABLED,
            true => REG_GPIO_16_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_16_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_16_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_16_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_16_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_16_o` writer - Output register for GPIO16."]
pub struct REG_GPIO_16_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_16_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_O_A::ENABLED)
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
#[doc = "Output register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_15_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_15_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_15_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_o` reader - Output register for GPIO15."]
pub struct REG_GPIO_15_O_R(crate::FieldReader<bool, REG_GPIO_15_O_A>);
impl REG_GPIO_15_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_15_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_15_O_A {
        match self.bits {
            false => REG_GPIO_15_O_A::DISABLED,
            true => REG_GPIO_15_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_15_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_15_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_15_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_15_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_15_o` writer - Output register for GPIO15."]
pub struct REG_GPIO_15_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_15_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_15_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_15_O_A::ENABLED)
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
#[doc = "Output register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_14_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_14_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_14_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_o` reader - Output register for GPIO14."]
pub struct REG_GPIO_14_O_R(crate::FieldReader<bool, REG_GPIO_14_O_A>);
impl REG_GPIO_14_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_14_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_14_O_A {
        match self.bits {
            false => REG_GPIO_14_O_A::DISABLED,
            true => REG_GPIO_14_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_14_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_14_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_14_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_14_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_14_o` writer - Output register for GPIO14."]
pub struct REG_GPIO_14_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_14_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_14_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_14_O_A::ENABLED)
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
#[doc = "Output register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_13_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_13_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_13_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_o` reader - Output register for GPIO13."]
pub struct REG_GPIO_13_O_R(crate::FieldReader<bool, REG_GPIO_13_O_A>);
impl REG_GPIO_13_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_13_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_13_O_A {
        match self.bits {
            false => REG_GPIO_13_O_A::DISABLED,
            true => REG_GPIO_13_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_13_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_13_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_13_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_13_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_13_o` writer - Output register for GPIO13."]
pub struct REG_GPIO_13_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_13_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_13_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_13_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_13_O_A::ENABLED)
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
#[doc = "Output register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_12_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_12_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_12_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_o` reader - Output register for GPIO12."]
pub struct REG_GPIO_12_O_R(crate::FieldReader<bool, REG_GPIO_12_O_A>);
impl REG_GPIO_12_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_12_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_12_O_A {
        match self.bits {
            false => REG_GPIO_12_O_A::DISABLED,
            true => REG_GPIO_12_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_12_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_12_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_12_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_12_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_12_o` writer - Output register for GPIO12."]
pub struct REG_GPIO_12_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_12_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_12_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_12_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_12_O_A::ENABLED)
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
#[doc = "Output register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_11_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_11_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_11_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_o` reader - Output register for GPIO11."]
pub struct REG_GPIO_11_O_R(crate::FieldReader<bool, REG_GPIO_11_O_A>);
impl REG_GPIO_11_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_11_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_11_O_A {
        match self.bits {
            false => REG_GPIO_11_O_A::DISABLED,
            true => REG_GPIO_11_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_11_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_11_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_11_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_11_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_11_o` writer - Output register for GPIO11."]
pub struct REG_GPIO_11_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_11_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_O_A::ENABLED)
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
#[doc = "Output register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_10_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_10_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_10_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_o` reader - Output register for GPIO10."]
pub struct REG_GPIO_10_O_R(crate::FieldReader<bool, REG_GPIO_10_O_A>);
impl REG_GPIO_10_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_10_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_10_O_A {
        match self.bits {
            false => REG_GPIO_10_O_A::DISABLED,
            true => REG_GPIO_10_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_10_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_10_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_10_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_10_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_10_o` writer - Output register for GPIO10."]
pub struct REG_GPIO_10_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_10_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_O_A::ENABLED)
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
#[doc = "Output register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_9_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_9_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_9_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_o` reader - Output register for GPIO9."]
pub struct REG_GPIO_9_O_R(crate::FieldReader<bool, REG_GPIO_9_O_A>);
impl REG_GPIO_9_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_9_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_9_O_A {
        match self.bits {
            false => REG_GPIO_9_O_A::DISABLED,
            true => REG_GPIO_9_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_9_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_9_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_9_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_9_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_9_o` writer - Output register for GPIO9."]
pub struct REG_GPIO_9_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_9_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_9_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_9_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_9_O_A::ENABLED)
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
#[doc = "Output register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_8_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_8_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_8_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_o` reader - Output register for GPIO8."]
pub struct REG_GPIO_8_O_R(crate::FieldReader<bool, REG_GPIO_8_O_A>);
impl REG_GPIO_8_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_8_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_8_O_A {
        match self.bits {
            false => REG_GPIO_8_O_A::DISABLED,
            true => REG_GPIO_8_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_8_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_8_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_8_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_8_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_8_o` writer - Output register for GPIO8."]
pub struct REG_GPIO_8_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_8_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_8_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_8_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_8_O_A::ENABLED)
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
#[doc = "Output register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_7_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_7_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_7_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_o` reader - Output register for GPIO7."]
pub struct REG_GPIO_7_O_R(crate::FieldReader<bool, REG_GPIO_7_O_A>);
impl REG_GPIO_7_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_7_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_7_O_A {
        match self.bits {
            false => REG_GPIO_7_O_A::DISABLED,
            true => REG_GPIO_7_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_7_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_7_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_7_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_7_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_7_o` writer - Output register for GPIO7."]
pub struct REG_GPIO_7_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_7_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_7_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_7_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_7_O_A::ENABLED)
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
#[doc = "Output register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_6_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_6_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_6_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_o` reader - Output register for GPIO6."]
pub struct REG_GPIO_6_O_R(crate::FieldReader<bool, REG_GPIO_6_O_A>);
impl REG_GPIO_6_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_6_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_6_O_A {
        match self.bits {
            false => REG_GPIO_6_O_A::DISABLED,
            true => REG_GPIO_6_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_6_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_6_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_6_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_6_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_6_o` writer - Output register for GPIO6."]
pub struct REG_GPIO_6_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_6_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_6_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_6_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_6_O_A::ENABLED)
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
#[doc = "Output register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_5_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_5_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_5_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_o` reader - Output register for GPIO5."]
pub struct REG_GPIO_5_O_R(crate::FieldReader<bool, REG_GPIO_5_O_A>);
impl REG_GPIO_5_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_5_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_5_O_A {
        match self.bits {
            false => REG_GPIO_5_O_A::DISABLED,
            true => REG_GPIO_5_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_5_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_5_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_5_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_5_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_5_o` writer - Output register for GPIO5."]
pub struct REG_GPIO_5_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_5_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_5_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_5_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_5_O_A::ENABLED)
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
#[doc = "Output register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_4_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_4_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_4_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_o` reader - Output register for GPIO4."]
pub struct REG_GPIO_4_O_R(crate::FieldReader<bool, REG_GPIO_4_O_A>);
impl REG_GPIO_4_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_4_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_4_O_A {
        match self.bits {
            false => REG_GPIO_4_O_A::DISABLED,
            true => REG_GPIO_4_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_4_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_4_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_4_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_4_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_4_o` writer - Output register for GPIO4."]
pub struct REG_GPIO_4_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_4_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_4_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_4_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_4_O_A::ENABLED)
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
#[doc = "Output register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_3_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_3_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_3_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_o` reader - Output register for GPIO3."]
pub struct REG_GPIO_3_O_R(crate::FieldReader<bool, REG_GPIO_3_O_A>);
impl REG_GPIO_3_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_3_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_3_O_A {
        match self.bits {
            false => REG_GPIO_3_O_A::DISABLED,
            true => REG_GPIO_3_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_3_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_3_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_3_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_3_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_3_o` writer - Output register for GPIO3."]
pub struct REG_GPIO_3_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_3_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_3_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_3_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_3_O_A::ENABLED)
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
#[doc = "Output register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_2_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_2_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_2_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_o` reader - Output register for GPIO2."]
pub struct REG_GPIO_2_O_R(crate::FieldReader<bool, REG_GPIO_2_O_A>);
impl REG_GPIO_2_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_2_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_2_O_A {
        match self.bits {
            false => REG_GPIO_2_O_A::DISABLED,
            true => REG_GPIO_2_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_2_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_2_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_2_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_2_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_2_o` writer - Output register for GPIO2."]
pub struct REG_GPIO_2_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_2_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_2_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_2_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_2_O_A::ENABLED)
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
#[doc = "Output register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_1_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_1_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_1_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_o` reader - Output register for GPIO1."]
pub struct REG_GPIO_1_O_R(crate::FieldReader<bool, REG_GPIO_1_O_A>);
impl REG_GPIO_1_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_1_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_1_O_A {
        match self.bits {
            false => REG_GPIO_1_O_A::DISABLED,
            true => REG_GPIO_1_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_1_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_1_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_1_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_1_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_1_o` writer - Output register for GPIO1."]
pub struct REG_GPIO_1_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_1_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_O_A::ENABLED)
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
#[doc = "Output register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_0_O_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_0_O_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_0_O_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_o` reader - Output register for GPIO0."]
pub struct REG_GPIO_0_O_R(crate::FieldReader<bool, REG_GPIO_0_O_A>);
impl REG_GPIO_0_O_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_0_O_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_0_O_A {
        match self.bits {
            false => REG_GPIO_0_O_A::DISABLED,
            true => REG_GPIO_0_O_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_0_O_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_0_O_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_0_O_R {
    type Target = crate::FieldReader<bool, REG_GPIO_0_O_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_0_o` writer - Output register for GPIO0."]
pub struct REG_GPIO_0_O_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_O_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_0_O_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_O_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_O_A::ENABLED)
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
impl R {
    #[doc = "Bit 22 - Output register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_o(&self) -> REG_GPIO_22_O_R {
        REG_GPIO_22_O_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Output register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_o(&self) -> REG_GPIO_21_O_R {
        REG_GPIO_21_O_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Output register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_o(&self) -> REG_GPIO_20_O_R {
        REG_GPIO_20_O_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Output register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_o(&self) -> REG_GPIO_19_O_R {
        REG_GPIO_19_O_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_o(&self) -> REG_GPIO_18_O_R {
        REG_GPIO_18_O_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Output register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_o(&self) -> REG_GPIO_17_O_R {
        REG_GPIO_17_O_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Output register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_o(&self) -> REG_GPIO_16_O_R {
        REG_GPIO_16_O_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Output register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_o(&self) -> REG_GPIO_15_O_R {
        REG_GPIO_15_O_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Output register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_o(&self) -> REG_GPIO_14_O_R {
        REG_GPIO_14_O_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Output register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_o(&self) -> REG_GPIO_13_O_R {
        REG_GPIO_13_O_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Output register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_o(&self) -> REG_GPIO_12_O_R {
        REG_GPIO_12_O_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_o(&self) -> REG_GPIO_11_O_R {
        REG_GPIO_11_O_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_o(&self) -> REG_GPIO_10_O_R {
        REG_GPIO_10_O_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Output register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_o(&self) -> REG_GPIO_9_O_R {
        REG_GPIO_9_O_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Output register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_o(&self) -> REG_GPIO_8_O_R {
        REG_GPIO_8_O_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Output register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_o(&self) -> REG_GPIO_7_O_R {
        REG_GPIO_7_O_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Output register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_o(&self) -> REG_GPIO_6_O_R {
        REG_GPIO_6_O_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Output register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_o(&self) -> REG_GPIO_5_O_R {
        REG_GPIO_5_O_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Output register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_o(&self) -> REG_GPIO_4_O_R {
        REG_GPIO_4_O_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_o(&self) -> REG_GPIO_3_O_R {
        REG_GPIO_3_O_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_o(&self) -> REG_GPIO_2_O_R {
        REG_GPIO_2_O_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_o(&self) -> REG_GPIO_1_O_R {
        REG_GPIO_1_O_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Output register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_o(&self) -> REG_GPIO_0_O_R {
        REG_GPIO_0_O_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - Output register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_o(&mut self) -> REG_GPIO_22_O_W {
        REG_GPIO_22_O_W { w: self }
    }
    #[doc = "Bit 21 - Output register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_o(&mut self) -> REG_GPIO_21_O_W {
        REG_GPIO_21_O_W { w: self }
    }
    #[doc = "Bit 20 - Output register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_o(&mut self) -> REG_GPIO_20_O_W {
        REG_GPIO_20_O_W { w: self }
    }
    #[doc = "Bit 19 - Output register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_o(&mut self) -> REG_GPIO_19_O_W {
        REG_GPIO_19_O_W { w: self }
    }
    #[doc = "Bit 18 - Output register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_o(&mut self) -> REG_GPIO_18_O_W {
        REG_GPIO_18_O_W { w: self }
    }
    #[doc = "Bit 17 - Output register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_o(&mut self) -> REG_GPIO_17_O_W {
        REG_GPIO_17_O_W { w: self }
    }
    #[doc = "Bit 16 - Output register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_o(&mut self) -> REG_GPIO_16_O_W {
        REG_GPIO_16_O_W { w: self }
    }
    #[doc = "Bit 15 - Output register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_o(&mut self) -> REG_GPIO_15_O_W {
        REG_GPIO_15_O_W { w: self }
    }
    #[doc = "Bit 14 - Output register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_o(&mut self) -> REG_GPIO_14_O_W {
        REG_GPIO_14_O_W { w: self }
    }
    #[doc = "Bit 13 - Output register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_o(&mut self) -> REG_GPIO_13_O_W {
        REG_GPIO_13_O_W { w: self }
    }
    #[doc = "Bit 12 - Output register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_o(&mut self) -> REG_GPIO_12_O_W {
        REG_GPIO_12_O_W { w: self }
    }
    #[doc = "Bit 11 - Output register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_o(&mut self) -> REG_GPIO_11_O_W {
        REG_GPIO_11_O_W { w: self }
    }
    #[doc = "Bit 10 - Output register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_o(&mut self) -> REG_GPIO_10_O_W {
        REG_GPIO_10_O_W { w: self }
    }
    #[doc = "Bit 9 - Output register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_o(&mut self) -> REG_GPIO_9_O_W {
        REG_GPIO_9_O_W { w: self }
    }
    #[doc = "Bit 8 - Output register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_o(&mut self) -> REG_GPIO_8_O_W {
        REG_GPIO_8_O_W { w: self }
    }
    #[doc = "Bit 7 - Output register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_o(&mut self) -> REG_GPIO_7_O_W {
        REG_GPIO_7_O_W { w: self }
    }
    #[doc = "Bit 6 - Output register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_o(&mut self) -> REG_GPIO_6_O_W {
        REG_GPIO_6_O_W { w: self }
    }
    #[doc = "Bit 5 - Output register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_o(&mut self) -> REG_GPIO_5_O_W {
        REG_GPIO_5_O_W { w: self }
    }
    #[doc = "Bit 4 - Output register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_o(&mut self) -> REG_GPIO_4_O_W {
        REG_GPIO_4_O_W { w: self }
    }
    #[doc = "Bit 3 - Output register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_o(&mut self) -> REG_GPIO_3_O_W {
        REG_GPIO_3_O_W { w: self }
    }
    #[doc = "Bit 2 - Output register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_o(&mut self) -> REG_GPIO_2_O_W {
        REG_GPIO_2_O_W { w: self }
    }
    #[doc = "Bit 1 - Output register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_o(&mut self) -> REG_GPIO_1_O_W {
        REG_GPIO_1_O_W { w: self }
    }
    #[doc = "Bit 0 - Output register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_o(&mut self) -> REG_GPIO_0_O_W {
        REG_GPIO_0_O_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output register for all GPIO pins. Output Enabled bit must be set in Output Enable register to work.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl32](index.html) module"]
pub struct GPIO_CFGCTL32_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl32::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl32::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL32_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_CFGCTL32 to value 0"]
impl crate::Resettable for GPIO_CFGCTL32_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
