#[doc = "Register `GPIO_CFGCTL13` reader"]
pub struct R(crate::R<GPIO_CFGCTL13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_CFGCTL13_SPEC>> for R {
    fn from(reader: crate::R<GPIO_CFGCTL13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL13` writer"]
pub struct W(crate::W<GPIO_CFGCTL13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL13_SPEC>;
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
impl core::convert::From<crate::W<GPIO_CFGCTL13_SPEC>> for W {
    fn from(writer: crate::W<GPIO_CFGCTL13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Pull Down Resistor for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_27_PD_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_27_PD_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_27_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_pd` reader - Pull Down Resistor for GPIO27."]
pub struct REG_GPIO_27_PD_R(crate::FieldReader<bool, REG_GPIO_27_PD_A>);
impl REG_GPIO_27_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_27_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_27_PD_A {
        match self.bits {
            false => REG_GPIO_27_PD_A::DISABLED,
            true => REG_GPIO_27_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_27_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_27_PD_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_27_PD_R {
    type Target = crate::FieldReader<bool, REG_GPIO_27_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_27_pd` writer - Pull Down Resistor for GPIO27."]
pub struct REG_GPIO_27_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_27_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_27_PD_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_27_PD_A::ENABLED)
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
#[doc = "Pull Up Resistor for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_27_PU_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_27_PU_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_27_PU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_pu` reader - Pull Up Resistor for GPIO27."]
pub struct REG_GPIO_27_PU_R(crate::FieldReader<bool, REG_GPIO_27_PU_A>);
impl REG_GPIO_27_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_27_PU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_27_PU_A {
        match self.bits {
            false => REG_GPIO_27_PU_A::DISABLED,
            true => REG_GPIO_27_PU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_27_PU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_27_PU_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_27_PU_R {
    type Target = crate::FieldReader<bool, REG_GPIO_27_PU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_27_pu` writer - Pull Up Resistor for GPIO27."]
pub struct REG_GPIO_27_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_PU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_27_PU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_27_PU_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_27_PU_A::ENABLED)
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
#[doc = "Driving control enabled for GPIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_27_DRV_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_27_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_27_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_27_drv` reader - Driving control enabled for GPIO27."]
pub struct REG_GPIO_27_DRV_R(crate::FieldReader<u8, REG_GPIO_27_DRV_A>);
impl REG_GPIO_27_DRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_27_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_27_DRV_A> {
        match self.bits {
            0 => Some(REG_GPIO_27_DRV_A::DISABLED),
            1 => Some(REG_GPIO_27_DRV_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_27_DRV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_27_DRV_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_27_DRV_R {
    type Target = crate::FieldReader<u8, REG_GPIO_27_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_27_drv` writer - Driving control enabled for GPIO27."]
pub struct REG_GPIO_27_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_27_DRV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_27_DRV_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_27_DRV_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Schmitt trigger enabled for GPIO27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_27_SMT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_27_SMT_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_27_SMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_smt` reader - Schmitt trigger enabled for GPIO27."]
pub struct REG_GPIO_27_SMT_R(crate::FieldReader<bool, REG_GPIO_27_SMT_A>);
impl REG_GPIO_27_SMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_27_SMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_27_SMT_A {
        match self.bits {
            false => REG_GPIO_27_SMT_A::DISABLED,
            true => REG_GPIO_27_SMT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_27_SMT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_27_SMT_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_27_SMT_R {
    type Target = crate::FieldReader<bool, REG_GPIO_27_SMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_27_smt` writer - Schmitt trigger enabled for GPIO27."]
pub struct REG_GPIO_27_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_SMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_27_SMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_27_SMT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_27_SMT_A::ENABLED)
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
#[doc = "Input enable for GPIO27.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_27_IE_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_27_IE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_27_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_27_ie` reader - Input enable for GPIO27."]
pub struct REG_GPIO_27_IE_R(crate::FieldReader<bool, REG_GPIO_27_IE_A>);
impl REG_GPIO_27_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_27_IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_27_IE_A {
        match self.bits {
            false => REG_GPIO_27_IE_A::DISABLED,
            true => REG_GPIO_27_IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_27_IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_27_IE_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_27_IE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_27_IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_27_ie` writer - Input enable for GPIO27."]
pub struct REG_GPIO_27_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_27_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_27_IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_27_IE_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_27_IE_A::ENABLED)
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
#[doc = "Pull Down Resistor for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_26_PD_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_26_PD_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_26_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_pd` reader - Pull Down Resistor for GPIO26."]
pub struct REG_GPIO_26_PD_R(crate::FieldReader<bool, REG_GPIO_26_PD_A>);
impl REG_GPIO_26_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_26_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_26_PD_A {
        match self.bits {
            false => REG_GPIO_26_PD_A::DISABLED,
            true => REG_GPIO_26_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_26_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_26_PD_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_26_PD_R {
    type Target = crate::FieldReader<bool, REG_GPIO_26_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_26_pd` writer - Pull Down Resistor for GPIO26."]
pub struct REG_GPIO_26_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_26_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_26_PD_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_26_PD_A::ENABLED)
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
#[doc = "Pull Up Resistor for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_26_PU_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_26_PU_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_26_PU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_pu` reader - Pull Up Resistor for GPIO26."]
pub struct REG_GPIO_26_PU_R(crate::FieldReader<bool, REG_GPIO_26_PU_A>);
impl REG_GPIO_26_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_26_PU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_26_PU_A {
        match self.bits {
            false => REG_GPIO_26_PU_A::DISABLED,
            true => REG_GPIO_26_PU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_26_PU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_26_PU_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_26_PU_R {
    type Target = crate::FieldReader<bool, REG_GPIO_26_PU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_26_pu` writer - Pull Up Resistor for GPIO26."]
pub struct REG_GPIO_26_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_PU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_26_PU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_26_PU_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_26_PU_A::ENABLED)
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
#[doc = "Driving control enabled for GPIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_26_DRV_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_26_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_26_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_26_drv` reader - Driving control enabled for GPIO26."]
pub struct REG_GPIO_26_DRV_R(crate::FieldReader<u8, REG_GPIO_26_DRV_A>);
impl REG_GPIO_26_DRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_26_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_26_DRV_A> {
        match self.bits {
            0 => Some(REG_GPIO_26_DRV_A::DISABLED),
            1 => Some(REG_GPIO_26_DRV_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_26_DRV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_26_DRV_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_26_DRV_R {
    type Target = crate::FieldReader<u8, REG_GPIO_26_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_26_drv` writer - Driving control enabled for GPIO26."]
pub struct REG_GPIO_26_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_26_DRV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_26_DRV_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_26_DRV_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Schmitt trigger enabled for GPIO26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_26_SMT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_26_SMT_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_26_SMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_smt` reader - Schmitt trigger enabled for GPIO26."]
pub struct REG_GPIO_26_SMT_R(crate::FieldReader<bool, REG_GPIO_26_SMT_A>);
impl REG_GPIO_26_SMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_26_SMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_26_SMT_A {
        match self.bits {
            false => REG_GPIO_26_SMT_A::DISABLED,
            true => REG_GPIO_26_SMT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_26_SMT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_26_SMT_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_26_SMT_R {
    type Target = crate::FieldReader<bool, REG_GPIO_26_SMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_26_smt` writer - Schmitt trigger enabled for GPIO26."]
pub struct REG_GPIO_26_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_SMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_26_SMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_26_SMT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_26_SMT_A::ENABLED)
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
#[doc = "Input enable for GPIO26.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_26_IE_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_26_IE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_26_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_26_ie` reader - Input enable for GPIO26."]
pub struct REG_GPIO_26_IE_R(crate::FieldReader<bool, REG_GPIO_26_IE_A>);
impl REG_GPIO_26_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_26_IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_26_IE_A {
        match self.bits {
            false => REG_GPIO_26_IE_A::DISABLED,
            true => REG_GPIO_26_IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_26_IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_26_IE_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_26_IE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_26_IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_26_ie` writer - Input enable for GPIO26."]
pub struct REG_GPIO_26_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_26_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_26_IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_26_IE_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_26_IE_A::ENABLED)
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
    #[doc = "Bit 21 - Pull Down Resistor for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_pd(&self) -> REG_GPIO_27_PD_R {
        REG_GPIO_27_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_pu(&self) -> REG_GPIO_27_PU_R {
        REG_GPIO_27_PU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_drv(&self) -> REG_GPIO_27_DRV_R {
        REG_GPIO_27_DRV_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_smt(&self) -> REG_GPIO_27_SMT_R {
        REG_GPIO_27_SMT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input enable for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_ie(&self) -> REG_GPIO_27_IE_R {
        REG_GPIO_27_IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_pd(&self) -> REG_GPIO_26_PD_R {
        REG_GPIO_26_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_pu(&self) -> REG_GPIO_26_PU_R {
        REG_GPIO_26_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_drv(&self) -> REG_GPIO_26_DRV_R {
        REG_GPIO_26_DRV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_smt(&self) -> REG_GPIO_26_SMT_R {
        REG_GPIO_26_SMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Input enable for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_ie(&self) -> REG_GPIO_26_IE_R {
        REG_GPIO_26_IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - Pull Down Resistor for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_pd(&mut self) -> REG_GPIO_27_PD_W {
        REG_GPIO_27_PD_W { w: self }
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_pu(&mut self) -> REG_GPIO_27_PU_W {
        REG_GPIO_27_PU_W { w: self }
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_drv(&mut self) -> REG_GPIO_27_DRV_W {
        REG_GPIO_27_DRV_W { w: self }
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_smt(&mut self) -> REG_GPIO_27_SMT_W {
        REG_GPIO_27_SMT_W { w: self }
    }
    #[doc = "Bit 16 - Input enable for GPIO27."]
    #[inline(always)]
    pub fn reg_gpio_27_ie(&mut self) -> REG_GPIO_27_IE_W {
        REG_GPIO_27_IE_W { w: self }
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_pd(&mut self) -> REG_GPIO_26_PD_W {
        REG_GPIO_26_PD_W { w: self }
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_pu(&mut self) -> REG_GPIO_26_PU_W {
        REG_GPIO_26_PU_W { w: self }
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_drv(&mut self) -> REG_GPIO_26_DRV_W {
        REG_GPIO_26_DRV_W { w: self }
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_smt(&mut self) -> REG_GPIO_26_SMT_W {
        REG_GPIO_26_SMT_W { w: self }
    }
    #[doc = "Bit 0 - Input enable for GPIO26."]
    #[inline(always)]
    pub fn reg_gpio_26_ie(&mut self) -> REG_GPIO_26_IE_W {
        REG_GPIO_26_IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO26, GPIO27 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl13](index.html) module"]
pub struct GPIO_CFGCTL13_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl13::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl13::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_CFGCTL13 to value 0x0003_0003"]
impl crate::Resettable for GPIO_CFGCTL13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0003_0003
    }
}
