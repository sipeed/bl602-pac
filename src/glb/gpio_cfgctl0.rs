#[doc = "Register `GPIO_CFGCTL0` reader"]
pub struct R(crate::R<GPIO_CFGCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO_CFGCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPIO_CFGCTL0_SPEC>> for R {
    fn from(reader: crate::R<GPIO_CFGCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO_CFGCTL0` writer"]
pub struct W(crate::W<GPIO_CFGCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO_CFGCTL0_SPEC>;
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
impl core::convert::From<crate::W<GPIO_CFGCTL0_SPEC>> for W {
    fn from(writer: crate::W<GPIO_CFGCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REAL_GPIO_1_FUNC_SEL_A {
    #[doc = "0: Function select is reg_gpio_1_func_sel"]
    GLB_GPIO_REAL_MODE_REG = 0,
    #[doc = "1: `1`"]
    GLB_GPIO_REAL_MODE_SDIO = 1,
    #[doc = "12: `1100`"]
    GLB_GPIO_REAL_MODE_RF = 12,
    #[doc = "14: `1110`"]
    GLB_GPIO_REAL_MODE_JTAG = 14,
    #[doc = "15: `1111`"]
    GLB_GPIO_REAL_MODE_CCI = 15,
}
impl From<REAL_GPIO_1_FUNC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REAL_GPIO_1_FUNC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `real_gpio_1_func_sel` reader - "]
pub struct REAL_GPIO_1_FUNC_SEL_R(crate::FieldReader<u8, REAL_GPIO_1_FUNC_SEL_A>);
impl REAL_GPIO_1_FUNC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REAL_GPIO_1_FUNC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REAL_GPIO_1_FUNC_SEL_A> {
        match self.bits {
            0 => Some(REAL_GPIO_1_FUNC_SEL_A::GLB_GPIO_REAL_MODE_REG),
            1 => Some(REAL_GPIO_1_FUNC_SEL_A::GLB_GPIO_REAL_MODE_SDIO),
            12 => Some(REAL_GPIO_1_FUNC_SEL_A::GLB_GPIO_REAL_MODE_RF),
            14 => Some(REAL_GPIO_1_FUNC_SEL_A::GLB_GPIO_REAL_MODE_JTAG),
            15 => Some(REAL_GPIO_1_FUNC_SEL_A::GLB_GPIO_REAL_MODE_CCI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_REG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        **self == REAL_GPIO_1_FUNC_SEL_A::GLB_GPIO_REAL_MODE_REG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_SDIO`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        **self == REAL_GPIO_1_FUNC_SEL_A::GLB_GPIO_REAL_MODE_SDIO
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_RF`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        **self == REAL_GPIO_1_FUNC_SEL_A::GLB_GPIO_REAL_MODE_RF
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_JTAG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        **self == REAL_GPIO_1_FUNC_SEL_A::GLB_GPIO_REAL_MODE_JTAG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_CCI`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        **self == REAL_GPIO_1_FUNC_SEL_A::GLB_GPIO_REAL_MODE_CCI
    }
}
impl core::ops::Deref for REAL_GPIO_1_FUNC_SEL_R {
    type Target = crate::FieldReader<u8, REAL_GPIO_1_FUNC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Function select for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_1_FUNC_SEL_A {
    #[doc = "1: `1`"]
    SDIO_CMD = 1,
    #[doc = "2: `10`"]
    SF_D2 = 2,
    #[doc = "4: `100`"]
    SPI_MOSI_SPI_MISO = 4,
    #[doc = "6: `110`"]
    I2C_SDA = 6,
    #[doc = "7: `111`"]
    UART_SIG1 = 7,
    #[doc = "8: `1000`"]
    PWM_CH1 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_1 = 9,
    #[doc = "10: `1010`"]
    ATEST_IP = 10,
    #[doc = "11: `1011`"]
    SWGPIO_1 = 11,
    #[doc = "14: `1110`"]
    E21_TDI = 14,
}
impl From<REG_GPIO_1_FUNC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_1_FUNC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_1_func_sel` reader - Function select for GPIO1."]
pub struct REG_GPIO_1_FUNC_SEL_R(crate::FieldReader<u8, REG_GPIO_1_FUNC_SEL_A>);
impl REG_GPIO_1_FUNC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_1_FUNC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_1_FUNC_SEL_A> {
        match self.bits {
            1 => Some(REG_GPIO_1_FUNC_SEL_A::SDIO_CMD),
            2 => Some(REG_GPIO_1_FUNC_SEL_A::SF_D2),
            4 => Some(REG_GPIO_1_FUNC_SEL_A::SPI_MOSI_SPI_MISO),
            6 => Some(REG_GPIO_1_FUNC_SEL_A::I2C_SDA),
            7 => Some(REG_GPIO_1_FUNC_SEL_A::UART_SIG1),
            8 => Some(REG_GPIO_1_FUNC_SEL_A::PWM_CH1),
            9 => Some(REG_GPIO_1_FUNC_SEL_A::FEM_GPIO_1),
            10 => Some(REG_GPIO_1_FUNC_SEL_A::ATEST_IP),
            11 => Some(REG_GPIO_1_FUNC_SEL_A::SWGPIO_1),
            14 => Some(REG_GPIO_1_FUNC_SEL_A::E21_TDI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO_CMD`"]
    #[inline(always)]
    pub fn is_sdio_cmd(&self) -> bool {
        **self == REG_GPIO_1_FUNC_SEL_A::SDIO_CMD
    }
    #[doc = "Checks if the value of the field is `SF_D2`"]
    #[inline(always)]
    pub fn is_sf_d2(&self) -> bool {
        **self == REG_GPIO_1_FUNC_SEL_A::SF_D2
    }
    #[doc = "Checks if the value of the field is `SPI_MOSI_SPI_MISO`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        **self == REG_GPIO_1_FUNC_SEL_A::SPI_MOSI_SPI_MISO
    }
    #[doc = "Checks if the value of the field is `I2C_SDA`"]
    #[inline(always)]
    pub fn is_i2c_sda(&self) -> bool {
        **self == REG_GPIO_1_FUNC_SEL_A::I2C_SDA
    }
    #[doc = "Checks if the value of the field is `UART_SIG1`"]
    #[inline(always)]
    pub fn is_uart_sig1(&self) -> bool {
        **self == REG_GPIO_1_FUNC_SEL_A::UART_SIG1
    }
    #[doc = "Checks if the value of the field is `PWM_CH1`"]
    #[inline(always)]
    pub fn is_pwm_ch1(&self) -> bool {
        **self == REG_GPIO_1_FUNC_SEL_A::PWM_CH1
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_1`"]
    #[inline(always)]
    pub fn is_fem_gpio_1(&self) -> bool {
        **self == REG_GPIO_1_FUNC_SEL_A::FEM_GPIO_1
    }
    #[doc = "Checks if the value of the field is `ATEST_IP`"]
    #[inline(always)]
    pub fn is_atest_ip(&self) -> bool {
        **self == REG_GPIO_1_FUNC_SEL_A::ATEST_IP
    }
    #[doc = "Checks if the value of the field is `SWGPIO_1`"]
    #[inline(always)]
    pub fn is_swgpio_1(&self) -> bool {
        **self == REG_GPIO_1_FUNC_SEL_A::SWGPIO_1
    }
    #[doc = "Checks if the value of the field is `E21_TDI`"]
    #[inline(always)]
    pub fn is_e21_tdi(&self) -> bool {
        **self == REG_GPIO_1_FUNC_SEL_A::E21_TDI
    }
}
impl core::ops::Deref for REG_GPIO_1_FUNC_SEL_R {
    type Target = crate::FieldReader<u8, REG_GPIO_1_FUNC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_1_func_sel` writer - Function select for GPIO1."]
pub struct REG_GPIO_1_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_FUNC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_1_FUNC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_cmd(self) -> &'a mut W {
        self.variant(REG_GPIO_1_FUNC_SEL_A::SDIO_CMD)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d2(self) -> &'a mut W {
        self.variant(REG_GPIO_1_FUNC_SEL_A::SF_D2)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut W {
        self.variant(REG_GPIO_1_FUNC_SEL_A::SPI_MOSI_SPI_MISO)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_sda(self) -> &'a mut W {
        self.variant(REG_GPIO_1_FUNC_SEL_A::I2C_SDA)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig1(self) -> &'a mut W {
        self.variant(REG_GPIO_1_FUNC_SEL_A::UART_SIG1)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch1(self) -> &'a mut W {
        self.variant(REG_GPIO_1_FUNC_SEL_A::PWM_CH1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_1(self) -> &'a mut W {
        self.variant(REG_GPIO_1_FUNC_SEL_A::FEM_GPIO_1)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn atest_ip(self) -> &'a mut W {
        self.variant(REG_GPIO_1_FUNC_SEL_A::ATEST_IP)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_1(self) -> &'a mut W {
        self.variant(REG_GPIO_1_FUNC_SEL_A::SWGPIO_1)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tdi(self) -> &'a mut W {
        self.variant(REG_GPIO_1_FUNC_SEL_A::E21_TDI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Pull Down Resistor for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_1_PD_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_1_PD_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_1_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_pd` reader - Pull Down Resistor for GPIO1."]
pub struct REG_GPIO_1_PD_R(crate::FieldReader<bool, REG_GPIO_1_PD_A>);
impl REG_GPIO_1_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_1_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_1_PD_A {
        match self.bits {
            false => REG_GPIO_1_PD_A::DISABLED,
            true => REG_GPIO_1_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_1_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_1_PD_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_1_PD_R {
    type Target = crate::FieldReader<bool, REG_GPIO_1_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_1_pd` writer - Pull Down Resistor for GPIO1."]
pub struct REG_GPIO_1_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_1_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_PD_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_PD_A::ENABLED)
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
#[doc = "Pull Up Resistor for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_1_PU_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_1_PU_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_1_PU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_pu` reader - Pull Up Resistor for GPIO1."]
pub struct REG_GPIO_1_PU_R(crate::FieldReader<bool, REG_GPIO_1_PU_A>);
impl REG_GPIO_1_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_1_PU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_1_PU_A {
        match self.bits {
            false => REG_GPIO_1_PU_A::DISABLED,
            true => REG_GPIO_1_PU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_1_PU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_1_PU_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_1_PU_R {
    type Target = crate::FieldReader<bool, REG_GPIO_1_PU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_1_pu` writer - Pull Up Resistor for GPIO1."]
pub struct REG_GPIO_1_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_PU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_1_PU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_PU_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_PU_A::ENABLED)
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
#[doc = "Driving control enabled for GPIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_1_DRV_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_1_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_1_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_1_drv` reader - Driving control enabled for GPIO1."]
pub struct REG_GPIO_1_DRV_R(crate::FieldReader<u8, REG_GPIO_1_DRV_A>);
impl REG_GPIO_1_DRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_1_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_1_DRV_A> {
        match self.bits {
            0 => Some(REG_GPIO_1_DRV_A::DISABLED),
            1 => Some(REG_GPIO_1_DRV_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_1_DRV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_1_DRV_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_1_DRV_R {
    type Target = crate::FieldReader<u8, REG_GPIO_1_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_1_drv` writer - Driving control enabled for GPIO1."]
pub struct REG_GPIO_1_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_1_DRV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_DRV_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_DRV_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "Schmitt trigger enabled for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_1_SMT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_1_SMT_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_1_SMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_smt` reader - Schmitt trigger enabled for GPIO1."]
pub struct REG_GPIO_1_SMT_R(crate::FieldReader<bool, REG_GPIO_1_SMT_A>);
impl REG_GPIO_1_SMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_1_SMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_1_SMT_A {
        match self.bits {
            false => REG_GPIO_1_SMT_A::DISABLED,
            true => REG_GPIO_1_SMT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_1_SMT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_1_SMT_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_1_SMT_R {
    type Target = crate::FieldReader<bool, REG_GPIO_1_SMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_1_smt` writer - Schmitt trigger enabled for GPIO1."]
pub struct REG_GPIO_1_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_SMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_1_SMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_SMT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_SMT_A::ENABLED)
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
#[doc = "Input enable for GPIO1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_1_IE_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_1_IE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_1_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_1_ie` reader - Input enable for GPIO1."]
pub struct REG_GPIO_1_IE_R(crate::FieldReader<bool, REG_GPIO_1_IE_A>);
impl REG_GPIO_1_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_1_IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_1_IE_A {
        match self.bits {
            false => REG_GPIO_1_IE_A::DISABLED,
            true => REG_GPIO_1_IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_1_IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_1_IE_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_1_IE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_1_IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_1_ie` writer - Input enable for GPIO1."]
pub struct REG_GPIO_1_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_1_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_1_IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_IE_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_1_IE_A::ENABLED)
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
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REAL_GPIO_0_FUNC_SEL_A {
    #[doc = "0: Function select is reg_gpio_0_func_sel"]
    GLB_GPIO_REAL_MODE_REG = 0,
    #[doc = "1: `1`"]
    GLB_GPIO_REAL_MODE_SDIO = 1,
    #[doc = "12: `1100`"]
    GLB_GPIO_REAL_MODE_RF = 12,
    #[doc = "14: `1110`"]
    GLB_GPIO_REAL_MODE_JTAG = 14,
    #[doc = "15: `1111`"]
    GLB_GPIO_REAL_MODE_CCI = 15,
}
impl From<REAL_GPIO_0_FUNC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REAL_GPIO_0_FUNC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `real_gpio_0_func_sel` reader - "]
pub struct REAL_GPIO_0_FUNC_SEL_R(crate::FieldReader<u8, REAL_GPIO_0_FUNC_SEL_A>);
impl REAL_GPIO_0_FUNC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REAL_GPIO_0_FUNC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REAL_GPIO_0_FUNC_SEL_A> {
        match self.bits {
            0 => Some(REAL_GPIO_0_FUNC_SEL_A::GLB_GPIO_REAL_MODE_REG),
            1 => Some(REAL_GPIO_0_FUNC_SEL_A::GLB_GPIO_REAL_MODE_SDIO),
            12 => Some(REAL_GPIO_0_FUNC_SEL_A::GLB_GPIO_REAL_MODE_RF),
            14 => Some(REAL_GPIO_0_FUNC_SEL_A::GLB_GPIO_REAL_MODE_JTAG),
            15 => Some(REAL_GPIO_0_FUNC_SEL_A::GLB_GPIO_REAL_MODE_CCI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_REG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_reg(&self) -> bool {
        **self == REAL_GPIO_0_FUNC_SEL_A::GLB_GPIO_REAL_MODE_REG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_SDIO`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_sdio(&self) -> bool {
        **self == REAL_GPIO_0_FUNC_SEL_A::GLB_GPIO_REAL_MODE_SDIO
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_RF`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_rf(&self) -> bool {
        **self == REAL_GPIO_0_FUNC_SEL_A::GLB_GPIO_REAL_MODE_RF
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_JTAG`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_jtag(&self) -> bool {
        **self == REAL_GPIO_0_FUNC_SEL_A::GLB_GPIO_REAL_MODE_JTAG
    }
    #[doc = "Checks if the value of the field is `GLB_GPIO_REAL_MODE_CCI`"]
    #[inline(always)]
    pub fn is_glb_gpio_real_mode_cci(&self) -> bool {
        **self == REAL_GPIO_0_FUNC_SEL_A::GLB_GPIO_REAL_MODE_CCI
    }
}
impl core::ops::Deref for REAL_GPIO_0_FUNC_SEL_R {
    type Target = crate::FieldReader<u8, REAL_GPIO_0_FUNC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Function select for GPIO0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_0_FUNC_SEL_A {
    #[doc = "1: `1`"]
    SDIO_CLK = 1,
    #[doc = "2: `10`"]
    SF_D1 = 2,
    #[doc = "4: `100`"]
    SPI_MOSI_SPI_MISO = 4,
    #[doc = "6: `110`"]
    I2C_SCL = 6,
    #[doc = "7: `111`"]
    UART_SIG0 = 7,
    #[doc = "8: `1000`"]
    PWM_CH0 = 8,
    #[doc = "9: `1001`"]
    FEM_GPIO_0 = 9,
    #[doc = "10: `1010`"]
    ATEST_IN = 10,
    #[doc = "11: `1011`"]
    SWGPIO_0 = 11,
    #[doc = "14: `1110`"]
    E21_TMS = 14,
}
impl From<REG_GPIO_0_FUNC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_0_FUNC_SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_0_func_sel` reader - Function select for GPIO0."]
pub struct REG_GPIO_0_FUNC_SEL_R(crate::FieldReader<u8, REG_GPIO_0_FUNC_SEL_A>);
impl REG_GPIO_0_FUNC_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_0_FUNC_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_0_FUNC_SEL_A> {
        match self.bits {
            1 => Some(REG_GPIO_0_FUNC_SEL_A::SDIO_CLK),
            2 => Some(REG_GPIO_0_FUNC_SEL_A::SF_D1),
            4 => Some(REG_GPIO_0_FUNC_SEL_A::SPI_MOSI_SPI_MISO),
            6 => Some(REG_GPIO_0_FUNC_SEL_A::I2C_SCL),
            7 => Some(REG_GPIO_0_FUNC_SEL_A::UART_SIG0),
            8 => Some(REG_GPIO_0_FUNC_SEL_A::PWM_CH0),
            9 => Some(REG_GPIO_0_FUNC_SEL_A::FEM_GPIO_0),
            10 => Some(REG_GPIO_0_FUNC_SEL_A::ATEST_IN),
            11 => Some(REG_GPIO_0_FUNC_SEL_A::SWGPIO_0),
            14 => Some(REG_GPIO_0_FUNC_SEL_A::E21_TMS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SDIO_CLK`"]
    #[inline(always)]
    pub fn is_sdio_clk(&self) -> bool {
        **self == REG_GPIO_0_FUNC_SEL_A::SDIO_CLK
    }
    #[doc = "Checks if the value of the field is `SF_D1`"]
    #[inline(always)]
    pub fn is_sf_d1(&self) -> bool {
        **self == REG_GPIO_0_FUNC_SEL_A::SF_D1
    }
    #[doc = "Checks if the value of the field is `SPI_MOSI_SPI_MISO`"]
    #[inline(always)]
    pub fn is_spi_mosi_spi_miso(&self) -> bool {
        **self == REG_GPIO_0_FUNC_SEL_A::SPI_MOSI_SPI_MISO
    }
    #[doc = "Checks if the value of the field is `I2C_SCL`"]
    #[inline(always)]
    pub fn is_i2c_scl(&self) -> bool {
        **self == REG_GPIO_0_FUNC_SEL_A::I2C_SCL
    }
    #[doc = "Checks if the value of the field is `UART_SIG0`"]
    #[inline(always)]
    pub fn is_uart_sig0(&self) -> bool {
        **self == REG_GPIO_0_FUNC_SEL_A::UART_SIG0
    }
    #[doc = "Checks if the value of the field is `PWM_CH0`"]
    #[inline(always)]
    pub fn is_pwm_ch0(&self) -> bool {
        **self == REG_GPIO_0_FUNC_SEL_A::PWM_CH0
    }
    #[doc = "Checks if the value of the field is `FEM_GPIO_0`"]
    #[inline(always)]
    pub fn is_fem_gpio_0(&self) -> bool {
        **self == REG_GPIO_0_FUNC_SEL_A::FEM_GPIO_0
    }
    #[doc = "Checks if the value of the field is `ATEST_IN`"]
    #[inline(always)]
    pub fn is_atest_in(&self) -> bool {
        **self == REG_GPIO_0_FUNC_SEL_A::ATEST_IN
    }
    #[doc = "Checks if the value of the field is `SWGPIO_0`"]
    #[inline(always)]
    pub fn is_swgpio_0(&self) -> bool {
        **self == REG_GPIO_0_FUNC_SEL_A::SWGPIO_0
    }
    #[doc = "Checks if the value of the field is `E21_TMS`"]
    #[inline(always)]
    pub fn is_e21_tms(&self) -> bool {
        **self == REG_GPIO_0_FUNC_SEL_A::E21_TMS
    }
}
impl core::ops::Deref for REG_GPIO_0_FUNC_SEL_R {
    type Target = crate::FieldReader<u8, REG_GPIO_0_FUNC_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_0_func_sel` writer - Function select for GPIO0."]
pub struct REG_GPIO_0_FUNC_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_FUNC_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_0_FUNC_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn sdio_clk(self) -> &'a mut W {
        self.variant(REG_GPIO_0_FUNC_SEL_A::SDIO_CLK)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn sf_d1(self) -> &'a mut W {
        self.variant(REG_GPIO_0_FUNC_SEL_A::SF_D1)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn spi_mosi_spi_miso(self) -> &'a mut W {
        self.variant(REG_GPIO_0_FUNC_SEL_A::SPI_MOSI_SPI_MISO)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn i2c_scl(self) -> &'a mut W {
        self.variant(REG_GPIO_0_FUNC_SEL_A::I2C_SCL)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn uart_sig0(self) -> &'a mut W {
        self.variant(REG_GPIO_0_FUNC_SEL_A::UART_SIG0)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn pwm_ch0(self) -> &'a mut W {
        self.variant(REG_GPIO_0_FUNC_SEL_A::PWM_CH0)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn fem_gpio_0(self) -> &'a mut W {
        self.variant(REG_GPIO_0_FUNC_SEL_A::FEM_GPIO_0)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn atest_in(self) -> &'a mut W {
        self.variant(REG_GPIO_0_FUNC_SEL_A::ATEST_IN)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn swgpio_0(self) -> &'a mut W {
        self.variant(REG_GPIO_0_FUNC_SEL_A::SWGPIO_0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn e21_tms(self) -> &'a mut W {
        self.variant(REG_GPIO_0_FUNC_SEL_A::E21_TMS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Pull Down Resistor for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_0_PD_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_0_PD_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_0_PD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_pd` reader - Pull Down Resistor for GPIO0."]
pub struct REG_GPIO_0_PD_R(crate::FieldReader<bool, REG_GPIO_0_PD_A>);
impl REG_GPIO_0_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_0_PD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_0_PD_A {
        match self.bits {
            false => REG_GPIO_0_PD_A::DISABLED,
            true => REG_GPIO_0_PD_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_0_PD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_0_PD_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_0_PD_R {
    type Target = crate::FieldReader<bool, REG_GPIO_0_PD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_0_pd` writer - Pull Down Resistor for GPIO0."]
pub struct REG_GPIO_0_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_PD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_0_PD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_PD_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_PD_A::ENABLED)
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
#[doc = "Pull Up Resistor for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_0_PU_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_0_PU_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_0_PU_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_pu` reader - Pull Up Resistor for GPIO0."]
pub struct REG_GPIO_0_PU_R(crate::FieldReader<bool, REG_GPIO_0_PU_A>);
impl REG_GPIO_0_PU_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_0_PU_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_0_PU_A {
        match self.bits {
            false => REG_GPIO_0_PU_A::DISABLED,
            true => REG_GPIO_0_PU_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_0_PU_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_0_PU_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_0_PU_R {
    type Target = crate::FieldReader<bool, REG_GPIO_0_PU_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_0_pu` writer - Pull Up Resistor for GPIO0."]
pub struct REG_GPIO_0_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_PU_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_0_PU_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_PU_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_PU_A::ENABLED)
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
#[doc = "Driving control for GPIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum REG_GPIO_0_DRV_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_0_DRV_A> for u8 {
    #[inline(always)]
    fn from(variant: REG_GPIO_0_DRV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `reg_gpio_0_drv` reader - Driving control for GPIO0."]
pub struct REG_GPIO_0_DRV_R(crate::FieldReader<u8, REG_GPIO_0_DRV_A>);
impl REG_GPIO_0_DRV_R {
    pub(crate) fn new(bits: u8) -> Self {
        REG_GPIO_0_DRV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REG_GPIO_0_DRV_A> {
        match self.bits {
            0 => Some(REG_GPIO_0_DRV_A::DISABLED),
            1 => Some(REG_GPIO_0_DRV_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_0_DRV_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_0_DRV_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_0_DRV_R {
    type Target = crate::FieldReader<u8, REG_GPIO_0_DRV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_0_drv` writer - Driving control for GPIO0."]
pub struct REG_GPIO_0_DRV_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_DRV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_0_DRV_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_DRV_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_DRV_A::ENABLED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Schmitt trigger enabled for GPIO0.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_0_SMT_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_0_SMT_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_0_SMT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_smt` reader - Schmitt trigger enabled for GPIO0."]
pub struct REG_GPIO_0_SMT_R(crate::FieldReader<bool, REG_GPIO_0_SMT_A>);
impl REG_GPIO_0_SMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_0_SMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_0_SMT_A {
        match self.bits {
            false => REG_GPIO_0_SMT_A::DISABLED,
            true => REG_GPIO_0_SMT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_0_SMT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_0_SMT_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_0_SMT_R {
    type Target = crate::FieldReader<bool, REG_GPIO_0_SMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_0_smt` writer - Schmitt trigger enabled for GPIO0."]
pub struct REG_GPIO_0_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_SMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_0_SMT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_SMT_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_SMT_A::ENABLED)
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
#[doc = "GPIO0 input enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REG_GPIO_0_IE_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<REG_GPIO_0_IE_A> for bool {
    #[inline(always)]
    fn from(variant: REG_GPIO_0_IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `reg_gpio_0_ie` reader - GPIO0 input enable."]
pub struct REG_GPIO_0_IE_R(crate::FieldReader<bool, REG_GPIO_0_IE_A>);
impl REG_GPIO_0_IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_GPIO_0_IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REG_GPIO_0_IE_A {
        match self.bits {
            false => REG_GPIO_0_IE_A::DISABLED,
            true => REG_GPIO_0_IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == REG_GPIO_0_IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == REG_GPIO_0_IE_A::ENABLED
    }
}
impl core::ops::Deref for REG_GPIO_0_IE_R {
    type Target = crate::FieldReader<bool, REG_GPIO_0_IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_gpio_0_ie` writer - GPIO0 input enable."]
pub struct REG_GPIO_0_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_GPIO_0_IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REG_GPIO_0_IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_IE_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REG_GPIO_0_IE_A::ENABLED)
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
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn real_gpio_1_func_sel(&self) -> REAL_GPIO_1_FUNC_SEL_R {
        REAL_GPIO_1_FUNC_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Function select for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_func_sel(&self) -> REG_GPIO_1_FUNC_SEL_R {
        REG_GPIO_1_FUNC_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_pd(&self) -> REG_GPIO_1_PD_R {
        REG_GPIO_1_PD_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_pu(&self) -> REG_GPIO_1_PU_R {
        REG_GPIO_1_PU_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_drv(&self) -> REG_GPIO_1_DRV_R {
        REG_GPIO_1_DRV_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_smt(&self) -> REG_GPIO_1_SMT_R {
        REG_GPIO_1_SMT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input enable for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_ie(&self) -> REG_GPIO_1_IE_R {
        REG_GPIO_1_IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn real_gpio_0_func_sel(&self) -> REAL_GPIO_0_FUNC_SEL_R {
        REAL_GPIO_0_FUNC_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Function select for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_func_sel(&self) -> REG_GPIO_0_FUNC_SEL_R {
        REG_GPIO_0_FUNC_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_pd(&self) -> REG_GPIO_0_PD_R {
        REG_GPIO_0_PD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_pu(&self) -> REG_GPIO_0_PU_R {
        REG_GPIO_0_PU_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Driving control for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_drv(&self) -> REG_GPIO_0_DRV_R {
        REG_GPIO_0_DRV_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_smt(&self) -> REG_GPIO_0_SMT_R {
        REG_GPIO_0_SMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - GPIO0 input enable."]
    #[inline(always)]
    pub fn reg_gpio_0_ie(&self) -> REG_GPIO_0_IE_R {
        REG_GPIO_0_IE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27 - Function select for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_func_sel(&mut self) -> REG_GPIO_1_FUNC_SEL_W {
        REG_GPIO_1_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 21 - Pull Down Resistor for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_pd(&mut self) -> REG_GPIO_1_PD_W {
        REG_GPIO_1_PD_W { w: self }
    }
    #[doc = "Bit 20 - Pull Up Resistor for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_pu(&mut self) -> REG_GPIO_1_PU_W {
        REG_GPIO_1_PU_W { w: self }
    }
    #[doc = "Bits 18:19 - Driving control enabled for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_drv(&mut self) -> REG_GPIO_1_DRV_W {
        REG_GPIO_1_DRV_W { w: self }
    }
    #[doc = "Bit 17 - Schmitt trigger enabled for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_smt(&mut self) -> REG_GPIO_1_SMT_W {
        REG_GPIO_1_SMT_W { w: self }
    }
    #[doc = "Bit 16 - Input enable for GPIO1."]
    #[inline(always)]
    pub fn reg_gpio_1_ie(&mut self) -> REG_GPIO_1_IE_W {
        REG_GPIO_1_IE_W { w: self }
    }
    #[doc = "Bits 8:11 - Function select for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_func_sel(&mut self) -> REG_GPIO_0_FUNC_SEL_W {
        REG_GPIO_0_FUNC_SEL_W { w: self }
    }
    #[doc = "Bit 5 - Pull Down Resistor for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_pd(&mut self) -> REG_GPIO_0_PD_W {
        REG_GPIO_0_PD_W { w: self }
    }
    #[doc = "Bit 4 - Pull Up Resistor for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_pu(&mut self) -> REG_GPIO_0_PU_W {
        REG_GPIO_0_PU_W { w: self }
    }
    #[doc = "Bits 2:3 - Driving control for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_drv(&mut self) -> REG_GPIO_0_DRV_W {
        REG_GPIO_0_DRV_W { w: self }
    }
    #[doc = "Bit 1 - Schmitt trigger enabled for GPIO0."]
    #[inline(always)]
    pub fn reg_gpio_0_smt(&mut self) -> REG_GPIO_0_SMT_W {
        REG_GPIO_0_SMT_W { w: self }
    }
    #[doc = "Bit 0 - GPIO0 input enable."]
    #[inline(always)]
    pub fn reg_gpio_0_ie(&mut self) -> REG_GPIO_0_IE_W {
        REG_GPIO_0_IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO0, GPIO1 configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_cfgctl0](index.html) module"]
pub struct GPIO_CFGCTL0_SPEC;
impl crate::RegisterSpec for GPIO_CFGCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio_cfgctl0::R](R) reader structure"]
impl crate::Readable for GPIO_CFGCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio_cfgctl0::W](W) writer structure"]
impl crate::Writable for GPIO_CFGCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GPIO_CFGCTL0 to value 0x1103_1103"]
impl crate::Resettable for GPIO_CFGCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1103_1103
    }
}
