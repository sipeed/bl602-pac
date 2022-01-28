#[doc = "Register `GPIO_CFGCTL8` reader"]
pub struct R(crate::R<GPIO_CFGCTL8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO_CFGCTL8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO_CFGCTL8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL8` writer"]
pub struct W(crate::W<GPIO_CFGCTL8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL8_SPEC>;
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
impl From<crate::W<GPIO_CFGCTL8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO_CFGCTL8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Function select for GPIO17.\n\nValue on reset: 14"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_17_FUNC_SEL_A {
    #[doc = "2: `10`"]
    SF_D3 = 2,
    #[doc = "4: `100`"]
    SPI_MOSI_SPI_MISO = 4,
    #[doc = "6: `110`"]
    I2C_SDA = 6,
    #[doc = "7: `111`"]
    UART_SIG1 = 7,
    #[doc = "8: `1000`"]
    PWM_CH2 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_1 = 9,
    #[doc = "10: `1010`"]
    PMIP_DC_TP_OUT = 10,
    #[doc = "11: `1011`"]
    SWGPIO_17 = 11,
    #[doc = "14: `1110`"]
    E21_TDI = 14,
}
impl From<REG_GPIO_17_FUNC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_17_FUNC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_17_func_sel` reader - Function select for GPIO17."]
pub struct REG_GPIO_17_FUNC_SEL_R(crate::FieldReader<u8, REG_GPIO_17_FUNC_SEL_A>);
impl REG_GPIO_17_FUNC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_17_FUNC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_17_FUNC_SEL_A> {
        match self.bits {
            2 => Some(REG_GPIO_17_FUNC_SEL_A::SF_D3),
            4 => Some(REG_GPIO_17_FUNC_SEL_A::SPI_MOSI_SPI_MISO),
            6 => Some(REG_GPIO_17_FUNC_SEL_A::I2C_SDA),
            7 => Some(REG_GPIO_17_FUNC_SEL_A::UART_SIG1),
            8 => Some(REG_GPIO_17_FUNC_SEL_A::PWM_CH2),
            9 => Some(REG_GPIO_17_FUNC_SEL_A::FEM_GPIO_1),
            10 => Some(REG_GPIO_17_FUNC_SEL_A::PMIP_DC_TP_OUT),
            11 => Some(REG_GPIO_17_FUNC_SEL_A::SWGPIO_17),
            14 => Some(REG_GPIO_17_FUNC_SEL_A::E21_TDI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SF_D3`"]
    #[inline(always)]
    pub fn is_sf_d3(&self) -> bool {
        **self == REG_GPIO_17_FUNC_SEL_A::SF_D3
    }
    #[doc = "Checks if the value of the field is `SPI_MOSI_SPI_MISO`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        **self == REG_GPIO_17_FUNC_SEL_A::SPI_MOSI_SPI_MISO
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        **self == REG_GPIO_17_FUNC_SEL_A::I2C_SDA
    }
    #[doc = "Checks if the value of the field is `UART_SIG1`"]
    #[inline(always)]
    pub fn is_uart_sig1(&self) -> bool {
        **self == REG_GPIO_17_FUNC_SEL_A::UART_SIG1
    }
    #[doc = "Checks if the value of the field is `PWM_CH2`"]
    #[inline(always)]
    pub fn is_pwm_ch2(&self) -> bool {
        **self == REG_GPIO_17_FUNC_SEL_A::PWM_CH2
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_1`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        **self == REG_GPIO_17_FUNC_SEL_A::FEM_GPIO_1
    }
    #[doc = "Checks if the value of the field is `PMIP_DC_TP_OUT`"]
    #[inline(always)]
    pub fn is_pmip_dc_tp_out(&self) -> bool {
        **self == REG_GPIO_17_FUNC_SEL_A::PMIP_DC_TP_OUT
    }
    #[doc = "Checks if the value of the field is `SWGPIO_17`"]
    #[inline(always)]
    pub fn is_swgpio_17(&self) -> bool {
        **self == REG_GPIO_17_FUNC_SEL_A::SWGPIO_17
    }
    #[doc = "Checks if the value of the field is `E21_TDI`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        **self == REG_GPIO_17_FUNC_SEL_A::E21_TDI
    }
}
impl core::ops::Deref for REG_GPIO_17_FUNC_SEL_R {
    type Target = crate::FieldReader<u8, REG_GPIO_17_FUNC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_17_func_sel` writer - Function select for GPIO17."]
pub struct REG_GPIO_17_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_FUNC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_17_FUNC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d3(self) -> &'a mut W {
        self.variant(REG_GPIO_17_FUNC_SEL_A::SF_D3)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut W {
        self.variant(REG_GPIO_17_FUNC_SEL_A::SPI_MOSI_SPI_MISO)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(REG_GPIO_17_FUNC_SEL_A::I2C_SDA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig1(self) -> &'a mut W {
        self.variant(REG_GPIO_17_FUNC_SEL_A::UART_SIG1)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch2(self) -> &'a mut W {
        self.variant(REG_GPIO_17_FUNC_SEL_A::PWM_CH2)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut W {
        self.variant(REG_GPIO_17_FUNC_SEL_A::FEM_GPIO_1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn pmip_dc_tp_out(self) -> &'a mut W {
        self.variant(REG_GPIO_17_FUNC_SEL_A::PMIP_DC_TP_OUT)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_17(self) -> &'a mut W {
        self.variant(REG_GPIO_17_FUNC_SEL_A::SWGPIO_17)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut W {
        self.variant(REG_GPIO_17_FUNC_SEL_A::E21_TDI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Pull Down Resistor for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_17_PD_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_17_PD_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_17_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_pd` reader - Pull Down Resistor for GPIO17."]
pub struct REG_GPIO_17_PD_R(crate::FieldReader<bool, REG_GPIO_17_PD_A>);
impl REG_GPIO_17_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_17_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_17_PD_A {
        match self.bits {
            false => REG_GPIO_17_PD_A::DISABLED,
            true => REG_GPIO_17_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_17_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_17_PD_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_17_PD_R {
    type Target = crate::FieldReader<bool, REG_GPIO_17_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_17_pd` writer - Pull Down Resistor for GPIO17."]
pub struct REG_GPIO_17_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_17_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_PD_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_PD_A::ENABLED)
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
#[doc = "Pull Up Resistor for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_17_PU_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_17_PU_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_17_PU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_pu` reader - Pull Up Resistor for GPIO17."]
pub struct REG_GPIO_17_PU_R(crate::FieldReader<bool, REG_GPIO_17_PU_A>);
impl REG_GPIO_17_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_17_PU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_17_PU_A {
        match self.bits {
            false => REG_GPIO_17_PU_A::DISABLED,
            true => REG_GPIO_17_PU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_17_PU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_17_PU_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_17_PU_R {
    type Target = crate::FieldReader<bool, REG_GPIO_17_PU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_17_pu` writer - Pull Up Resistor for GPIO17."]
pub struct REG_GPIO_17_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_PU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_17_PU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_PU_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_PU_A::ENABLED)
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
#[doc = "Driving control enabled for GPIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_17_DRV_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_17_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_17_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_17_drv` reader - Driving control enabled for GPIO17."]
pub struct REG_GPIO_17_DRV_R(crate::FieldReader<u8, REG_GPIO_17_DRV_A>);
impl REG_GPIO_17_DRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_17_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_17_DRV_A> {
        match self.bits {
            0 => Some(REG_GPIO_17_DRV_A::DISABLED),
            1 => Some(REG_GPIO_17_DRV_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_17_DRV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_17_DRV_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_17_DRV_R {
    type Target = crate::FieldReader<u8, REG_GPIO_17_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_17_drv` writer - Driving control enabled for GPIO17."]
pub struct REG_GPIO_17_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_17_DRV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_DRV_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_DRV_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Schmitt trigger enabled for GPIO17.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_17_SMT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_17_SMT_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_17_SMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_smt` reader - Schmitt trigger enabled for GPIO17."]
pub struct REG_GPIO_17_SMT_R(crate::FieldReader<bool, REG_GPIO_17_SMT_A>);
impl REG_GPIO_17_SMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_17_SMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_17_SMT_A {
        match self.bits {
            false => REG_GPIO_17_SMT_A::DISABLED,
            true => REG_GPIO_17_SMT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_17_SMT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_17_SMT_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_17_SMT_R {
    type Target = crate::FieldReader<bool, REG_GPIO_17_SMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_17_smt` writer - Schmitt trigger enabled for GPIO17."]
pub struct REG_GPIO_17_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_SMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_17_SMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_SMT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_SMT_A::ENABLED)
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
#[doc = "Input enable for GPIO17.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_17_IE_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_17_IE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_17_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_17_ie` reader - Input enable for GPIO17."]
pub struct REG_GPIO_17_IE_R(crate::FieldReader<bool, REG_GPIO_17_IE_A>);
impl REG_GPIO_17_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_17_IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_17_IE_A {
        match self.bits {
            false => REG_GPIO_17_IE_A::DISABLED,
            true => REG_GPIO_17_IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_17_IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_17_IE_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_17_IE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_17_IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_17_ie` writer - Input enable for GPIO17."]
pub struct REG_GPIO_17_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_17_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_17_IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_IE_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_17_IE_A::ENABLED)
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
#[doc = "Function select for GPIO16.\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_16_FUNC_SEL_A {
    #[doc = "4: `100`"]
    SPI_MISO_SPI_MOSI = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG0 = 7,
    #[doc = "8: `1000`"]
    PWM_CH1 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_0 = 9,
    #[doc = "11: `1011`"]
    SWGPIO_16 = 11,
    #[doc = "14: `1110`"]
    E21_TMS = 14,
}
impl From<REG_GPIO_16_FUNC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_16_FUNC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_16_func_sel` reader - Function select for GPIO16."]
pub struct REG_GPIO_16_FUNC_SEL_R(crate::FieldReader<u8, REG_GPIO_16_FUNC_SEL_A>);
impl REG_GPIO_16_FUNC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_16_FUNC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_16_FUNC_SEL_A> {
        match self.bits {
            4 => Some(REG_GPIO_16_FUNC_SEL_A::SPI_MISO_SPI_MOSI),
            6 => Some(REG_GPIO_16_FUNC_SEL_A::I2C_SCL),
            7 => Some(REG_GPIO_16_FUNC_SEL_A::UART_SIG0),
            8 => Some(REG_GPIO_16_FUNC_SEL_A::PWM_CH1),
            9 => Some(REG_GPIO_16_FUNC_SEL_A::FEM_GPIO_0),
            11 => Some(REG_GPIO_16_FUNC_SEL_A::SWGPIO_16),
            14 => Some(REG_GPIO_16_FUNC_SEL_A::E21_TMS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MISO_SPI_MOSI`"]
    #[inline(always)]
    pub fn is_spi_miso_spi_mosi(&self) -> bool {
        **self == REG_GPIO_16_FUNC_SEL_A::SPI_MISO_SPI_MOSI
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        **self == REG_GPIO_16_FUNC_SEL_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG0`"]
    #[inline(always)]
    pub fn is_uart_sig0(&self) -> bool {
        **self == REG_GPIO_16_FUNC_SEL_A::UART_SIG0
    }
    #[doc = "Checks if the value of the field is `PWM_CH1`"]
    #[inline(always)]
    pub fn is_pwm_ch1(&self) -> bool {
        **self == REG_GPIO_16_FUNC_SEL_A::PWM_CH1
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_0`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        **self == REG_GPIO_16_FUNC_SEL_A::FEM_GPIO_0
    }
    #[doc = "Checks if the value of the field is `SWGPIO_16`"]
    #[inline(always)]
    pub fn is_swgpio_16(&self) -> bool {
        **self == REG_GPIO_16_FUNC_SEL_A::SWGPIO_16
    }
    #[doc = "Checks if the value of the field is `E21_TMS`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        **self == REG_GPIO_16_FUNC_SEL_A::E21_TMS
    }
}
impl core::ops::Deref for REG_GPIO_16_FUNC_SEL_R {
    type Target = crate::FieldReader<u8, REG_GPIO_16_FUNC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_16_func_sel` writer - Function select for GPIO16."]
pub struct REG_GPIO_16_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_FUNC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_16_FUNC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_miso_spi_mosi(self) -> &'a mut W {
        self.variant(REG_GPIO_16_FUNC_SEL_A::SPI_MISO_SPI_MOSI)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(REG_GPIO_16_FUNC_SEL_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig0(self) -> &'a mut W {
        self.variant(REG_GPIO_16_FUNC_SEL_A::UART_SIG0)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch1(self) -> &'a mut W {
        self.variant(REG_GPIO_16_FUNC_SEL_A::PWM_CH1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut W {
        self.variant(REG_GPIO_16_FUNC_SEL_A::FEM_GPIO_0)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_16(self) -> &'a mut W {
        self.variant(REG_GPIO_16_FUNC_SEL_A::SWGPIO_16)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut W {
        self.variant(REG_GPIO_16_FUNC_SEL_A::E21_TMS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Pull Down Resistor for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_16_PD_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_16_PD_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_16_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_pd` reader - Pull Down Resistor for GPIO16."]
pub struct REG_GPIO_16_PD_R(crate::FieldReader<bool, REG_GPIO_16_PD_A>);
impl REG_GPIO_16_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_16_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_16_PD_A {
        match self.bits {
            false => REG_GPIO_16_PD_A::DISABLED,
            true => REG_GPIO_16_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_16_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_16_PD_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_16_PD_R {
    type Target = crate::FieldReader<bool, REG_GPIO_16_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_16_pd` writer - Pull Down Resistor for GPIO16."]
pub struct REG_GPIO_16_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_16_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_PD_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_PD_A::ENABLED)
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
#[doc = "Pull Up Resistor for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_16_PU_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_16_PU_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_16_PU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_pu` reader - Pull Up Resistor for GPIO16."]
pub struct REG_GPIO_16_PU_R(crate::FieldReader<bool, REG_GPIO_16_PU_A>);
impl REG_GPIO_16_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_16_PU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_16_PU_A {
        match self.bits {
            false => REG_GPIO_16_PU_A::DISABLED,
            true => REG_GPIO_16_PU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_16_PU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_16_PU_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_16_PU_R {
    type Target = crate::FieldReader<bool, REG_GPIO_16_PU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_16_pu` writer - Pull Up Resistor for GPIO16."]
pub struct REG_GPIO_16_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_PU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_16_PU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_PU_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_PU_A::ENABLED)
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
#[doc = "Driving control enabled for GPIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_16_DRV_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_16_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_16_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_16_drv` reader - Driving control enabled for GPIO16."]
pub struct REG_GPIO_16_DRV_R(crate::FieldReader<u8, REG_GPIO_16_DRV_A>);
impl REG_GPIO_16_DRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_16_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_16_DRV_A> {
        match self.bits {
            0 => Some(REG_GPIO_16_DRV_A::DISABLED),
            1 => Some(REG_GPIO_16_DRV_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_16_DRV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_16_DRV_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_16_DRV_R {
    type Target = crate::FieldReader<u8, REG_GPIO_16_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_16_drv` writer - Driving control enabled for GPIO16."]
pub struct REG_GPIO_16_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_16_DRV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_DRV_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_DRV_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Schmitt trigger enabled for GPIO16.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_16_SMT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_16_SMT_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_16_SMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_smt` reader - Schmitt trigger enabled for GPIO16."]
pub struct REG_GPIO_16_SMT_R(crate::FieldReader<bool, REG_GPIO_16_SMT_A>);
impl REG_GPIO_16_SMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_16_SMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_16_SMT_A {
        match self.bits {
            false => REG_GPIO_16_SMT_A::DISABLED,
            true => REG_GPIO_16_SMT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_16_SMT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_16_SMT_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_16_SMT_R {
    type Target = crate::FieldReader<bool, REG_GPIO_16_SMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_16_smt` writer - Schmitt trigger enabled for GPIO16."]
pub struct REG_GPIO_16_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_SMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_16_SMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_SMT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_SMT_A::ENABLED)
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
#[doc = "Input enable for GPIO16.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_16_IE_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_16_IE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_16_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_16_ie` reader - Input enable for GPIO16."]
pub struct REG_GPIO_16_IE_R(crate::FieldReader<bool, REG_GPIO_16_IE_A>);
impl REG_GPIO_16_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_16_IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_16_IE_A {
        match self.bits {
            false => REG_GPIO_16_IE_A::DISABLED,
            true => REG_GPIO_16_IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_16_IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_16_IE_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_16_IE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_16_IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_16_ie` writer - Input enable for GPIO16."]
pub struct REG_GPIO_16_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_16_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_16_IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_IE_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_16_IE_A::ENABLED)
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
    #[doc = "Bits 24:27 - Function select for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_func_sel(&self) -> REG_GPIO_17_FUNC_SEL_R {
        REG_GPIO_17_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_pd(&self) -> REG_GPIO_17_PD_R {
        REG_GPIO_17_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_pu(&self) -> REG_GPIO_17_PU_R {
        REG_GPIO_17_PU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_drv(&self) -> REG_GPIO_17_DRV_R {
        REG_GPIO_17_DRV_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_smt(&self) -> REG_GPIO_17_SMT_R {
        REG_GPIO_17_SMT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input enable for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_ie(&self) -> REG_GPIO_17_IE_R {
        REG_GPIO_17_IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Function select for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_func_sel(&self) -> REG_GPIO_16_FUNC_SEL_R {
        REG_GPIO_16_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_pd(&self) -> REG_GPIO_16_PD_R {
        REG_GPIO_16_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_pu(&self) -> REG_GPIO_16_PU_R {
        REG_GPIO_16_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_drv(&self) -> REG_GPIO_16_DRV_R {
        REG_GPIO_16_DRV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_smt(&self) -> REG_GPIO_16_SMT_R {
        REG_GPIO_16_SMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Input enable for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_ie(&self) -> REG_GPIO_16_IE_R {
        REG_GPIO_16_IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27 - Function select for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_func_sel(&mut self) -> REG_GPIO_17_FUNC_SEL_W {
        REG_GPIO_17_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_pd(&mut self) -> REG_GPIO_17_PD_W {
        REG_GPIO_17_PD_W { w: self }
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_pu(&mut self) -> REG_GPIO_17_PU_W {
        REG_GPIO_17_PU_W { w: self }
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_drv(&mut self) -> REG_GPIO_17_DRV_W {
        REG_GPIO_17_DRV_W { w: self }
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_smt(&mut self) -> REG_GPIO_17_SMT_W {
        REG_GPIO_17_SMT_W { w: self }
    }
    #[doc = "Bit 16 - Input enable for GPIO17."]
    #[inline(always)]
    pub fn reg_gpio_17_ie(&mut self) -> REG_GPIO_17_IE_W {
        REG_GPIO_17_IE_W { w: self }
    }
    #[doc = "Bits 8:11 - Function select for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_func_sel(&mut self) -> REG_GPIO_16_FUNC_SEL_W {
        REG_GPIO_16_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_pd(&mut self) -> REG_GPIO_16_PD_W {
        REG_GPIO_16_PD_W { w: self }
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_pu(&mut self) -> REG_GPIO_16_PU_W {
        REG_GPIO_16_PU_W { w: self }
    }
    #[doc = "Bits 2:3 - Driving control enabled for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_drv(&mut self) -> REG_GPIO_16_DRV_W {
        REG_GPIO_16_DRV_W { w: self }
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_smt(&mut self) -> REG_GPIO_16_SMT_W {
        REG_GPIO_16_SMT_W { w: self }
    }
    #[doc = "Bit 0 - Input enable for GPIO16."]
    #[inline(always)]
    pub fn reg_gpio_16_ie(&mut self) -> REG_GPIO_16_IE_W {
        REG_GPIO_16_IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO16, GPIO17 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl8](index.html) module"]
pub struct GPIO_CFGCTL8_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl8::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl8::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_CFGCTL8 to value 0x0e03_0b03"]
impl crate::Resettable for GPIO_CFGCTL8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0e03_0b03
    }
}
