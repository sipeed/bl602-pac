#[doc = "Register `GPIO_INT_CLR1` reader"]
pub struct R(crate::R<GPIO_INT_CLR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_INT_CLR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_INT_CLR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_INT_CLR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_INT_CLR1` writer"]
pub struct W(crate::W<GPIO_INT_CLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_INT_CLR1_SPEC>;
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
impl From<crate::W<GPIO_INT_CLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_INT_CLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt clearing register for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_0_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_0_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_0_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_interrupt_clear` reader - Interrupt clearing register for GPIO0."]
pub struct REG_GPIO_0_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_0_INTERRUPT_CLEAR_A>);
impl REG_GPIO_0_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_0_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_0_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_0_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_0_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_0_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_0_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_0_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_0_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_0_interrupt_clear` writer - Interrupt clearing register for GPIO0."]
pub struct REG_GPIO_0_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_0_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_0_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_0_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_1_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_1_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_1_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_interrupt_clear` reader - Interrupt clearing register for GPIO1."]
pub struct REG_GPIO_1_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_1_INTERRUPT_CLEAR_A>);
impl REG_GPIO_1_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_1_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_1_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_1_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_1_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_1_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_1_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_1_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_1_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_1_interrupt_clear` writer - Interrupt clearing register for GPIO1."]
pub struct REG_GPIO_1_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_1_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_1_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_1_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_2_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_2_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_2_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_2_interrupt_clear` reader - Interrupt clearing register for GPIO2."]
pub struct REG_GPIO_2_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_2_INTERRUPT_CLEAR_A>);
impl REG_GPIO_2_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_2_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_2_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_2_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_2_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_2_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_2_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_2_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_2_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_2_interrupt_clear` writer - Interrupt clearing register for GPIO2."]
pub struct REG_GPIO_2_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_2_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_2_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_2_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_2_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_3_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_3_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_3_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_3_interrupt_clear` reader - Interrupt clearing register for GPIO3."]
pub struct REG_GPIO_3_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_3_INTERRUPT_CLEAR_A>);
impl REG_GPIO_3_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_3_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_3_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_3_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_3_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_3_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_3_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_3_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_3_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_3_interrupt_clear` writer - Interrupt clearing register for GPIO3."]
pub struct REG_GPIO_3_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_3_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_3_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_3_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_3_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_4_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_4_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_4_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_4_interrupt_clear` reader - Interrupt clearing register for GPIO4."]
pub struct REG_GPIO_4_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_4_INTERRUPT_CLEAR_A>);
impl REG_GPIO_4_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_4_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_4_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_4_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_4_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_4_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_4_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_4_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_4_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_4_interrupt_clear` writer - Interrupt clearing register for GPIO4."]
pub struct REG_GPIO_4_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_4_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_4_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_4_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_4_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_5_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_5_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_5_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_5_interrupt_clear` reader - Interrupt clearing register for GPIO5."]
pub struct REG_GPIO_5_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_5_INTERRUPT_CLEAR_A>);
impl REG_GPIO_5_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_5_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_5_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_5_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_5_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_5_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_5_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_5_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_5_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_5_interrupt_clear` writer - Interrupt clearing register for GPIO5."]
pub struct REG_GPIO_5_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_5_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_5_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_5_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_5_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_6_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_6_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_6_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_6_interrupt_clear` reader - Interrupt clearing register for GPIO6."]
pub struct REG_GPIO_6_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_6_INTERRUPT_CLEAR_A>);
impl REG_GPIO_6_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_6_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_6_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_6_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_6_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_6_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_6_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_6_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_6_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_6_interrupt_clear` writer - Interrupt clearing register for GPIO6."]
pub struct REG_GPIO_6_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_6_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_6_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_6_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_6_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_7_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_7_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_7_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_7_interrupt_clear` reader - Interrupt clearing register for GPIO7."]
pub struct REG_GPIO_7_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_7_INTERRUPT_CLEAR_A>);
impl REG_GPIO_7_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_7_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_7_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_7_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_7_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_7_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_7_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_7_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_7_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_7_interrupt_clear` writer - Interrupt clearing register for GPIO7."]
pub struct REG_GPIO_7_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_7_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_7_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_7_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_7_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_8_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_8_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_8_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_8_interrupt_clear` reader - Interrupt clearing register for GPIO8."]
pub struct REG_GPIO_8_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_8_INTERRUPT_CLEAR_A>);
impl REG_GPIO_8_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_8_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_8_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_8_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_8_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_8_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_8_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_8_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_8_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_8_interrupt_clear` writer - Interrupt clearing register for GPIO8."]
pub struct REG_GPIO_8_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_8_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_8_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_8_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_8_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_9_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_9_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_9_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_9_interrupt_clear` reader - Interrupt clearing register for GPIO9."]
pub struct REG_GPIO_9_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_9_INTERRUPT_CLEAR_A>);
impl REG_GPIO_9_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_9_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_9_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_9_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_9_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_9_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_9_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_9_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_9_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_9_interrupt_clear` writer - Interrupt clearing register for GPIO9."]
pub struct REG_GPIO_9_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_9_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_9_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_9_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_9_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_10_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_10_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_10_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_interrupt_clear` reader - Interrupt clearing register for GPIO10."]
pub struct REG_GPIO_10_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_10_INTERRUPT_CLEAR_A>);
impl REG_GPIO_10_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_10_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_10_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_10_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_10_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_10_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_10_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_10_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_10_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_10_interrupt_clear` writer - Interrupt clearing register for GPIO10."]
pub struct REG_GPIO_10_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_10_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_10_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_10_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_11_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_11_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_11_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_interrupt_clear` reader - Interrupt clearing register for GPIO11."]
pub struct REG_GPIO_11_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_11_INTERRUPT_CLEAR_A>);
impl REG_GPIO_11_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_11_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_11_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_11_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_11_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_11_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_11_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_11_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_11_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_11_interrupt_clear` writer - Interrupt clearing register for GPIO11."]
pub struct REG_GPIO_11_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_11_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_11_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_11_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_12_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_12_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_12_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_12_interrupt_clear` reader - Interrupt clearing register for GPIO12."]
pub struct REG_GPIO_12_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_12_INTERRUPT_CLEAR_A>);
impl REG_GPIO_12_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_12_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_12_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_12_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_12_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_12_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_12_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_12_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_12_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_12_interrupt_clear` writer - Interrupt clearing register for GPIO12."]
pub struct REG_GPIO_12_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_12_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_12_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_12_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_12_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_13_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_13_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_13_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_13_interrupt_clear` reader - Interrupt clearing register for GPIO13."]
pub struct REG_GPIO_13_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_13_INTERRUPT_CLEAR_A>);
impl REG_GPIO_13_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_13_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_13_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_13_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_13_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_13_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_13_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_13_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_13_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_13_interrupt_clear` writer - Interrupt clearing register for GPIO13."]
pub struct REG_GPIO_13_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_13_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_13_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_13_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_13_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_14_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_14_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_14_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_14_interrupt_clear` reader - Interrupt clearing register for GPIO14."]
pub struct REG_GPIO_14_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_14_INTERRUPT_CLEAR_A>);
impl REG_GPIO_14_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_14_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_14_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_14_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_14_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_14_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_14_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_14_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_14_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_14_interrupt_clear` writer - Interrupt clearing register for GPIO14."]
pub struct REG_GPIO_14_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_14_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_14_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_14_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_14_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_15_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_15_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_15_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_15_interrupt_clear` reader - Interrupt clearing register for GPIO15."]
pub struct REG_GPIO_15_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_15_INTERRUPT_CLEAR_A>);
impl REG_GPIO_15_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_15_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_15_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_15_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_15_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_15_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_15_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_15_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_15_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_15_interrupt_clear` writer - Interrupt clearing register for GPIO15."]
pub struct REG_GPIO_15_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_15_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_15_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_15_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_15_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_16_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_16_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_16_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_interrupt_clear` reader - Interrupt clearing register for GPIO16."]
pub struct REG_GPIO_16_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_16_INTERRUPT_CLEAR_A>);
impl REG_GPIO_16_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_16_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_16_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_16_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_16_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_16_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_16_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_16_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_16_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_16_interrupt_clear` writer - Interrupt clearing register for GPIO16."]
pub struct REG_GPIO_16_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_16_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_16_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_16_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_17_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_17_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_17_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_interrupt_clear` reader - Interrupt clearing register for GPIO17."]
pub struct REG_GPIO_17_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_17_INTERRUPT_CLEAR_A>);
impl REG_GPIO_17_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_17_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_17_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_17_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_17_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_17_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_17_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_17_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_17_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_17_interrupt_clear` writer - Interrupt clearing register for GPIO17."]
pub struct REG_GPIO_17_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_17_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_17_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_17_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_18_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_18_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_18_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_18_interrupt_clear` reader - Interrupt clearing register for GPIO18."]
pub struct REG_GPIO_18_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_18_INTERRUPT_CLEAR_A>);
impl REG_GPIO_18_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_18_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_18_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_18_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_18_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_18_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_18_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_18_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_18_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_18_interrupt_clear` writer - Interrupt clearing register for GPIO18."]
pub struct REG_GPIO_18_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_18_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_18_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_18_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_18_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_19_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_19_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_19_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_19_interrupt_clear` reader - Interrupt clearing register for GPIO19."]
pub struct REG_GPIO_19_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_19_INTERRUPT_CLEAR_A>);
impl REG_GPIO_19_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_19_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_19_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_19_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_19_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_19_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_19_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_19_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_19_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_19_interrupt_clear` writer - Interrupt clearing register for GPIO19."]
pub struct REG_GPIO_19_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_19_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_19_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_19_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_19_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_20_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_20_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_20_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_20_interrupt_clear` reader - Interrupt clearing register for GPIO20."]
pub struct REG_GPIO_20_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_20_INTERRUPT_CLEAR_A>);
impl REG_GPIO_20_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_20_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_20_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_20_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_20_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_20_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_20_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_20_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_20_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_20_interrupt_clear` writer - Interrupt clearing register for GPIO20."]
pub struct REG_GPIO_20_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_20_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_20_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_20_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_20_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_21_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_21_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_21_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_21_interrupt_clear` reader - Interrupt clearing register for GPIO21."]
pub struct REG_GPIO_21_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_21_INTERRUPT_CLEAR_A>);
impl REG_GPIO_21_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_21_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_21_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_21_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_21_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_21_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_21_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_21_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_21_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_21_interrupt_clear` writer - Interrupt clearing register for GPIO21."]
pub struct REG_GPIO_21_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_21_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_21_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_21_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_21_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_22_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_22_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_22_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_22_interrupt_clear` reader - Interrupt clearing register for GPIO22."]
pub struct REG_GPIO_22_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_22_INTERRUPT_CLEAR_A>);
impl REG_GPIO_22_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_22_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_22_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_22_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_22_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_22_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_22_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_22_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_22_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_22_interrupt_clear` writer - Interrupt clearing register for GPIO22."]
pub struct REG_GPIO_22_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_22_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_22_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_22_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_22_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_23_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_23_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_23_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_23_interrupt_clear` reader - Interrupt clearing register for GPIO23."]
pub struct REG_GPIO_23_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_23_INTERRUPT_CLEAR_A>);
impl REG_GPIO_23_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_23_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_23_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_23_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_23_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_23_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_23_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_23_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_23_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_23_interrupt_clear` writer - Interrupt clearing register for GPIO23."]
pub struct REG_GPIO_23_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_23_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_23_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_23_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_23_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_24_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_24_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_24_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_24_interrupt_clear` reader - Interrupt clearing register for GPIO24."]
pub struct REG_GPIO_24_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_24_INTERRUPT_CLEAR_A>);
impl REG_GPIO_24_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_24_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_24_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_24_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_24_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_24_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_24_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_24_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_24_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_24_interrupt_clear` writer - Interrupt clearing register for GPIO24."]
pub struct REG_GPIO_24_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_24_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_24_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_24_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_24_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_25_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_25_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_25_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_25_interrupt_clear` reader - Interrupt clearing register for GPIO25."]
pub struct REG_GPIO_25_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_25_INTERRUPT_CLEAR_A>);
impl REG_GPIO_25_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_25_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_25_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_25_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_25_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_25_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_25_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_25_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_25_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_25_interrupt_clear` writer - Interrupt clearing register for GPIO25."]
pub struct REG_GPIO_25_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_25_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_25_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_25_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_25_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_26_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_26_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_26_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_interrupt_clear` reader - Interrupt clearing register for GPIO26."]
pub struct REG_GPIO_26_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_26_INTERRUPT_CLEAR_A>);
impl REG_GPIO_26_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_26_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_26_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_26_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_26_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_26_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_26_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_26_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_26_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_26_interrupt_clear` writer - Interrupt clearing register for GPIO26."]
pub struct REG_GPIO_26_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_26_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_26_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_26_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_27_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_27_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_27_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_interrupt_clear` reader - Interrupt clearing register for GPIO27."]
pub struct REG_GPIO_27_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_27_INTERRUPT_CLEAR_A>);
impl REG_GPIO_27_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_27_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_27_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_27_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_27_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_27_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_27_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_27_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_27_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_27_interrupt_clear` writer - Interrupt clearing register for GPIO27."]
pub struct REG_GPIO_27_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_27_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_27_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_27_INTERRUPT_CLEAR_A::CLEAR)
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
#[doc = "Interrupt clearing register for GPIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_28_INTERRUPT_CLEAR_A {
    #[doc = "0: `0`"]
    NO_CLEAR = 0,
    #[doc = "1: `1`"]
    CLEAR = 1,
}
impl From<REG_GPIO_28_INTERRUPT_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_28_INTERRUPT_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_28_interrupt_clear` reader - Interrupt clearing register for GPIO28."]
pub struct REG_GPIO_28_INTERRUPT_CLEAR_R(crate::FieldReader<bool, REG_GPIO_28_INTERRUPT_CLEAR_A>);
impl REG_GPIO_28_INTERRUPT_CLEAR_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_28_INTERRUPT_CLEAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_28_INTERRUPT_CLEAR_A {
        match self.bits {
            false => REG_GPIO_28_INTERRUPT_CLEAR_A::NO_CLEAR,
            true => REG_GPIO_28_INTERRUPT_CLEAR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLEAR`"]
    #[inline(always)]
    pub fn is_no_clear(&self) -> bool {
        **self == REG_GPIO_28_INTERRUPT_CLEAR_A::NO_CLEAR
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == REG_GPIO_28_INTERRUPT_CLEAR_A::CLEAR
    }
}
impl core::ops::Deref for REG_GPIO_28_INTERRUPT_CLEAR_R {
    type Target = crate::FieldReader<bool, REG_GPIO_28_INTERRUPT_CLEAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_28_interrupt_clear` writer - Interrupt clearing register for GPIO28."]
pub struct REG_GPIO_28_INTERRUPT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_28_INTERRUPT_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_28_INTERRUPT_CLEAR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_clear(self) -> &'a mut W {
        self.variant(REG_GPIO_28_INTERRUPT_CLEAR_A::NO_CLEAR)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REG_GPIO_28_INTERRUPT_CLEAR_A::CLEAR)
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
    #[doc = "Bit 0 - Interrupt clearing register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_clear(&self) -> REG_GPIO_0_INTERRUPT_CLEAR_R {
        REG_GPIO_0_INTERRUPT_CLEAR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Interrupt clearing register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_clear(&self) -> REG_GPIO_1_INTERRUPT_CLEAR_R {
        REG_GPIO_1_INTERRUPT_CLEAR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt clearing register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_clear(&self) -> REG_GPIO_2_INTERRUPT_CLEAR_R {
        REG_GPIO_2_INTERRUPT_CLEAR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt clearing register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_clear(&self) -> REG_GPIO_3_INTERRUPT_CLEAR_R {
        REG_GPIO_3_INTERRUPT_CLEAR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Interrupt clearing register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_clear(&self) -> REG_GPIO_4_INTERRUPT_CLEAR_R {
        REG_GPIO_4_INTERRUPT_CLEAR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Interrupt clearing register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_clear(&self) -> REG_GPIO_5_INTERRUPT_CLEAR_R {
        REG_GPIO_5_INTERRUPT_CLEAR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Interrupt clearing register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_clear(&self) -> REG_GPIO_6_INTERRUPT_CLEAR_R {
        REG_GPIO_6_INTERRUPT_CLEAR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Interrupt clearing register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_clear(&self) -> REG_GPIO_7_INTERRUPT_CLEAR_R {
        REG_GPIO_7_INTERRUPT_CLEAR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Interrupt clearing register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_clear(&self) -> REG_GPIO_8_INTERRUPT_CLEAR_R {
        REG_GPIO_8_INTERRUPT_CLEAR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Interrupt clearing register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_clear(&self) -> REG_GPIO_9_INTERRUPT_CLEAR_R {
        REG_GPIO_9_INTERRUPT_CLEAR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Interrupt clearing register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_clear(&self) -> REG_GPIO_10_INTERRUPT_CLEAR_R {
        REG_GPIO_10_INTERRUPT_CLEAR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Interrupt clearing register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_clear(&self) -> REG_GPIO_11_INTERRUPT_CLEAR_R {
        REG_GPIO_11_INTERRUPT_CLEAR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Interrupt clearing register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_clear(&self) -> REG_GPIO_12_INTERRUPT_CLEAR_R {
        REG_GPIO_12_INTERRUPT_CLEAR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Interrupt clearing register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_clear(&self) -> REG_GPIO_13_INTERRUPT_CLEAR_R {
        REG_GPIO_13_INTERRUPT_CLEAR_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Interrupt clearing register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_clear(&self) -> REG_GPIO_14_INTERRUPT_CLEAR_R {
        REG_GPIO_14_INTERRUPT_CLEAR_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Interrupt clearing register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_clear(&self) -> REG_GPIO_15_INTERRUPT_CLEAR_R {
        REG_GPIO_15_INTERRUPT_CLEAR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Interrupt clearing register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_clear(&self) -> REG_GPIO_16_INTERRUPT_CLEAR_R {
        REG_GPIO_16_INTERRUPT_CLEAR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Interrupt clearing register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_clear(&self) -> REG_GPIO_17_INTERRUPT_CLEAR_R {
        REG_GPIO_17_INTERRUPT_CLEAR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Interrupt clearing register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_clear(&self) -> REG_GPIO_18_INTERRUPT_CLEAR_R {
        REG_GPIO_18_INTERRUPT_CLEAR_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Interrupt clearing register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_clear(&self) -> REG_GPIO_19_INTERRUPT_CLEAR_R {
        REG_GPIO_19_INTERRUPT_CLEAR_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Interrupt clearing register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_clear(&self) -> REG_GPIO_20_INTERRUPT_CLEAR_R {
        REG_GPIO_20_INTERRUPT_CLEAR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Interrupt clearing register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_clear(&self) -> REG_GPIO_21_INTERRUPT_CLEAR_R {
        REG_GPIO_21_INTERRUPT_CLEAR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Interrupt clearing register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_clear(&self) -> REG_GPIO_22_INTERRUPT_CLEAR_R {
        REG_GPIO_22_INTERRUPT_CLEAR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Interrupt clearing register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_clear(&self) -> REG_GPIO_23_INTERRUPT_CLEAR_R {
        REG_GPIO_23_INTERRUPT_CLEAR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Interrupt clearing register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_clear(&self) -> REG_GPIO_24_INTERRUPT_CLEAR_R {
        REG_GPIO_24_INTERRUPT_CLEAR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Interrupt clearing register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_clear(&self) -> REG_GPIO_25_INTERRUPT_CLEAR_R {
        REG_GPIO_25_INTERRUPT_CLEAR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Interrupt clearing register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_clear(&self) -> REG_GPIO_26_INTERRUPT_CLEAR_R {
        REG_GPIO_26_INTERRUPT_CLEAR_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Interrupt clearing register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_clear(&self) -> REG_GPIO_27_INTERRUPT_CLEAR_R {
        REG_GPIO_27_INTERRUPT_CLEAR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Interrupt clearing register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_clear(&self) -> REG_GPIO_28_INTERRUPT_CLEAR_R {
        REG_GPIO_28_INTERRUPT_CLEAR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt clearing register for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_interrupt_clear(&mut self) -> REG_GPIO_0_INTERRUPT_CLEAR_W {
        REG_GPIO_0_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 1 - Interrupt clearing register for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_interrupt_clear(&mut self) -> REG_GPIO_1_INTERRUPT_CLEAR_W {
        REG_GPIO_1_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 2 - Interrupt clearing register for GPIO2."]
    #[inline(always)]
    pub fn reg_gpio_2_interrupt_clear(&mut self) -> REG_GPIO_2_INTERRUPT_CLEAR_W {
        REG_GPIO_2_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 3 - Interrupt clearing register for GPIO3."]
    #[inline(always)]
    pub fn reg_gpio_3_interrupt_clear(&mut self) -> REG_GPIO_3_INTERRUPT_CLEAR_W {
        REG_GPIO_3_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 4 - Interrupt clearing register for GPIO4."]
    #[inline(always)]
    pub fn reg_gpio_4_interrupt_clear(&mut self) -> REG_GPIO_4_INTERRUPT_CLEAR_W {
        REG_GPIO_4_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt clearing register for GPIO5."]
    #[inline(always)]
    pub fn reg_gpio_5_interrupt_clear(&mut self) -> REG_GPIO_5_INTERRUPT_CLEAR_W {
        REG_GPIO_5_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 6 - Interrupt clearing register for GPIO6."]
    #[inline(always)]
    pub fn reg_gpio_6_interrupt_clear(&mut self) -> REG_GPIO_6_INTERRUPT_CLEAR_W {
        REG_GPIO_6_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 7 - Interrupt clearing register for GPIO7."]
    #[inline(always)]
    pub fn reg_gpio_7_interrupt_clear(&mut self) -> REG_GPIO_7_INTERRUPT_CLEAR_W {
        REG_GPIO_7_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 8 - Interrupt clearing register for GPIO8."]
    #[inline(always)]
    pub fn reg_gpio_8_interrupt_clear(&mut self) -> REG_GPIO_8_INTERRUPT_CLEAR_W {
        REG_GPIO_8_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 9 - Interrupt clearing register for GPIO9."]
    #[inline(always)]
    pub fn reg_gpio_9_interrupt_clear(&mut self) -> REG_GPIO_9_INTERRUPT_CLEAR_W {
        REG_GPIO_9_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 10 - Interrupt clearing register for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_interrupt_clear(&mut self) -> REG_GPIO_10_INTERRUPT_CLEAR_W {
        REG_GPIO_10_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 11 - Interrupt clearing register for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_interrupt_clear(&mut self) -> REG_GPIO_11_INTERRUPT_CLEAR_W {
        REG_GPIO_11_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 12 - Interrupt clearing register for GPIO12."]
    #[inline(always)]
    pub fn reg_gpio_12_interrupt_clear(&mut self) -> REG_GPIO_12_INTERRUPT_CLEAR_W {
        REG_GPIO_12_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 13 - Interrupt clearing register for GPIO13."]
    #[inline(always)]
    pub fn reg_gpio_13_interrupt_clear(&mut self) -> REG_GPIO_13_INTERRUPT_CLEAR_W {
        REG_GPIO_13_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 14 - Interrupt clearing register for GPIO14."]
    #[inline(always)]
    pub fn reg_gpio_14_interrupt_clear(&mut self) -> REG_GPIO_14_INTERRUPT_CLEAR_W {
        REG_GPIO_14_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 15 - Interrupt clearing register for GPIO15."]
    #[inline(always)]
    pub fn reg_gpio_15_interrupt_clear(&mut self) -> REG_GPIO_15_INTERRUPT_CLEAR_W {
        REG_GPIO_15_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 16 - Interrupt clearing register for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_interrupt_clear(&mut self) -> REG_GPIO_16_INTERRUPT_CLEAR_W {
        REG_GPIO_16_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 17 - Interrupt clearing register for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_interrupt_clear(&mut self) -> REG_GPIO_17_INTERRUPT_CLEAR_W {
        REG_GPIO_17_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 18 - Interrupt clearing register for GPIO18."]
    #[inline(always)]
    pub fn reg_gpio_18_interrupt_clear(&mut self) -> REG_GPIO_18_INTERRUPT_CLEAR_W {
        REG_GPIO_18_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 19 - Interrupt clearing register for GPIO19."]
    #[inline(always)]
    pub fn reg_gpio_19_interrupt_clear(&mut self) -> REG_GPIO_19_INTERRUPT_CLEAR_W {
        REG_GPIO_19_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 20 - Interrupt clearing register for GPIO20."]
    #[inline(always)]
    pub fn reg_gpio_20_interrupt_clear(&mut self) -> REG_GPIO_20_INTERRUPT_CLEAR_W {
        REG_GPIO_20_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 21 - Interrupt clearing register for GPIO21."]
    #[inline(always)]
    pub fn reg_gpio_21_interrupt_clear(&mut self) -> REG_GPIO_21_INTERRUPT_CLEAR_W {
        REG_GPIO_21_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 22 - Interrupt clearing register for GPIO22."]
    #[inline(always)]
    pub fn reg_gpio_22_interrupt_clear(&mut self) -> REG_GPIO_22_INTERRUPT_CLEAR_W {
        REG_GPIO_22_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 23 - Interrupt clearing register for GPIO23."]
    #[inline(always)]
    pub fn reg_gpio_23_interrupt_clear(&mut self) -> REG_GPIO_23_INTERRUPT_CLEAR_W {
        REG_GPIO_23_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 24 - Interrupt clearing register for GPIO24."]
    #[inline(always)]
    pub fn reg_gpio_24_interrupt_clear(&mut self) -> REG_GPIO_24_INTERRUPT_CLEAR_W {
        REG_GPIO_24_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 25 - Interrupt clearing register for GPIO25."]
    #[inline(always)]
    pub fn reg_gpio_25_interrupt_clear(&mut self) -> REG_GPIO_25_INTERRUPT_CLEAR_W {
        REG_GPIO_25_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 26 - Interrupt clearing register for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_interrupt_clear(&mut self) -> REG_GPIO_26_INTERRUPT_CLEAR_W {
        REG_GPIO_26_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 27 - Interrupt clearing register for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_interrupt_clear(&mut self) -> REG_GPIO_27_INTERRUPT_CLEAR_W {
        REG_GPIO_27_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Bit 28 - Interrupt clearing register for GPIO28."]
    #[inline(always)]
    pub fn reg_gpio_28_interrupt_clear(&mut self) -> REG_GPIO_28_INTERRUPT_CLEAR_W {
        REG_GPIO_28_INTERRUPT_CLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clearing register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_int_clr1](index.html) module"]
pub struct GPIO_INT_CLR1_SPEC;
impl crate::RegisterSpec for GPIO_INT_CLR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_int_clr1::R](R) reader structure"]
impl crate::Readable for GPIO_INT_CLR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_int_clr1::W](W) writer structure"]
impl crate::Writable for GPIO_INT_CLR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_INT_CLR1 to value 0"]
impl crate::Resettable for GPIO_INT_CLR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
