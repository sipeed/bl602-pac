#[doc = "Register `GPIO_INT_STAT1` reader"]
pub struct R(crate::R<GPIO_INT_STAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_STAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_INT_STAT1_SPEC>> for R {
    fn from(reader: crate::R<GPIO_INT_STAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Interrupt status register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_0_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_0_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_0_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_interrupt_status` reader - Interrupt status register for GPIO0."]
pub struct REG_GPIO_0_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_0_INTERRUPT_STATUS_A>);
impl REG_GPIO_0_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_0_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_0_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_0_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_0_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_0_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_0_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_0_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_0_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_1_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_1_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_1_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_interrupt_status` reader - Interrupt status register for GPIO1."]
pub struct REG_GPIO_1_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_1_INTERRUPT_STATUS_A>);
impl REG_GPIO_1_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_1_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_1_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_1_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_1_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_1_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_1_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_1_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_1_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_2_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_2_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_2_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_interrupt_status` reader - Interrupt status register for GPIO2."]
pub struct REG_GPIO_2_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_2_INTERRUPT_STATUS_A>);
impl REG_GPIO_2_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_2_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_2_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_2_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_2_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_2_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_2_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_2_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_2_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_3_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_3_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_3_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_interrupt_status` reader - Interrupt status register for GPIO3."]
pub struct REG_GPIO_3_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_3_INTERRUPT_STATUS_A>);
impl REG_GPIO_3_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_3_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_3_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_3_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_3_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_3_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_3_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_3_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_3_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_4_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_4_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_4_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_interrupt_status` reader - Interrupt status register for GPIO4."]
pub struct REG_GPIO_4_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_4_INTERRUPT_STATUS_A>);
impl REG_GPIO_4_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_4_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_4_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_4_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_4_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_4_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_4_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_4_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_4_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_5_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_5_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_5_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_interrupt_status` reader - Interrupt status register for GPIO5."]
pub struct REG_GPIO_5_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_5_INTERRUPT_STATUS_A>);
impl REG_GPIO_5_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_5_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_5_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_5_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_5_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_5_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_5_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_5_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_5_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_6_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_6_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_6_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_interrupt_status` reader - Interrupt status register for GPIO6."]
pub struct REG_GPIO_6_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_6_INTERRUPT_STATUS_A>);
impl REG_GPIO_6_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_6_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_6_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_6_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_6_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_6_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_6_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_6_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_6_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_7_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_7_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_7_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_interrupt_status` reader - Interrupt status register for GPIO7."]
pub struct REG_GPIO_7_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_7_INTERRUPT_STATUS_A>);
impl REG_GPIO_7_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_7_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_7_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_7_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_7_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_7_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_7_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_7_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_7_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_8_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_8_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_8_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_interrupt_status` reader - Interrupt status register for GPIO8."]
pub struct REG_GPIO_8_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_8_INTERRUPT_STATUS_A>);
impl REG_GPIO_8_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_8_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_8_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_8_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_8_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_8_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_8_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_8_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_8_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_9_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_9_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_9_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_interrupt_status` reader - Interrupt status register for GPIO9."]
pub struct REG_GPIO_9_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_9_INTERRUPT_STATUS_A>);
impl REG_GPIO_9_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_9_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_9_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_9_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_9_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_9_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_9_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_9_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_9_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_10_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_10_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_10_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_interrupt_status` reader - Interrupt status register for GPIO10."]
pub struct REG_GPIO_10_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_10_INTERRUPT_STATUS_A>);
impl REG_GPIO_10_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_10_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_10_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_10_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_10_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_10_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_10_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_10_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_10_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_11_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_11_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_11_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_interrupt_status` reader - Interrupt status register for GPIO11."]
pub struct REG_GPIO_11_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_11_INTERRUPT_STATUS_A>);
impl REG_GPIO_11_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_11_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_11_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_11_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_11_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_11_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_11_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_11_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_11_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_12_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_12_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_12_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_interrupt_status` reader - Interrupt status register for GPIO12."]
pub struct REG_GPIO_12_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_12_INTERRUPT_STATUS_A>);
impl REG_GPIO_12_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_12_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_12_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_12_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_12_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_12_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_12_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_12_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_12_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_13_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_13_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_13_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_interrupt_status` reader - Interrupt status register for GPIO13."]
pub struct REG_GPIO_13_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_13_INTERRUPT_STATUS_A>);
impl REG_GPIO_13_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_13_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_13_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_13_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_13_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_13_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_13_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_13_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_13_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_14_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_14_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_14_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_interrupt_status` reader - Interrupt status register for GPIO14."]
pub struct REG_GPIO_14_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_14_INTERRUPT_STATUS_A>);
impl REG_GPIO_14_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_14_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_14_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_14_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_14_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_14_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_14_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_14_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_14_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_15_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_15_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_15_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_interrupt_status` reader - Interrupt status register for GPIO15."]
pub struct REG_GPIO_15_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_15_INTERRUPT_STATUS_A>);
impl REG_GPIO_15_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_15_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_15_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_15_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_15_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_15_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_15_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_15_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_15_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_16_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_16_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_16_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_interrupt_status` reader - Interrupt status register for GPIO16."]
pub struct REG_GPIO_16_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_16_INTERRUPT_STATUS_A>);
impl REG_GPIO_16_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_16_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_16_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_16_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_16_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_16_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_16_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_16_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_16_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_17_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_17_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_17_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_interrupt_status` reader - Interrupt status register for GPIO17."]
pub struct REG_GPIO_17_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_17_INTERRUPT_STATUS_A>);
impl REG_GPIO_17_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_17_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_17_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_17_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_17_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_17_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_17_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_17_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_17_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_18_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_18_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_18_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_interrupt_status` reader - Interrupt status register for GPIO18."]
pub struct REG_GPIO_18_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_18_INTERRUPT_STATUS_A>);
impl REG_GPIO_18_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_18_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_18_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_18_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_18_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_18_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_18_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_18_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_18_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_19_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_19_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_19_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_interrupt_status` reader - Interrupt status register for GPIO19."]
pub struct REG_GPIO_19_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_19_INTERRUPT_STATUS_A>);
impl REG_GPIO_19_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_19_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_19_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_19_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_19_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_19_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_19_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_19_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_19_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_20_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_20_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_20_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_interrupt_status` reader - Interrupt status register for GPIO20."]
pub struct REG_GPIO_20_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_20_INTERRUPT_STATUS_A>);
impl REG_GPIO_20_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_20_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_20_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_20_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_20_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_20_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_20_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_20_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_20_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_21_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_21_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_21_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_interrupt_status` reader - Interrupt status register for GPIO21."]
pub struct REG_GPIO_21_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_21_INTERRUPT_STATUS_A>);
impl REG_GPIO_21_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_21_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_21_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_21_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_21_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_21_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_21_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_21_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_21_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_22_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_22_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_22_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_interrupt_status` reader - Interrupt status register for GPIO22."]
pub struct REG_GPIO_22_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_22_INTERRUPT_STATUS_A>);
impl REG_GPIO_22_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_22_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_22_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_22_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_22_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_22_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_22_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_22_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_23_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_23_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_23_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_interrupt_status` reader - Interrupt status register for GPIO23."]
pub struct REG_GPIO_23_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_23_INTERRUPT_STATUS_A>);
impl REG_GPIO_23_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_23_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_23_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_23_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_23_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_23_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_23_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_23_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_23_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_24_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_24_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_24_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_interrupt_status` reader - Interrupt status register for GPIO24."]
pub struct REG_GPIO_24_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_24_INTERRUPT_STATUS_A>);
impl REG_GPIO_24_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_24_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_24_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_24_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_24_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_24_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_24_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_24_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_24_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_25_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_25_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_25_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_interrupt_status` reader - Interrupt status register for GPIO25."]
pub struct REG_GPIO_25_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_25_INTERRUPT_STATUS_A>);
impl REG_GPIO_25_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_25_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_25_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_25_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_25_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_25_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_25_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_25_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_25_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_26_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_26_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_26_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_interrupt_status` reader - Interrupt status register for GPIO26."]
pub struct REG_GPIO_26_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_26_INTERRUPT_STATUS_A>);
impl REG_GPIO_26_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_26_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_26_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_26_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_26_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_26_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_26_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_26_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_26_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_27_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_27_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_27_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_interrupt_status` reader - Interrupt status register for GPIO27."]
pub struct REG_GPIO_27_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_27_INTERRUPT_STATUS_A>);
impl REG_GPIO_27_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_27_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_27_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_27_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_27_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_27_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_27_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_27_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_27_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt status register for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_28_INTERRUPT_STATUS_A {
    #[doc = "0: `0`"]
    RESET = 0,
    #[doc = "1: `1`"]
    SET = 1,
}
impl From<REG_GPIO_28_INTERRUPT_STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_28_INTERRUPT_STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_interrupt_status` reader - Interrupt status register for GPIO28."]
pub struct REG_GPIO_28_INTERRUPT_STATUS_R(crate::FieldReader<bool, REG_GPIO_28_INTERRUPT_STATUS_A>);
impl REG_GPIO_28_INTERRUPT_STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_28_INTERRUPT_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_28_INTERRUPT_STATUS_A {
        match self.bits {
            false => REG_GPIO_28_INTERRUPT_STATUS_A::RESET,
            true => REG_GPIO_28_INTERRUPT_STATUS_A::SET,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        **self == REG_GPIO_28_INTERRUPT_STATUS_A::RESET
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == REG_GPIO_28_INTERRUPT_STATUS_A::SET
    }
}
impl core::ops::Deref for REG_GPIO_28_INTERRUPT_STATUS_R {
    type Target = crate::FieldReader<bool, REG_GPIO_28_INTERRUPT_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt status register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_status(&self) -> REG_GPIO_0_INTERRUPT_STATUS_R {
        REG_GPIO_0_INTERRUPT_STATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt status register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_status(&self) -> REG_GPIO_1_INTERRUPT_STATUS_R {
        REG_GPIO_1_INTERRUPT_STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt status register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_status(&self) -> REG_GPIO_2_INTERRUPT_STATUS_R {
        REG_GPIO_2_INTERRUPT_STATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt status register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_status(&self) -> REG_GPIO_3_INTERRUPT_STATUS_R {
        REG_GPIO_3_INTERRUPT_STATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt status register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_status(&self) -> REG_GPIO_4_INTERRUPT_STATUS_R {
        REG_GPIO_4_INTERRUPT_STATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt status register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_status(&self) -> REG_GPIO_5_INTERRUPT_STATUS_R {
        REG_GPIO_5_INTERRUPT_STATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt status register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_status(&self) -> REG_GPIO_6_INTERRUPT_STATUS_R {
        REG_GPIO_6_INTERRUPT_STATUS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt status register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_status(&self) -> REG_GPIO_7_INTERRUPT_STATUS_R {
        REG_GPIO_7_INTERRUPT_STATUS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt status register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_status(&self) -> REG_GPIO_8_INTERRUPT_STATUS_R {
        REG_GPIO_8_INTERRUPT_STATUS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt status register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_status(&self) -> REG_GPIO_9_INTERRUPT_STATUS_R {
        REG_GPIO_9_INTERRUPT_STATUS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt status register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_status(&self) -> REG_GPIO_10_INTERRUPT_STATUS_R {
        REG_GPIO_10_INTERRUPT_STATUS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt status register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_status(&self) -> REG_GPIO_11_INTERRUPT_STATUS_R {
        REG_GPIO_11_INTERRUPT_STATUS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt status register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_status(&self) -> REG_GPIO_12_INTERRUPT_STATUS_R {
        REG_GPIO_12_INTERRUPT_STATUS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt status register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_status(&self) -> REG_GPIO_13_INTERRUPT_STATUS_R {
        REG_GPIO_13_INTERRUPT_STATUS_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt status register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_status(&self) -> REG_GPIO_14_INTERRUPT_STATUS_R {
        REG_GPIO_14_INTERRUPT_STATUS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt status register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_status(&self) -> REG_GPIO_15_INTERRUPT_STATUS_R {
        REG_GPIO_15_INTERRUPT_STATUS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt status register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_status(&self) -> REG_GPIO_16_INTERRUPT_STATUS_R {
        REG_GPIO_16_INTERRUPT_STATUS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt status register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_status(&self) -> REG_GPIO_17_INTERRUPT_STATUS_R {
        REG_GPIO_17_INTERRUPT_STATUS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt status register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_status(&self) -> REG_GPIO_18_INTERRUPT_STATUS_R {
        REG_GPIO_18_INTERRUPT_STATUS_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt status register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_status(&self) -> REG_GPIO_19_INTERRUPT_STATUS_R {
        REG_GPIO_19_INTERRUPT_STATUS_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interrupt status register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_status(&self) -> REG_GPIO_20_INTERRUPT_STATUS_R {
        REG_GPIO_20_INTERRUPT_STATUS_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Interrupt status register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_status(&self) -> REG_GPIO_21_INTERRUPT_STATUS_R {
        REG_GPIO_21_INTERRUPT_STATUS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Interrupt status register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_status(&self) -> REG_GPIO_22_INTERRUPT_STATUS_R {
        REG_GPIO_22_INTERRUPT_STATUS_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Interrupt status register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_status(&self) -> REG_GPIO_23_INTERRUPT_STATUS_R {
        REG_GPIO_23_INTERRUPT_STATUS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Interrupt status register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_status(&self) -> REG_GPIO_24_INTERRUPT_STATUS_R {
        REG_GPIO_24_INTERRUPT_STATUS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Interrupt status register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_status(&self) -> REG_GPIO_25_INTERRUPT_STATUS_R {
        REG_GPIO_25_INTERRUPT_STATUS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Interrupt status register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_status(&self) -> REG_GPIO_26_INTERRUPT_STATUS_R {
        REG_GPIO_26_INTERRUPT_STATUS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Interrupt status register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_status(&self) -> REG_GPIO_27_INTERRUPT_STATUS_R {
        REG_GPIO_27_INTERRUPT_STATUS_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Interrupt status register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_status(&self) -> REG_GPIO_28_INTERRUPT_STATUS_R {
        REG_GPIO_28_INTERRUPT_STATUS_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
#[doc = "Interrupt status register. The SDK limits the GPIO pins to < 32 although the docs do not mention more than 28 GPIO pins.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_stat1](index.html) module"]
pub struct GPIO_INT_STAT1_SPEC;
impl crate::RegisterSpec for GPIO_INT_STAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_stat1::R](R) reader structure"]
impl crate::Readable for GPIO_INT_STAT1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIO_INT_STAT1 to value 0"]
impl crate::Resettable for GPIO_INT_STAT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
