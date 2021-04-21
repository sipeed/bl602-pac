#[doc = "Register `GPIO_INT_MODE_SET3` reader"]
pub struct R(crate::R<GPIO_INT_MODE_SET3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_MODE_SET3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_INT_MODE_SET3_SPEC>> for R {
    fn from(reader: crate::R<GPIO_INT_MODE_SET3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_MODE_SET3` writer"]
pub struct W(crate::W<GPIO_INT_MODE_SET3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_MODE_SET3_SPEC>;
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
impl core::convert::From<crate::W<GPIO_INT_MODE_SET3_SPEC>> for W {
    fn from(writer: crate::W<GPIO_INT_MODE_SET3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt trigger mode register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_20_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO20."]
pub struct REG_GPIO_20_INTERRUPT_TRIGGER_MODE_R(
    crate::FieldReader<u8, REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A>,
);
impl REG_GPIO_20_INTERRUPT_TRIGGER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_20_INTERRUPT_TRIGGER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A {
        match self.bits {
            0 => REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        **self == REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        **self == REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        **self == REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        **self == REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
impl core::ops::Deref for REG_GPIO_20_INTERRUPT_TRIGGER_MODE_R {
    type Target = crate::FieldReader<u8, REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_20_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO20."]
pub struct REG_GPIO_20_INTERRUPT_TRIGGER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_20_INTERRUPT_TRIGGER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(REG_GPIO_20_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Interrupt control mode register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_20_INTERRUPT_CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<REG_GPIO_20_INTERRUPT_CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_20_INTERRUPT_CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_interrupt_control_mode` reader - Interrupt control mode register for GPIO20."]
pub struct REG_GPIO_20_INTERRUPT_CONTROL_MODE_R(
    crate::FieldReader<bool, REG_GPIO_20_INTERRUPT_CONTROL_MODE_A>,
);
impl REG_GPIO_20_INTERRUPT_CONTROL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_20_INTERRUPT_CONTROL_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_20_INTERRUPT_CONTROL_MODE_A {
        match self.bits {
            false => REG_GPIO_20_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS,
            true => REG_GPIO_20_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        **self == REG_GPIO_20_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        **self == REG_GPIO_20_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS
    }
}
impl core::ops::Deref for REG_GPIO_20_INTERRUPT_CONTROL_MODE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_20_INTERRUPT_CONTROL_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_20_interrupt_control_mode` writer - Interrupt control mode register for GPIO20."]
pub struct REG_GPIO_20_INTERRUPT_CONTROL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_20_INTERRUPT_CONTROL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_20_INTERRUPT_CONTROL_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_20_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_20_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS)
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
#[doc = "Interrupt trigger mode register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_21_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO21."]
pub struct REG_GPIO_21_INTERRUPT_TRIGGER_MODE_R(
    crate::FieldReader<u8, REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A>,
);
impl REG_GPIO_21_INTERRUPT_TRIGGER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_21_INTERRUPT_TRIGGER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A {
        match self.bits {
            0 => REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        **self == REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        **self == REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        **self == REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        **self == REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
impl core::ops::Deref for REG_GPIO_21_INTERRUPT_TRIGGER_MODE_R {
    type Target = crate::FieldReader<u8, REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_21_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO21."]
pub struct REG_GPIO_21_INTERRUPT_TRIGGER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_21_INTERRUPT_TRIGGER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(REG_GPIO_21_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Interrupt control mode register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_21_INTERRUPT_CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<REG_GPIO_21_INTERRUPT_CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_21_INTERRUPT_CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_interrupt_control_mode` reader - Interrupt control mode register for GPIO21."]
pub struct REG_GPIO_21_INTERRUPT_CONTROL_MODE_R(
    crate::FieldReader<bool, REG_GPIO_21_INTERRUPT_CONTROL_MODE_A>,
);
impl REG_GPIO_21_INTERRUPT_CONTROL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_21_INTERRUPT_CONTROL_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_21_INTERRUPT_CONTROL_MODE_A {
        match self.bits {
            false => REG_GPIO_21_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS,
            true => REG_GPIO_21_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        **self == REG_GPIO_21_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        **self == REG_GPIO_21_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS
    }
}
impl core::ops::Deref for REG_GPIO_21_INTERRUPT_CONTROL_MODE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_21_INTERRUPT_CONTROL_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_21_interrupt_control_mode` writer - Interrupt control mode register for GPIO21."]
pub struct REG_GPIO_21_INTERRUPT_CONTROL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_21_INTERRUPT_CONTROL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_21_INTERRUPT_CONTROL_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_21_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_21_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS)
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
#[doc = "Interrupt trigger mode register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_22_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO22."]
pub struct REG_GPIO_22_INTERRUPT_TRIGGER_MODE_R(
    crate::FieldReader<u8, REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A>,
);
impl REG_GPIO_22_INTERRUPT_TRIGGER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_22_INTERRUPT_TRIGGER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A {
        match self.bits {
            0 => REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        **self == REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        **self == REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        **self == REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        **self == REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
impl core::ops::Deref for REG_GPIO_22_INTERRUPT_TRIGGER_MODE_R {
    type Target = crate::FieldReader<u8, REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO22."]
pub struct REG_GPIO_22_INTERRUPT_TRIGGER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_INTERRUPT_TRIGGER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(REG_GPIO_22_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Interrupt control mode register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_22_INTERRUPT_CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<REG_GPIO_22_INTERRUPT_CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_22_INTERRUPT_CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_interrupt_control_mode` reader - Interrupt control mode register for GPIO22."]
pub struct REG_GPIO_22_INTERRUPT_CONTROL_MODE_R(
    crate::FieldReader<bool, REG_GPIO_22_INTERRUPT_CONTROL_MODE_A>,
);
impl REG_GPIO_22_INTERRUPT_CONTROL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_INTERRUPT_CONTROL_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_22_INTERRUPT_CONTROL_MODE_A {
        match self.bits {
            false => REG_GPIO_22_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS,
            true => REG_GPIO_22_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        **self == REG_GPIO_22_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        **self == REG_GPIO_22_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS
    }
}
impl core::ops::Deref for REG_GPIO_22_INTERRUPT_CONTROL_MODE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_22_INTERRUPT_CONTROL_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_interrupt_control_mode` writer - Interrupt control mode register for GPIO22."]
pub struct REG_GPIO_22_INTERRUPT_CONTROL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_INTERRUPT_CONTROL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_22_INTERRUPT_CONTROL_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_22_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_22_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS)
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
#[doc = "Interrupt trigger mode register for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_23_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO23."]
pub struct REG_GPIO_23_INTERRUPT_TRIGGER_MODE_R(
    crate::FieldReader<u8, REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A>,
);
impl REG_GPIO_23_INTERRUPT_TRIGGER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_23_INTERRUPT_TRIGGER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A {
        match self.bits {
            0 => REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        **self == REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        **self == REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        **self == REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        **self == REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
impl core::ops::Deref for REG_GPIO_23_INTERRUPT_TRIGGER_MODE_R {
    type Target = crate::FieldReader<u8, REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_23_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO23."]
pub struct REG_GPIO_23_INTERRUPT_TRIGGER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_23_INTERRUPT_TRIGGER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(REG_GPIO_23_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 9)) | ((value as u32 & 0x03) << 9);
        self.w
    }
}
#[doc = "Interrupt control mode register for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_23_INTERRUPT_CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<REG_GPIO_23_INTERRUPT_CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_23_INTERRUPT_CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_interrupt_control_mode` reader - Interrupt control mode register for GPIO23."]
pub struct REG_GPIO_23_INTERRUPT_CONTROL_MODE_R(
    crate::FieldReader<bool, REG_GPIO_23_INTERRUPT_CONTROL_MODE_A>,
);
impl REG_GPIO_23_INTERRUPT_CONTROL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_23_INTERRUPT_CONTROL_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_23_INTERRUPT_CONTROL_MODE_A {
        match self.bits {
            false => REG_GPIO_23_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS,
            true => REG_GPIO_23_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        **self == REG_GPIO_23_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        **self == REG_GPIO_23_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS
    }
}
impl core::ops::Deref for REG_GPIO_23_INTERRUPT_CONTROL_MODE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_23_INTERRUPT_CONTROL_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_23_interrupt_control_mode` writer - Interrupt control mode register for GPIO23."]
pub struct REG_GPIO_23_INTERRUPT_CONTROL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_23_INTERRUPT_CONTROL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_23_INTERRUPT_CONTROL_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_23_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_23_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS)
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
#[doc = "Interrupt trigger mode register for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_24_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO24."]
pub struct REG_GPIO_24_INTERRUPT_TRIGGER_MODE_R(
    crate::FieldReader<u8, REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A>,
);
impl REG_GPIO_24_INTERRUPT_TRIGGER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_24_INTERRUPT_TRIGGER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A {
        match self.bits {
            0 => REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        **self == REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        **self == REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        **self == REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        **self == REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
impl core::ops::Deref for REG_GPIO_24_INTERRUPT_TRIGGER_MODE_R {
    type Target = crate::FieldReader<u8, REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_24_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO24."]
pub struct REG_GPIO_24_INTERRUPT_TRIGGER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_24_INTERRUPT_TRIGGER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(REG_GPIO_24_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Interrupt control mode register for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_24_INTERRUPT_CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<REG_GPIO_24_INTERRUPT_CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_24_INTERRUPT_CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_interrupt_control_mode` reader - Interrupt control mode register for GPIO24."]
pub struct REG_GPIO_24_INTERRUPT_CONTROL_MODE_R(
    crate::FieldReader<bool, REG_GPIO_24_INTERRUPT_CONTROL_MODE_A>,
);
impl REG_GPIO_24_INTERRUPT_CONTROL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_24_INTERRUPT_CONTROL_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_24_INTERRUPT_CONTROL_MODE_A {
        match self.bits {
            false => REG_GPIO_24_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS,
            true => REG_GPIO_24_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        **self == REG_GPIO_24_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        **self == REG_GPIO_24_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS
    }
}
impl core::ops::Deref for REG_GPIO_24_INTERRUPT_CONTROL_MODE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_24_INTERRUPT_CONTROL_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_24_interrupt_control_mode` writer - Interrupt control mode register for GPIO24."]
pub struct REG_GPIO_24_INTERRUPT_CONTROL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_24_INTERRUPT_CONTROL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_24_INTERRUPT_CONTROL_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_24_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_24_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS)
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
#[doc = "Interrupt trigger mode register for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_25_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO25."]
pub struct REG_GPIO_25_INTERRUPT_TRIGGER_MODE_R(
    crate::FieldReader<u8, REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A>,
);
impl REG_GPIO_25_INTERRUPT_TRIGGER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_25_INTERRUPT_TRIGGER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A {
        match self.bits {
            0 => REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        **self == REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        **self == REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        **self == REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        **self == REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
impl core::ops::Deref for REG_GPIO_25_INTERRUPT_TRIGGER_MODE_R {
    type Target = crate::FieldReader<u8, REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_25_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO25."]
pub struct REG_GPIO_25_INTERRUPT_TRIGGER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_25_INTERRUPT_TRIGGER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(REG_GPIO_25_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | ((value as u32 & 0x03) << 15);
        self.w
    }
}
#[doc = "Interrupt control mode register for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_25_INTERRUPT_CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<REG_GPIO_25_INTERRUPT_CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_25_INTERRUPT_CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_interrupt_control_mode` reader - Interrupt control mode register for GPIO25."]
pub struct REG_GPIO_25_INTERRUPT_CONTROL_MODE_R(
    crate::FieldReader<bool, REG_GPIO_25_INTERRUPT_CONTROL_MODE_A>,
);
impl REG_GPIO_25_INTERRUPT_CONTROL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_25_INTERRUPT_CONTROL_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_25_INTERRUPT_CONTROL_MODE_A {
        match self.bits {
            false => REG_GPIO_25_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS,
            true => REG_GPIO_25_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        **self == REG_GPIO_25_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        **self == REG_GPIO_25_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS
    }
}
impl core::ops::Deref for REG_GPIO_25_INTERRUPT_CONTROL_MODE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_25_INTERRUPT_CONTROL_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_25_interrupt_control_mode` writer - Interrupt control mode register for GPIO25."]
pub struct REG_GPIO_25_INTERRUPT_CONTROL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_25_INTERRUPT_CONTROL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_25_INTERRUPT_CONTROL_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_25_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_25_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS)
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
#[doc = "Interrupt trigger mode register for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_26_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO26."]
pub struct REG_GPIO_26_INTERRUPT_TRIGGER_MODE_R(
    crate::FieldReader<u8, REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A>,
);
impl REG_GPIO_26_INTERRUPT_TRIGGER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_26_INTERRUPT_TRIGGER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A {
        match self.bits {
            0 => REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        **self == REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        **self == REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        **self == REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        **self == REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
impl core::ops::Deref for REG_GPIO_26_INTERRUPT_TRIGGER_MODE_R {
    type Target = crate::FieldReader<u8, REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_26_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO26."]
pub struct REG_GPIO_26_INTERRUPT_TRIGGER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_INTERRUPT_TRIGGER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(REG_GPIO_26_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Interrupt control mode register for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_26_INTERRUPT_CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<REG_GPIO_26_INTERRUPT_CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_26_INTERRUPT_CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_interrupt_control_mode` reader - Interrupt control mode register for GPIO26."]
pub struct REG_GPIO_26_INTERRUPT_CONTROL_MODE_R(
    crate::FieldReader<bool, REG_GPIO_26_INTERRUPT_CONTROL_MODE_A>,
);
impl REG_GPIO_26_INTERRUPT_CONTROL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_26_INTERRUPT_CONTROL_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_26_INTERRUPT_CONTROL_MODE_A {
        match self.bits {
            false => REG_GPIO_26_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS,
            true => REG_GPIO_26_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        **self == REG_GPIO_26_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        **self == REG_GPIO_26_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS
    }
}
impl core::ops::Deref for REG_GPIO_26_INTERRUPT_CONTROL_MODE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_26_INTERRUPT_CONTROL_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_26_interrupt_control_mode` writer - Interrupt control mode register for GPIO26."]
pub struct REG_GPIO_26_INTERRUPT_CONTROL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_INTERRUPT_CONTROL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_26_INTERRUPT_CONTROL_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_26_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_26_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS)
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
#[doc = "Interrupt trigger mode register for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_27_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO27."]
pub struct REG_GPIO_27_INTERRUPT_TRIGGER_MODE_R(
    crate::FieldReader<u8, REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A>,
);
impl REG_GPIO_27_INTERRUPT_TRIGGER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_27_INTERRUPT_TRIGGER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A {
        match self.bits {
            0 => REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        **self == REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        **self == REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        **self == REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        **self == REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
impl core::ops::Deref for REG_GPIO_27_INTERRUPT_TRIGGER_MODE_R {
    type Target = crate::FieldReader<u8, REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_27_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO27."]
pub struct REG_GPIO_27_INTERRUPT_TRIGGER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_INTERRUPT_TRIGGER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(REG_GPIO_27_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | ((value as u32 & 0x03) << 21);
        self.w
    }
}
#[doc = "Interrupt control mode register for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_27_INTERRUPT_CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<REG_GPIO_27_INTERRUPT_CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_27_INTERRUPT_CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_interrupt_control_mode` reader - Interrupt control mode register for GPIO27."]
pub struct REG_GPIO_27_INTERRUPT_CONTROL_MODE_R(
    crate::FieldReader<bool, REG_GPIO_27_INTERRUPT_CONTROL_MODE_A>,
);
impl REG_GPIO_27_INTERRUPT_CONTROL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_27_INTERRUPT_CONTROL_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_27_INTERRUPT_CONTROL_MODE_A {
        match self.bits {
            false => REG_GPIO_27_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS,
            true => REG_GPIO_27_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        **self == REG_GPIO_27_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        **self == REG_GPIO_27_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS
    }
}
impl core::ops::Deref for REG_GPIO_27_INTERRUPT_CONTROL_MODE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_27_INTERRUPT_CONTROL_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_27_interrupt_control_mode` writer - Interrupt control mode register for GPIO27."]
pub struct REG_GPIO_27_INTERRUPT_CONTROL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_INTERRUPT_CONTROL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_27_INTERRUPT_CONTROL_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_27_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_27_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS)
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
#[doc = "Interrupt trigger mode register for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_28_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO28."]
pub struct REG_GPIO_28_INTERRUPT_TRIGGER_MODE_R(
    crate::FieldReader<u8, REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A>,
);
impl REG_GPIO_28_INTERRUPT_TRIGGER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_28_INTERRUPT_TRIGGER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A {
        match self.bits {
            0 => REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        **self == REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        **self == REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        **self == REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        **self == REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
impl core::ops::Deref for REG_GPIO_28_INTERRUPT_TRIGGER_MODE_R {
    type Target = crate::FieldReader<u8, REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_28_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO28."]
pub struct REG_GPIO_28_INTERRUPT_TRIGGER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_28_INTERRUPT_TRIGGER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(REG_GPIO_28_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Interrupt control mode register for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_28_INTERRUPT_CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<REG_GPIO_28_INTERRUPT_CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_28_INTERRUPT_CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_interrupt_control_mode` reader - Interrupt control mode register for GPIO28."]
pub struct REG_GPIO_28_INTERRUPT_CONTROL_MODE_R(
    crate::FieldReader<bool, REG_GPIO_28_INTERRUPT_CONTROL_MODE_A>,
);
impl REG_GPIO_28_INTERRUPT_CONTROL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_28_INTERRUPT_CONTROL_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_28_INTERRUPT_CONTROL_MODE_A {
        match self.bits {
            false => REG_GPIO_28_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS,
            true => REG_GPIO_28_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        **self == REG_GPIO_28_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        **self == REG_GPIO_28_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS
    }
}
impl core::ops::Deref for REG_GPIO_28_INTERRUPT_CONTROL_MODE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_28_INTERRUPT_CONTROL_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_28_interrupt_control_mode` writer - Interrupt control mode register for GPIO28."]
pub struct REG_GPIO_28_INTERRUPT_CONTROL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_28_INTERRUPT_CONTROL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_28_INTERRUPT_CONTROL_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_28_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_28_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS)
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
#[doc = "Interrupt trigger mode register for GPIO29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A {
    #[doc = "0: `0`"]
    NEGATIVE_PULSE = 0,
    #[doc = "1: `1`"]
    POSITIVE_PULSE = 1,
    #[doc = "2: `10`"]
    NEGATIVE_LEVEL = 2,
    #[doc = "3: `11`"]
    POSITIVE_LEVEL = 3,
}
impl From<REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_29_interrupt_trigger_mode` reader - Interrupt trigger mode register for GPIO29."]
pub struct REG_GPIO_29_INTERRUPT_TRIGGER_MODE_R(
    crate::FieldReader<u8, REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A>,
);
impl REG_GPIO_29_INTERRUPT_TRIGGER_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_29_INTERRUPT_TRIGGER_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A {
        match self.bits {
            0 => REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE,
            1 => REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE,
            2 => REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL,
            3 => REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_PULSE`"]
    #[inline(always)]
    pub fn is_negative_pulse(&self) -> bool {
        **self == REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `POSITIVE_PULSE`"]
    #[inline(always)]
    pub fn is_positive_pulse(&self) -> bool {
        **self == REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE
    }
    #[doc = "Checks if the value of the field is `NEGATIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_negative_level(&self) -> bool {
        **self == REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL
    }
    #[doc = "Checks if the value of the field is `POSITIVE_LEVEL`"]
    #[inline(always)]
    pub fn is_positive_level(&self) -> bool {
        **self == REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL
    }
}
impl core::ops::Deref for REG_GPIO_29_INTERRUPT_TRIGGER_MODE_R {
    type Target = crate::FieldReader<u8, REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_29_interrupt_trigger_mode` writer - Interrupt trigger mode register for GPIO29."]
pub struct REG_GPIO_29_INTERRUPT_TRIGGER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_29_INTERRUPT_TRIGGER_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn negative_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_PULSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn positive_pulse(self) -> &'a mut W {
        self.variant(REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::POSITIVE_PULSE)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn negative_level(self) -> &'a mut W {
        self.variant(REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::NEGATIVE_LEVEL)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn positive_level(self) -> &'a mut W {
        self.variant(REG_GPIO_29_INTERRUPT_TRIGGER_MODE_A::POSITIVE_LEVEL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 27)) | ((value as u32 & 0x03) << 27);
        self.w
    }
}
#[doc = "Interrupt control mode register for GPIO29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_29_INTERRUPT_CONTROL_MODE_A {
    #[doc = "0: `0`"]
    SYNCHRONOUS = 0,
    #[doc = "1: `1`"]
    ASYNCHRONOUS = 1,
}
impl From<REG_GPIO_29_INTERRUPT_CONTROL_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_29_INTERRUPT_CONTROL_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_29_interrupt_control_mode` reader - Interrupt control mode register for GPIO29."]
pub struct REG_GPIO_29_INTERRUPT_CONTROL_MODE_R(
    crate::FieldReader<bool, REG_GPIO_29_INTERRUPT_CONTROL_MODE_A>,
);
impl REG_GPIO_29_INTERRUPT_CONTROL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_29_INTERRUPT_CONTROL_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_29_INTERRUPT_CONTROL_MODE_A {
        match self.bits {
            false => REG_GPIO_29_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS,
            true => REG_GPIO_29_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_synchronous(&self) -> bool {
        **self == REG_GPIO_29_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        **self == REG_GPIO_29_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS
    }
}
impl core::ops::Deref for REG_GPIO_29_INTERRUPT_CONTROL_MODE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_29_INTERRUPT_CONTROL_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_29_interrupt_control_mode` writer - Interrupt control mode register for GPIO29."]
pub struct REG_GPIO_29_INTERRUPT_CONTROL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_29_INTERRUPT_CONTROL_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_29_INTERRUPT_CONTROL_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn synchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_29_INTERRUPT_CONTROL_MODE_A::SYNCHRONOUS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(REG_GPIO_29_INTERRUPT_CONTROL_MODE_A::ASYNCHRONOUS)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_trigger_mode(&self) -> REG_GPIO_20_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_20_INTERRUPT_TRIGGER_MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_control_mode(&self) -> REG_GPIO_20_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_20_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_trigger_mode(&self) -> REG_GPIO_21_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_21_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_control_mode(&self) -> REG_GPIO_21_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_21_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_trigger_mode(&self) -> REG_GPIO_22_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_22_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_control_mode(&self) -> REG_GPIO_22_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_22_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_trigger_mode(&self) -> REG_GPIO_23_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_23_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 9) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_control_mode(&self) -> REG_GPIO_23_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_23_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_trigger_mode(&self) -> REG_GPIO_24_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_24_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_control_mode(&self) -> REG_GPIO_24_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_24_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_trigger_mode(&self) -> REG_GPIO_25_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_25_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_control_mode(&self) -> REG_GPIO_25_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_25_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_trigger_mode(&self) -> REG_GPIO_26_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_26_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_control_mode(&self) -> REG_GPIO_26_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_26_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_trigger_mode(&self) -> REG_GPIO_27_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_27_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_control_mode(&self) -> REG_GPIO_27_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_27_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_trigger_mode(&self) -> REG_GPIO_28_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_28_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_control_mode(&self) -> REG_GPIO_28_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_28_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO29."]
    #[inline(always)]
    pub fn reg_gpio_29_interrupt_trigger_mode(&self) -> REG_GPIO_29_INTERRUPT_TRIGGER_MODE_R {
        REG_GPIO_29_INTERRUPT_TRIGGER_MODE_R::new(((self.bits >> 27) & 0x03) as u8)
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO29."]
    #[inline(always)]
    pub fn reg_gpio_29_interrupt_control_mode(&self) -> REG_GPIO_29_INTERRUPT_CONTROL_MODE_R {
        REG_GPIO_29_INTERRUPT_CONTROL_MODE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interrupt trigger mode register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_trigger_mode(&mut self) -> REG_GPIO_20_INTERRUPT_TRIGGER_MODE_W {
        REG_GPIO_20_INTERRUPT_TRIGGER_MODE_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt control mode register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_control_mode(&mut self) -> REG_GPIO_20_INTERRUPT_CONTROL_MODE_W {
        REG_GPIO_20_INTERRUPT_CONTROL_MODE_W { w: self }
    }
    #[doc = "Bits 3:4 - Interrupt trigger mode register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_trigger_mode(&mut self) -> REG_GPIO_21_INTERRUPT_TRIGGER_MODE_W {
        REG_GPIO_21_INTERRUPT_TRIGGER_MODE_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt control mode register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_control_mode(&mut self) -> REG_GPIO_21_INTERRUPT_CONTROL_MODE_W {
        REG_GPIO_21_INTERRUPT_CONTROL_MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Interrupt trigger mode register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_trigger_mode(&mut self) -> REG_GPIO_22_INTERRUPT_TRIGGER_MODE_W {
        REG_GPIO_22_INTERRUPT_TRIGGER_MODE_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt control mode register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_control_mode(&mut self) -> REG_GPIO_22_INTERRUPT_CONTROL_MODE_W {
        REG_GPIO_22_INTERRUPT_CONTROL_MODE_W { w: self }
    }
    #[doc = "Bits 9:10 - Interrupt trigger mode register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_trigger_mode(&mut self) -> REG_GPIO_23_INTERRUPT_TRIGGER_MODE_W {
        REG_GPIO_23_INTERRUPT_TRIGGER_MODE_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt control mode register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_control_mode(&mut self) -> REG_GPIO_23_INTERRUPT_CONTROL_MODE_W {
        REG_GPIO_23_INTERRUPT_CONTROL_MODE_W { w: self }
    }
    #[doc = "Bits 12:13 - Interrupt trigger mode register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_trigger_mode(&mut self) -> REG_GPIO_24_INTERRUPT_TRIGGER_MODE_W {
        REG_GPIO_24_INTERRUPT_TRIGGER_MODE_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt control mode register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_control_mode(&mut self) -> REG_GPIO_24_INTERRUPT_CONTROL_MODE_W {
        REG_GPIO_24_INTERRUPT_CONTROL_MODE_W { w: self }
    }
    #[doc = "Bits 15:16 - Interrupt trigger mode register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_trigger_mode(&mut self) -> REG_GPIO_25_INTERRUPT_TRIGGER_MODE_W {
        REG_GPIO_25_INTERRUPT_TRIGGER_MODE_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt control mode register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_control_mode(&mut self) -> REG_GPIO_25_INTERRUPT_CONTROL_MODE_W {
        REG_GPIO_25_INTERRUPT_CONTROL_MODE_W { w: self }
    }
    #[doc = "Bits 18:19 - Interrupt trigger mode register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_trigger_mode(&mut self) -> REG_GPIO_26_INTERRUPT_TRIGGER_MODE_W {
        REG_GPIO_26_INTERRUPT_TRIGGER_MODE_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt control mode register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_control_mode(&mut self) -> REG_GPIO_26_INTERRUPT_CONTROL_MODE_W {
        REG_GPIO_26_INTERRUPT_CONTROL_MODE_W { w: self }
    }
    #[doc = "Bits 21:22 - Interrupt trigger mode register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_trigger_mode(&mut self) -> REG_GPIO_27_INTERRUPT_TRIGGER_MODE_W {
        REG_GPIO_27_INTERRUPT_TRIGGER_MODE_W { w: self }
    }
    #[doc = "Bit 23 - Interrupt control mode register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_control_mode(&mut self) -> REG_GPIO_27_INTERRUPT_CONTROL_MODE_W {
        REG_GPIO_27_INTERRUPT_CONTROL_MODE_W { w: self }
    }
    #[doc = "Bits 24:25 - Interrupt trigger mode register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_trigger_mode(&mut self) -> REG_GPIO_28_INTERRUPT_TRIGGER_MODE_W {
        REG_GPIO_28_INTERRUPT_TRIGGER_MODE_W { w: self }
    }
    #[doc = "Bit 26 - Interrupt control mode register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_control_mode(&mut self) -> REG_GPIO_28_INTERRUPT_CONTROL_MODE_W {
        REG_GPIO_28_INTERRUPT_CONTROL_MODE_W { w: self }
    }
    #[doc = "Bits 27:28 - Interrupt trigger mode register for GPIO29."]
    #[inline(always)]
    pub fn reg_gpio_29_interrupt_trigger_mode(&mut self) -> REG_GPIO_29_INTERRUPT_TRIGGER_MODE_W {
        REG_GPIO_29_INTERRUPT_TRIGGER_MODE_W { w: self }
    }
    #[doc = "Bit 29 - Interrupt control mode register for GPIO29."]
    #[inline(always)]
    pub fn reg_gpio_29_interrupt_control_mode(&mut self) -> REG_GPIO_29_INTERRUPT_CONTROL_MODE_W {
        REG_GPIO_29_INTERRUPT_CONTROL_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO interrupt trigger and control register for GPIO20-GPIO29.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_mode_set3](index.html) module"]
pub struct GPIO_INT_MODE_SET3_SPEC;
impl crate::RegisterSpec for GPIO_INT_MODE_SET3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_mode_set3::R](R) reader structure"]
impl crate::Readable for GPIO_INT_MODE_SET3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_mode_set3::W](W) writer structure"]
impl crate::Writable for GPIO_INT_MODE_SET3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INT_MODE_SET3 to value 0"]
impl crate::Resettable for GPIO_INT_MODE_SET3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
