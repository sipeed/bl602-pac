#[doc = "Register `GPIO_CFGCTL5` reader"]
pub struct R(crate::R<GPIO_CFGCTL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL5` writer"]
pub struct W(crate::W<GPIO_CFGCTL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL5_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Function select for GPIO11.\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_11_FUNC_SEL_A {
    #[doc = "4: `100`"]
    SPI_SCLK = 4,
    #[doc = "6: `110`"]
    I2C_SDA = 6,
    #[doc = "7: `111`"]
    UART_SIG3 = 7,
    #[doc = "8: `1000`"]
    PWM_CH1 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_3 = 9,
    #[doc = "10: `1010`"]
    IRLED_OUT_GPIP_CH10 = 10,
    #[doc = "11: `1011`"]
    SWGPIO_11 = 11,
    #[doc = "14: `1110`"]
    E21_TDO = 14,
}
impl From<REG_GPIO_11_FUNC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_11_FUNC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_11_func_sel` reader - Function select for GPIO11."]
pub struct REG_GPIO_11_FUNC_SEL_R(crate::FieldReader<u8, REG_GPIO_11_FUNC_SEL_A>);
impl REG_GPIO_11_FUNC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_11_FUNC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_11_FUNC_SEL_A> {
        match self.bits {
            4 => Some(REG_GPIO_11_FUNC_SEL_A::SPI_SCLK),
            6 => Some(REG_GPIO_11_FUNC_SEL_A::I2C_SDA),
            7 => Some(REG_GPIO_11_FUNC_SEL_A::UART_SIG3),
            8 => Some(REG_GPIO_11_FUNC_SEL_A::PWM_CH1),
            9 => Some(REG_GPIO_11_FUNC_SEL_A::FEM_GPIO_3),
            10 => Some(REG_GPIO_11_FUNC_SEL_A::IRLED_OUT_GPIP_CH10),
            11 => Some(REG_GPIO_11_FUNC_SEL_A::SWGPIO_11),
            14 => Some(REG_GPIO_11_FUNC_SEL_A::E21_TDO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_SCLK`"]
    #[inline(always)]
    pub fn is_spi_sclk(&self) -> bool {
        **self == REG_GPIO_11_FUNC_SEL_A::SPI_SCLK
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        **self == REG_GPIO_11_FUNC_SEL_A::I2C_SDA
    }
    #[doc = "Checks if the value of the field is `UART_SIG3`"]
    #[inline(always)]
    pub fn is_uart_sig3(&self) -> bool {
        **self == REG_GPIO_11_FUNC_SEL_A::UART_SIG3
    }
    #[doc = "Checks if the value of the field is `PWM_CH1`"]
    #[inline(always)]
    pub fn is_pwm_ch1(&self) -> bool {
        **self == REG_GPIO_11_FUNC_SEL_A::PWM_CH1
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_3`"]
    #[inline(always)]
    pub fn is_fem_gpio_3(&self) -> bool {
        **self == REG_GPIO_11_FUNC_SEL_A::FEM_GPIO_3
    }
    #[doc = "Checks if the value of the field is `IRLED_OUT_GPIP_CH10`"]
    #[inline(always)]
    pub fn is_irled_out_gpip_ch10(&self) -> bool {
        **self == REG_GPIO_11_FUNC_SEL_A::IRLED_OUT_GPIP_CH10
    }
    #[doc = "Checks if the value of the field is `SWGPIO_11`"]
    #[inline(always)]
    pub fn is_swgpio_11(&self) -> bool {
        **self == REG_GPIO_11_FUNC_SEL_A::SWGPIO_11
    }
    #[doc = "Checks if the value of the field is `E21_TDO`"]
    #[inline(always)]
    pub fn is_e21_tdo(&self) -> bool {
        **self == REG_GPIO_11_FUNC_SEL_A::E21_TDO
    }
}
impl core::ops::Deref for REG_GPIO_11_FUNC_SEL_R {
    type Target = crate::FieldReader<u8, REG_GPIO_11_FUNC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_11_func_sel` writer - Function select for GPIO11."]
pub struct REG_GPIO_11_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_FUNC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_11_FUNC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_sclk(self) -> &'a mut W {
        self.variant(REG_GPIO_11_FUNC_SEL_A::SPI_SCLK)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(REG_GPIO_11_FUNC_SEL_A::I2C_SDA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig3(self) -> &'a mut W {
        self.variant(REG_GPIO_11_FUNC_SEL_A::UART_SIG3)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch1(self) -> &'a mut W {
        self.variant(REG_GPIO_11_FUNC_SEL_A::PWM_CH1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_3(self) -> &'a mut W {
        self.variant(REG_GPIO_11_FUNC_SEL_A::FEM_GPIO_3)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn irled_out_gpip_ch10(self) -> &'a mut W {
        self.variant(REG_GPIO_11_FUNC_SEL_A::IRLED_OUT_GPIP_CH10)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_11(self) -> &'a mut W {
        self.variant(REG_GPIO_11_FUNC_SEL_A::SWGPIO_11)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdo(self) -> &'a mut W {
        self.variant(REG_GPIO_11_FUNC_SEL_A::E21_TDO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Pull Down Resistor for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_11_PD_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_11_PD_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_11_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_pd` reader - Pull Down Resistor for GPIO11."]
pub struct REG_GPIO_11_PD_R(crate::FieldReader<bool, REG_GPIO_11_PD_A>);
impl REG_GPIO_11_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_11_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_11_PD_A {
        match self.bits {
            false => REG_GPIO_11_PD_A::DISABLED,
            true => REG_GPIO_11_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_11_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_11_PD_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_11_PD_R {
    type Target = crate::FieldReader<bool, REG_GPIO_11_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_11_pd` writer - Pull Down Resistor for GPIO11."]
pub struct REG_GPIO_11_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_11_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_PD_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_PD_A::ENABLED)
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
#[doc = "Pull Up Resistor for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_11_PU_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_11_PU_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_11_PU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_pu` reader - Pull Up Resistor for GPIO11."]
pub struct REG_GPIO_11_PU_R(crate::FieldReader<bool, REG_GPIO_11_PU_A>);
impl REG_GPIO_11_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_11_PU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_11_PU_A {
        match self.bits {
            false => REG_GPIO_11_PU_A::DISABLED,
            true => REG_GPIO_11_PU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_11_PU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_11_PU_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_11_PU_R {
    type Target = crate::FieldReader<bool, REG_GPIO_11_PU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_11_pu` writer - Pull Up Resistor for GPIO11."]
pub struct REG_GPIO_11_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_PU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_11_PU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_PU_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_PU_A::ENABLED)
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
#[doc = "Driving control enabled for GPIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_11_DRV_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_11_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_11_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_11_drv` reader - Driving control enabled for GPIO11."]
pub struct REG_GPIO_11_DRV_R(crate::FieldReader<u8, REG_GPIO_11_DRV_A>);
impl REG_GPIO_11_DRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_11_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_11_DRV_A> {
        match self.bits {
            0 => Some(REG_GPIO_11_DRV_A::DISABLED),
            1 => Some(REG_GPIO_11_DRV_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_11_DRV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_11_DRV_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_11_DRV_R {
    type Target = crate::FieldReader<u8, REG_GPIO_11_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_11_drv` writer - Driving control enabled for GPIO11."]
pub struct REG_GPIO_11_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_11_DRV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_DRV_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_DRV_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Schmitt trigger enabled for GPIO11.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_11_SMT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_11_SMT_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_11_SMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_smt` reader - Schmitt trigger enabled for GPIO11."]
pub struct REG_GPIO_11_SMT_R(crate::FieldReader<bool, REG_GPIO_11_SMT_A>);
impl REG_GPIO_11_SMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_11_SMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_11_SMT_A {
        match self.bits {
            false => REG_GPIO_11_SMT_A::DISABLED,
            true => REG_GPIO_11_SMT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_11_SMT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_11_SMT_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_11_SMT_R {
    type Target = crate::FieldReader<bool, REG_GPIO_11_SMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_11_smt` writer - Schmitt trigger enabled for GPIO11."]
pub struct REG_GPIO_11_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_SMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_11_SMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_SMT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_SMT_A::ENABLED)
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
#[doc = "Input enable for GPIO11.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_11_IE_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_11_IE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_11_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_11_ie` reader - Input enable for GPIO11."]
pub struct REG_GPIO_11_IE_R(crate::FieldReader<bool, REG_GPIO_11_IE_A>);
impl REG_GPIO_11_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_11_IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_11_IE_A {
        match self.bits {
            false => REG_GPIO_11_IE_A::DISABLED,
            true => REG_GPIO_11_IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_11_IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_11_IE_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_11_IE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_11_IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_11_ie` writer - Input enable for GPIO11."]
pub struct REG_GPIO_11_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_11_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_11_IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_IE_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_11_IE_A::ENABLED)
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
#[doc = "Function select for GPIO10.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_10_FUNC_SEL_A {
    #[doc = "4: `100`"]
    SPI_SS = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG2 = 7,
    #[doc = "8: `1000`"]
    PWM_CH0 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_2 = 9,
    #[doc = "10: `1010`"]
    MICBIAS_GPIP_CH8_GPIP_CH9 = 10,
    #[doc = "11: `1011`"]
    SWGPIO_10 = 11,
    #[doc = "14: `1110`"]
    E21_TCK = 14,
}
impl From<REG_GPIO_10_FUNC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_10_FUNC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_10_func_sel` reader - Function select for GPIO10."]
pub struct REG_GPIO_10_FUNC_SEL_R(crate::FieldReader<u8, REG_GPIO_10_FUNC_SEL_A>);
impl REG_GPIO_10_FUNC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_10_FUNC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_10_FUNC_SEL_A> {
        match self.bits {
            4 => Some(REG_GPIO_10_FUNC_SEL_A::SPI_SS),
            6 => Some(REG_GPIO_10_FUNC_SEL_A::I2C_SCL),
            7 => Some(REG_GPIO_10_FUNC_SEL_A::UART_SIG2),
            8 => Some(REG_GPIO_10_FUNC_SEL_A::PWM_CH0),
            9 => Some(REG_GPIO_10_FUNC_SEL_A::FEM_GPIO_2),
            10 => Some(REG_GPIO_10_FUNC_SEL_A::MICBIAS_GPIP_CH8_GPIP_CH9),
            11 => Some(REG_GPIO_10_FUNC_SEL_A::SWGPIO_10),
            14 => Some(REG_GPIO_10_FUNC_SEL_A::E21_TCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_SS`"]
    #[inline(always)]
    pub fn is_spi_ss(&self) -> bool {
        **self == REG_GPIO_10_FUNC_SEL_A::SPI_SS
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        **self == REG_GPIO_10_FUNC_SEL_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG2`"]
    #[inline(always)]
    pub fn is_uart_sig2(&self) -> bool {
        **self == REG_GPIO_10_FUNC_SEL_A::UART_SIG2
    }
    #[doc = "Checks if the value of the field is `PWM_CH0`"]
    #[inline(always)]
    pub fn is_pwm_ch0(&self) -> bool {
        **self == REG_GPIO_10_FUNC_SEL_A::PWM_CH0
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_2`"]
    #[inline(always)]
    pub fn is_fem_gpio_2(&self) -> bool {
        **self == REG_GPIO_10_FUNC_SEL_A::FEM_GPIO_2
    }
    #[doc = "Checks if the value of the field is `MICBIAS_GPIP_CH8_GPIP_CH9`"]
    #[inline(always)]
    pub fn is_micbias_gpip_ch8_gpip_ch9(&self) -> bool {
        **self == REG_GPIO_10_FUNC_SEL_A::MICBIAS_GPIP_CH8_GPIP_CH9
    }
    #[doc = "Checks if the value of the field is `SWGPIO_10`"]
    #[inline(always)]
    pub fn is_swgpio_10(&self) -> bool {
        **self == REG_GPIO_10_FUNC_SEL_A::SWGPIO_10
    }
    #[doc = "Checks if the value of the field is `E21_TCK`"]
    #[inline(always)]
    pub fn is_e21_tck(&self) -> bool {
        **self == REG_GPIO_10_FUNC_SEL_A::E21_TCK
    }
}
impl core::ops::Deref for REG_GPIO_10_FUNC_SEL_R {
    type Target = crate::FieldReader<u8, REG_GPIO_10_FUNC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_10_func_sel` writer - Function select for GPIO10."]
pub struct REG_GPIO_10_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_FUNC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_10_FUNC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_ss(self) -> &'a mut W {
        self.variant(REG_GPIO_10_FUNC_SEL_A::SPI_SS)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(REG_GPIO_10_FUNC_SEL_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig2(self) -> &'a mut W {
        self.variant(REG_GPIO_10_FUNC_SEL_A::UART_SIG2)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch0(self) -> &'a mut W {
        self.variant(REG_GPIO_10_FUNC_SEL_A::PWM_CH0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_2(self) -> &'a mut W {
        self.variant(REG_GPIO_10_FUNC_SEL_A::FEM_GPIO_2)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn micbias_gpip_ch8_gpip_ch9(self) -> &'a mut W {
        self.variant(REG_GPIO_10_FUNC_SEL_A::MICBIAS_GPIP_CH8_GPIP_CH9)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_10(self) -> &'a mut W {
        self.variant(REG_GPIO_10_FUNC_SEL_A::SWGPIO_10)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tck(self) -> &'a mut W {
        self.variant(REG_GPIO_10_FUNC_SEL_A::E21_TCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Pull Down Resistor for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_10_PD_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_10_PD_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_10_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_pd` reader - Pull Down Resistor for GPIO10."]
pub struct REG_GPIO_10_PD_R(crate::FieldReader<bool, REG_GPIO_10_PD_A>);
impl REG_GPIO_10_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_10_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_10_PD_A {
        match self.bits {
            false => REG_GPIO_10_PD_A::DISABLED,
            true => REG_GPIO_10_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_10_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_10_PD_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_10_PD_R {
    type Target = crate::FieldReader<bool, REG_GPIO_10_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_10_pd` writer - Pull Down Resistor for GPIO10."]
pub struct REG_GPIO_10_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_10_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_PD_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_PD_A::ENABLED)
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
#[doc = "Pull Up Resistor for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_10_PU_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_10_PU_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_10_PU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_pu` reader - Pull Up Resistor for GPIO10."]
pub struct REG_GPIO_10_PU_R(crate::FieldReader<bool, REG_GPIO_10_PU_A>);
impl REG_GPIO_10_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_10_PU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_10_PU_A {
        match self.bits {
            false => REG_GPIO_10_PU_A::DISABLED,
            true => REG_GPIO_10_PU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_10_PU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_10_PU_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_10_PU_R {
    type Target = crate::FieldReader<bool, REG_GPIO_10_PU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_10_pu` writer - Pull Up Resistor for GPIO10."]
pub struct REG_GPIO_10_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_PU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_10_PU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_PU_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_PU_A::ENABLED)
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
#[doc = "Driving control enabled for GPIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_10_DRV_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_10_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_10_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_10_drv` reader - Driving control enabled for GPIO10."]
pub struct REG_GPIO_10_DRV_R(crate::FieldReader<u8, REG_GPIO_10_DRV_A>);
impl REG_GPIO_10_DRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_10_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_10_DRV_A> {
        match self.bits {
            0 => Some(REG_GPIO_10_DRV_A::DISABLED),
            1 => Some(REG_GPIO_10_DRV_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_10_DRV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_10_DRV_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_10_DRV_R {
    type Target = crate::FieldReader<u8, REG_GPIO_10_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_10_drv` writer - Driving control enabled for GPIO10."]
pub struct REG_GPIO_10_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_10_DRV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_DRV_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_DRV_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Schmitt trigger enabled for GPIO10.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_10_SMT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_10_SMT_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_10_SMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_smt` reader - Schmitt trigger enabled for GPIO10."]
pub struct REG_GPIO_10_SMT_R(crate::FieldReader<bool, REG_GPIO_10_SMT_A>);
impl REG_GPIO_10_SMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_10_SMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_10_SMT_A {
        match self.bits {
            false => REG_GPIO_10_SMT_A::DISABLED,
            true => REG_GPIO_10_SMT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_10_SMT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_10_SMT_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_10_SMT_R {
    type Target = crate::FieldReader<bool, REG_GPIO_10_SMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_10_smt` writer - Schmitt trigger enabled for GPIO10."]
pub struct REG_GPIO_10_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_SMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_10_SMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_SMT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_SMT_A::ENABLED)
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
#[doc = "Input enable for GPIO10.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_10_IE_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_10_IE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_10_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_10_ie` reader - Input enable for GPIO10."]
pub struct REG_GPIO_10_IE_R(crate::FieldReader<bool, REG_GPIO_10_IE_A>);
impl REG_GPIO_10_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_10_IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_10_IE_A {
        match self.bits {
            false => REG_GPIO_10_IE_A::DISABLED,
            true => REG_GPIO_10_IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_10_IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_10_IE_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_10_IE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_10_IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_10_ie` writer - Input enable for GPIO10."]
pub struct REG_GPIO_10_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_10_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_10_IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_IE_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_10_IE_A::ENABLED)
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
    #[doc = "Bits 24:27 - Function select for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_func_sel(&self) -> REG_GPIO_11_FUNC_SEL_R {
        REG_GPIO_11_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_pd(&self) -> REG_GPIO_11_PD_R {
        REG_GPIO_11_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_pu(&self) -> REG_GPIO_11_PU_R {
        REG_GPIO_11_PU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_drv(&self) -> REG_GPIO_11_DRV_R {
        REG_GPIO_11_DRV_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_smt(&self) -> REG_GPIO_11_SMT_R {
        REG_GPIO_11_SMT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input enable for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_ie(&self) -> REG_GPIO_11_IE_R {
        REG_GPIO_11_IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_func_sel(&self) -> REG_GPIO_10_FUNC_SEL_R {
        REG_GPIO_10_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_pd(&self) -> REG_GPIO_10_PD_R {
        REG_GPIO_10_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_pu(&self) -> REG_GPIO_10_PU_R {
        REG_GPIO_10_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_drv(&self) -> REG_GPIO_10_DRV_R {
        REG_GPIO_10_DRV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_smt(&self) -> REG_GPIO_10_SMT_R {
        REG_GPIO_10_SMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Input enable for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_ie(&self) -> REG_GPIO_10_IE_R {
        REG_GPIO_10_IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27 - Function select for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_func_sel(&mut self) -> REG_GPIO_11_FUNC_SEL_W {
        REG_GPIO_11_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_pd(&mut self) -> REG_GPIO_11_PD_W {
        REG_GPIO_11_PD_W { w: self }
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_pu(&mut self) -> REG_GPIO_11_PU_W {
        REG_GPIO_11_PU_W { w: self }
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_drv(&mut self) -> REG_GPIO_11_DRV_W {
        REG_GPIO_11_DRV_W { w: self }
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_smt(&mut self) -> REG_GPIO_11_SMT_W {
        REG_GPIO_11_SMT_W { w: self }
    }
    #[doc = "Bit 16 - Input enable for GPIO11."]
    #[inline(always)]
    pub fn reg_gpio_11_ie(&mut self) -> REG_GPIO_11_IE_W {
        REG_GPIO_11_IE_W { w: self }
    }
    #[doc = "Bits 8:11 - Function select for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_func_sel(&mut self) -> REG_GPIO_10_FUNC_SEL_W {
        REG_GPIO_10_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_pd(&mut self) -> REG_GPIO_10_PD_W {
        REG_GPIO_10_PD_W { w: self }
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_pu(&mut self) -> REG_GPIO_10_PU_W {
        REG_GPIO_10_PU_W { w: self }
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_drv(&mut self) -> REG_GPIO_10_DRV_W {
        REG_GPIO_10_DRV_W { w: self }
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_smt(&mut self) -> REG_GPIO_10_SMT_W {
        REG_GPIO_10_SMT_W { w: self }
    }
    #[doc = "Bit 0 - Input enable for GPIO10."]
    #[inline(always)]
    pub fn reg_gpio_10_ie(&mut self) -> REG_GPIO_10_IE_W {
        REG_GPIO_10_IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO10, GPIO11 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl5](index.html) module"]
pub struct GPIO_CFGCTL5_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl5::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl5::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_CFGCTL5 to value 0x0e03_0b03"]
impl crate::Resettable for GPIO_CFGCTL5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e03_0b03
    }
}
