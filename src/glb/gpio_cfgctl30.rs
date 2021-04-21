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
#[doc = "Input register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_22_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_22_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_22_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_i` reader - Input register for GPIO22."]
pub struct REG_GPIO_22_I_R(crate::FieldReader<bool, REG_GPIO_22_I_A>);
impl REG_GPIO_22_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_22_I_A {
        match self.bits {
            false => REG_GPIO_22_I_A::DISABLED,
            true => REG_GPIO_22_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_22_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_22_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_22_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_22_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_21_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_21_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_21_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_i` reader - Input register for GPIO21."]
pub struct REG_GPIO_21_I_R(crate::FieldReader<bool, REG_GPIO_21_I_A>);
impl REG_GPIO_21_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_21_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_21_I_A {
        match self.bits {
            false => REG_GPIO_21_I_A::DISABLED,
            true => REG_GPIO_21_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_21_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_21_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_21_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_21_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_20_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_20_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_20_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_i` reader - Input register for GPIO20."]
pub struct REG_GPIO_20_I_R(crate::FieldReader<bool, REG_GPIO_20_I_A>);
impl REG_GPIO_20_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_20_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_20_I_A {
        match self.bits {
            false => REG_GPIO_20_I_A::DISABLED,
            true => REG_GPIO_20_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_20_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_20_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_20_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_20_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_19_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_19_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_19_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_i` reader - Input register for GPIO19."]
pub struct REG_GPIO_19_I_R(crate::FieldReader<bool, REG_GPIO_19_I_A>);
impl REG_GPIO_19_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_19_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_19_I_A {
        match self.bits {
            false => REG_GPIO_19_I_A::DISABLED,
            true => REG_GPIO_19_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_19_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_19_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_19_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_19_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_18_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_18_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_18_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_i` reader - Input register for GPIO18."]
pub struct REG_GPIO_18_I_R(crate::FieldReader<bool, REG_GPIO_18_I_A>);
impl REG_GPIO_18_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_18_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_18_I_A {
        match self.bits {
            false => REG_GPIO_18_I_A::DISABLED,
            true => REG_GPIO_18_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_18_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_18_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_18_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_18_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_17_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_17_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_17_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_i` reader - Input register for GPIO17."]
pub struct REG_GPIO_17_I_R(crate::FieldReader<bool, REG_GPIO_17_I_A>);
impl REG_GPIO_17_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_17_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_17_I_A {
        match self.bits {
            false => REG_GPIO_17_I_A::DISABLED,
            true => REG_GPIO_17_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_17_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_17_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_17_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_17_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_16_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_16_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_16_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_i` reader - Input register for GPIO16."]
pub struct REG_GPIO_16_I_R(crate::FieldReader<bool, REG_GPIO_16_I_A>);
impl REG_GPIO_16_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_16_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_16_I_A {
        match self.bits {
            false => REG_GPIO_16_I_A::DISABLED,
            true => REG_GPIO_16_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_16_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_16_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_16_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_16_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_15_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_15_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_15_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_i` reader - Input register for GPIO15."]
pub struct REG_GPIO_15_I_R(crate::FieldReader<bool, REG_GPIO_15_I_A>);
impl REG_GPIO_15_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_15_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_15_I_A {
        match self.bits {
            false => REG_GPIO_15_I_A::DISABLED,
            true => REG_GPIO_15_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_15_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_15_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_15_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_15_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_14_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_14_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_14_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_i` reader - Input register for GPIO14."]
pub struct REG_GPIO_14_I_R(crate::FieldReader<bool, REG_GPIO_14_I_A>);
impl REG_GPIO_14_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_14_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_14_I_A {
        match self.bits {
            false => REG_GPIO_14_I_A::DISABLED,
            true => REG_GPIO_14_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_14_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_14_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_14_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_14_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_13_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_13_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_13_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_i` reader - Input register for GPIO13."]
pub struct REG_GPIO_13_I_R(crate::FieldReader<bool, REG_GPIO_13_I_A>);
impl REG_GPIO_13_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_13_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_13_I_A {
        match self.bits {
            false => REG_GPIO_13_I_A::DISABLED,
            true => REG_GPIO_13_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_13_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_13_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_13_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_13_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_12_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_12_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_12_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_i` reader - Input register for GPIO12."]
pub struct REG_GPIO_12_I_R(crate::FieldReader<bool, REG_GPIO_12_I_A>);
impl REG_GPIO_12_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_12_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_12_I_A {
        match self.bits {
            false => REG_GPIO_12_I_A::DISABLED,
            true => REG_GPIO_12_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_12_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_12_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_12_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_12_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_11_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_11_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_11_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_i` reader - Input register for GPIO11."]
pub struct REG_GPIO_11_I_R(crate::FieldReader<bool, REG_GPIO_11_I_A>);
impl REG_GPIO_11_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_11_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_11_I_A {
        match self.bits {
            false => REG_GPIO_11_I_A::DISABLED,
            true => REG_GPIO_11_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_11_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_11_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_11_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_11_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_10_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_10_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_10_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_i` reader - Input register for GPIO10."]
pub struct REG_GPIO_10_I_R(crate::FieldReader<bool, REG_GPIO_10_I_A>);
impl REG_GPIO_10_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_10_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_10_I_A {
        match self.bits {
            false => REG_GPIO_10_I_A::DISABLED,
            true => REG_GPIO_10_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_10_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_10_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_10_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_10_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_9_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_9_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_9_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_i` reader - Input register for GPIO9."]
pub struct REG_GPIO_9_I_R(crate::FieldReader<bool, REG_GPIO_9_I_A>);
impl REG_GPIO_9_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_9_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_9_I_A {
        match self.bits {
            false => REG_GPIO_9_I_A::DISABLED,
            true => REG_GPIO_9_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_9_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_9_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_9_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_9_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_8_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_8_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_8_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_i` reader - Input register for GPIO8."]
pub struct REG_GPIO_8_I_R(crate::FieldReader<bool, REG_GPIO_8_I_A>);
impl REG_GPIO_8_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_8_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_8_I_A {
        match self.bits {
            false => REG_GPIO_8_I_A::DISABLED,
            true => REG_GPIO_8_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_8_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_8_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_8_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_8_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_7_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_7_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_7_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_i` reader - Input register for GPIO7."]
pub struct REG_GPIO_7_I_R(crate::FieldReader<bool, REG_GPIO_7_I_A>);
impl REG_GPIO_7_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_7_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_7_I_A {
        match self.bits {
            false => REG_GPIO_7_I_A::DISABLED,
            true => REG_GPIO_7_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_7_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_7_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_7_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_7_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_6_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_6_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_6_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_i` reader - Input register for GPIO6."]
pub struct REG_GPIO_6_I_R(crate::FieldReader<bool, REG_GPIO_6_I_A>);
impl REG_GPIO_6_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_6_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_6_I_A {
        match self.bits {
            false => REG_GPIO_6_I_A::DISABLED,
            true => REG_GPIO_6_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_6_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_6_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_6_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_6_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_5_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_5_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_5_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_i` reader - Input register for GPIO5."]
pub struct REG_GPIO_5_I_R(crate::FieldReader<bool, REG_GPIO_5_I_A>);
impl REG_GPIO_5_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_5_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_5_I_A {
        match self.bits {
            false => REG_GPIO_5_I_A::DISABLED,
            true => REG_GPIO_5_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_5_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_5_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_5_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_5_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_4_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_4_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_4_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_i` reader - Input register for GPIO4."]
pub struct REG_GPIO_4_I_R(crate::FieldReader<bool, REG_GPIO_4_I_A>);
impl REG_GPIO_4_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_4_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_4_I_A {
        match self.bits {
            false => REG_GPIO_4_I_A::DISABLED,
            true => REG_GPIO_4_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_4_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_4_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_4_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_4_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_3_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_3_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_3_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_i` reader - Input register for GPIO3."]
pub struct REG_GPIO_3_I_R(crate::FieldReader<bool, REG_GPIO_3_I_A>);
impl REG_GPIO_3_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_3_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_3_I_A {
        match self.bits {
            false => REG_GPIO_3_I_A::DISABLED,
            true => REG_GPIO_3_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_3_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_3_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_3_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_3_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_2_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_2_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_2_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_i` reader - Input register for GPIO2."]
pub struct REG_GPIO_2_I_R(crate::FieldReader<bool, REG_GPIO_2_I_A>);
impl REG_GPIO_2_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_2_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_2_I_A {
        match self.bits {
            false => REG_GPIO_2_I_A::DISABLED,
            true => REG_GPIO_2_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_2_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_2_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_2_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_2_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Input register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_1_I_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_1_I_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_1_I_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_i` reader - Input register for GPIO1."]
pub struct REG_GPIO_1_I_R(crate::FieldReader<bool, REG_GPIO_1_I_A>);
impl REG_GPIO_1_I_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_1_I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_1_I_A {
        match self.bits {
            false => REG_GPIO_1_I_A::DISABLED,
            true => REG_GPIO_1_I_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_1_I_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_1_I_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_1_I_R {
    type Target = crate::FieldReader<bool, REG_GPIO_1_I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
impl R {
    #[doc = "Bit 22 - Input register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_i(&self) -> REG_GPIO_22_I_R {
        REG_GPIO_22_I_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Input register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_i(&self) -> REG_GPIO_21_I_R {
        REG_GPIO_21_I_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Input register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_i(&self) -> REG_GPIO_20_I_R {
        REG_GPIO_20_I_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Input register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_i(&self) -> REG_GPIO_19_I_R {
        REG_GPIO_19_I_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Input register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_i(&self) -> REG_GPIO_18_I_R {
        REG_GPIO_18_I_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Input register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_i(&self) -> REG_GPIO_17_I_R {
        REG_GPIO_17_I_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_i(&self) -> REG_GPIO_16_I_R {
        REG_GPIO_16_I_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Input register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_i(&self) -> REG_GPIO_15_I_R {
        REG_GPIO_15_I_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Input register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_i(&self) -> REG_GPIO_14_I_R {
        REG_GPIO_14_I_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Input register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_i(&self) -> REG_GPIO_13_I_R {
        REG_GPIO_13_I_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Input register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_i(&self) -> REG_GPIO_12_I_R {
        REG_GPIO_12_I_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Input register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_i(&self) -> REG_GPIO_11_I_R {
        REG_GPIO_11_I_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Input register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_i(&self) -> REG_GPIO_10_I_R {
        REG_GPIO_10_I_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Input register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_i(&self) -> REG_GPIO_9_I_R {
        REG_GPIO_9_I_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Input register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_i(&self) -> REG_GPIO_8_I_R {
        REG_GPIO_8_I_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Input register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_i(&self) -> REG_GPIO_7_I_R {
        REG_GPIO_7_I_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Input register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_i(&self) -> REG_GPIO_6_I_R {
        REG_GPIO_6_I_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Input register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_i(&self) -> REG_GPIO_5_I_R {
        REG_GPIO_5_I_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Input register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_i(&self) -> REG_GPIO_4_I_R {
        REG_GPIO_4_I_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Input register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_i(&self) -> REG_GPIO_3_I_R {
        REG_GPIO_3_I_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_i(&self) -> REG_GPIO_2_I_R {
        REG_GPIO_2_I_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Input register for GPIO1."]
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
#[doc = "Input register for all GPIO pins. Input Enabled bit must be set in configuration register to work.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl30](index.html) module"]
pub struct GPIO_CFGCTL30_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL30_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl30::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL30_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_CFGCTL30 to value 0"]
impl crate::Resettable for GPIO_CFGCTL30_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
