#[doc = "Register `glb_parm` reader"]
pub struct R(crate::R<GLB_PARM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GLB_PARM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GLB_PARM_SPEC>> for R {
    fn from(reader: crate::R<GLB_PARM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `glb_parm` writer"]
pub struct W(crate::W<GLB_PARM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GLB_PARM_SPEC>;
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
impl core::convert::From<crate::W<GLB_PARM_SPEC>> for W {
    fn from(writer: crate::W<GLB_PARM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart_swap_set` reader - "]
pub struct UART_SWAP_SET_R(crate::FieldReader<u8, u8>);
impl UART_SWAP_SET_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_SWAP_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SWAP_SET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_swap_set` writer - "]
pub struct UART_SWAP_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SWAP_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `p7_jtag_use_io_2_5` reader - "]
pub struct P7_JTAG_USE_IO_2_5_R(crate::FieldReader<bool, bool>);
impl P7_JTAG_USE_IO_2_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7_JTAG_USE_IO_2_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7_JTAG_USE_IO_2_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p7_jtag_use_io_2_5` writer - "]
pub struct P7_JTAG_USE_IO_2_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P7_JTAG_USE_IO_2_5_W<'a> {
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
#[doc = "Field `p6_sdio_use_io_0_5` reader - "]
pub struct P6_SDIO_USE_IO_0_5_R(crate::FieldReader<bool, bool>);
impl P6_SDIO_USE_IO_0_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P6_SDIO_USE_IO_0_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6_SDIO_USE_IO_0_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p6_sdio_use_io_0_5` writer - "]
pub struct P6_SDIO_USE_IO_0_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P6_SDIO_USE_IO_0_5_W<'a> {
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
#[doc = "Field `p5_dac_test_with_jtag` reader - "]
pub struct P5_DAC_TEST_WITH_JTAG_R(crate::FieldReader<bool, bool>);
impl P5_DAC_TEST_WITH_JTAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        P5_DAC_TEST_WITH_JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5_DAC_TEST_WITH_JTAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p5_dac_test_with_jtag` writer - "]
pub struct P5_DAC_TEST_WITH_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> P5_DAC_TEST_WITH_JTAG_W<'a> {
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
#[doc = "Field `p4_adc_test_with_jtag` reader - "]
pub struct P4_ADC_TEST_WITH_JTAG_R(crate::FieldReader<bool, bool>);
impl P4_ADC_TEST_WITH_JTAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4_ADC_TEST_WITH_JTAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4_ADC_TEST_WITH_JTAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p4_adc_test_with_jtag` writer - "]
pub struct P4_ADC_TEST_WITH_JTAG_W<'a> {
    w: &'a mut W,
}
impl<'a> P4_ADC_TEST_WITH_JTAG_W<'a> {
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
#[doc = "Field `p3_cci_use_io_2_5` reader - "]
pub struct P3_CCI_USE_IO_2_5_R(crate::FieldReader<bool, bool>);
impl P3_CCI_USE_IO_2_5_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3_CCI_USE_IO_2_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3_CCI_USE_IO_2_5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p3_cci_use_io_2_5` writer - "]
pub struct P3_CCI_USE_IO_2_5_W<'a> {
    w: &'a mut W,
}
impl<'a> P3_CCI_USE_IO_2_5_W<'a> {
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
#[doc = "Field `p2_dac_test_with_cci` reader - "]
pub struct P2_DAC_TEST_WITH_CCI_R(crate::FieldReader<bool, bool>);
impl P2_DAC_TEST_WITH_CCI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_DAC_TEST_WITH_CCI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_DAC_TEST_WITH_CCI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p2_dac_test_with_cci` writer - "]
pub struct P2_DAC_TEST_WITH_CCI_W<'a> {
    w: &'a mut W,
}
impl<'a> P2_DAC_TEST_WITH_CCI_W<'a> {
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
#[doc = "Field `p1_adc_test_with_cci` reader - "]
pub struct P1_ADC_TEST_WITH_CCI_R(crate::FieldReader<bool, bool>);
impl P1_ADC_TEST_WITH_CCI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1_ADC_TEST_WITH_CCI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1_ADC_TEST_WITH_CCI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `p1_adc_test_with_cci` writer - "]
pub struct P1_ADC_TEST_WITH_CCI_W<'a> {
    w: &'a mut W,
}
impl<'a> P1_ADC_TEST_WITH_CCI_W<'a> {
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
#[doc = "Field `reg_cci_use_sdio_pin` reader - "]
pub struct REG_CCI_USE_SDIO_PIN_R(crate::FieldReader<bool, bool>);
impl REG_CCI_USE_SDIO_PIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_CCI_USE_SDIO_PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CCI_USE_SDIO_PIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_cci_use_sdio_pin` writer - "]
pub struct REG_CCI_USE_SDIO_PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CCI_USE_SDIO_PIN_W<'a> {
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
#[doc = "Field `reg_cci_use_jtag_pin` reader - "]
pub struct REG_CCI_USE_JTAG_PIN_R(crate::FieldReader<bool, bool>);
impl REG_CCI_USE_JTAG_PIN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_CCI_USE_JTAG_PIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_CCI_USE_JTAG_PIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_cci_use_jtag_pin` writer - "]
pub struct REG_CCI_USE_JTAG_PIN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_CCI_USE_JTAG_PIN_W<'a> {
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
#[doc = "Field `reg_spi_0_swap` reader - "]
pub struct REG_SPI_0_SWAP_R(crate::FieldReader<bool, bool>);
impl REG_SPI_0_SWAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_SPI_0_SWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_SPI_0_SWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_spi_0_swap` writer - "]
pub struct REG_SPI_0_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SPI_0_SWAP_W<'a> {
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
#[doc = "Field `reg_spi_0_master_mode` reader - "]
pub struct REG_SPI_0_MASTER_MODE_R(crate::FieldReader<bool, bool>);
impl REG_SPI_0_MASTER_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_SPI_0_MASTER_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_SPI_0_MASTER_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_spi_0_master_mode` writer - "]
pub struct REG_SPI_0_MASTER_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_SPI_0_MASTER_MODE_W<'a> {
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
#[doc = "Field `sel_embedded_sflash` reader - "]
pub struct SEL_EMBEDDED_SFLASH_R(crate::FieldReader<bool, bool>);
impl SEL_EMBEDDED_SFLASH_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEL_EMBEDDED_SFLASH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEL_EMBEDDED_SFLASH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sel_embedded_sflash` writer - "]
pub struct SEL_EMBEDDED_SFLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> SEL_EMBEDDED_SFLASH_W<'a> {
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
#[doc = "Field `swap_sflash_io_3_io_0` reader - "]
pub struct SWAP_SFLASH_IO_3_IO_0_R(crate::FieldReader<bool, bool>);
impl SWAP_SFLASH_IO_3_IO_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWAP_SFLASH_IO_3_IO_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWAP_SFLASH_IO_3_IO_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swap_sflash_io_3_io_0` writer - "]
pub struct SWAP_SFLASH_IO_3_IO_0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWAP_SFLASH_IO_3_IO_0_W<'a> {
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
#[doc = "Field `jtag_swap_set` reader - "]
pub struct JTAG_SWAP_SET_R(crate::FieldReader<u8, u8>);
impl JTAG_SWAP_SET_R {
    pub(crate) fn new(bits: u8) -> Self {
        JTAG_SWAP_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_SWAP_SET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `jtag_swap_set` writer - "]
pub struct JTAG_SWAP_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> JTAG_SWAP_SET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u32 & 0x3f) << 2);
        self.w
    }
}
#[doc = "Field `reg_ext_rst_smt` reader - "]
pub struct REG_EXT_RST_SMT_R(crate::FieldReader<bool, bool>);
impl REG_EXT_RST_SMT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_EXT_RST_SMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_EXT_RST_SMT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_ext_rst_smt` writer - "]
pub struct REG_EXT_RST_SMT_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_EXT_RST_SMT_W<'a> {
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
#[doc = "Field `reg_bd_en` reader - "]
pub struct REG_BD_EN_R(crate::FieldReader<bool, bool>);
impl REG_BD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_BD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_BD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_bd_en` writer - "]
pub struct REG_BD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_BD_EN_W<'a> {
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
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn uart_swap_set(&self) -> UART_SWAP_SET_R {
        UART_SWAP_SET_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn p7_jtag_use_io_2_5(&self) -> P7_JTAG_USE_IO_2_5_R {
        P7_JTAG_USE_IO_2_5_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn p6_sdio_use_io_0_5(&self) -> P6_SDIO_USE_IO_0_5_R {
        P6_SDIO_USE_IO_0_5_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn p5_dac_test_with_jtag(&self) -> P5_DAC_TEST_WITH_JTAG_R {
        P5_DAC_TEST_WITH_JTAG_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn p4_adc_test_with_jtag(&self) -> P4_ADC_TEST_WITH_JTAG_R {
        P4_ADC_TEST_WITH_JTAG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_2_5(&self) -> P3_CCI_USE_IO_2_5_R {
        P3_CCI_USE_IO_2_5_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn p2_dac_test_with_cci(&self) -> P2_DAC_TEST_WITH_CCI_R {
        P2_DAC_TEST_WITH_CCI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn p1_adc_test_with_cci(&self) -> P1_ADC_TEST_WITH_CCI_R {
        P1_ADC_TEST_WITH_CCI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_cci_use_sdio_pin(&self) -> REG_CCI_USE_SDIO_PIN_R {
        REG_CCI_USE_SDIO_PIN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_cci_use_jtag_pin(&self) -> REG_CCI_USE_JTAG_PIN_R {
        REG_CCI_USE_JTAG_PIN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&self) -> REG_SPI_0_SWAP_R {
        REG_SPI_0_SWAP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&self) -> REG_SPI_0_MASTER_MODE_R {
        REG_SPI_0_MASTER_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sel_embedded_sflash(&self) -> SEL_EMBEDDED_SFLASH_R {
        SEL_EMBEDDED_SFLASH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap_sflash_io_3_io_0(&self) -> SWAP_SFLASH_IO_3_IO_0_R {
        SWAP_SFLASH_IO_3_IO_0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn jtag_swap_set(&self) -> JTAG_SWAP_SET_R {
        JTAG_SWAP_SET_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ext_rst_smt(&self) -> REG_EXT_RST_SMT_R {
        REG_EXT_RST_SMT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bd_en(&self) -> REG_BD_EN_R {
        REG_BD_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn uart_swap_set(&mut self) -> UART_SWAP_SET_W {
        UART_SWAP_SET_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn p7_jtag_use_io_2_5(&mut self) -> P7_JTAG_USE_IO_2_5_W {
        P7_JTAG_USE_IO_2_5_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn p6_sdio_use_io_0_5(&mut self) -> P6_SDIO_USE_IO_0_5_W {
        P6_SDIO_USE_IO_0_5_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn p5_dac_test_with_jtag(&mut self) -> P5_DAC_TEST_WITH_JTAG_W {
        P5_DAC_TEST_WITH_JTAG_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn p4_adc_test_with_jtag(&mut self) -> P4_ADC_TEST_WITH_JTAG_W {
        P4_ADC_TEST_WITH_JTAG_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn p3_cci_use_io_2_5(&mut self) -> P3_CCI_USE_IO_2_5_W {
        P3_CCI_USE_IO_2_5_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn p2_dac_test_with_cci(&mut self) -> P2_DAC_TEST_WITH_CCI_W {
        P2_DAC_TEST_WITH_CCI_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn p1_adc_test_with_cci(&mut self) -> P1_ADC_TEST_WITH_CCI_W {
        P1_ADC_TEST_WITH_CCI_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_cci_use_sdio_pin(&mut self) -> REG_CCI_USE_SDIO_PIN_W {
        REG_CCI_USE_SDIO_PIN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reg_cci_use_jtag_pin(&mut self) -> REG_CCI_USE_JTAG_PIN_W {
        REG_CCI_USE_JTAG_PIN_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn reg_spi_0_swap(&mut self) -> REG_SPI_0_SWAP_W {
        REG_SPI_0_SWAP_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn reg_spi_0_master_mode(&mut self) -> REG_SPI_0_MASTER_MODE_W {
        REG_SPI_0_MASTER_MODE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sel_embedded_sflash(&mut self) -> SEL_EMBEDDED_SFLASH_W {
        SEL_EMBEDDED_SFLASH_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swap_sflash_io_3_io_0(&mut self) -> SWAP_SFLASH_IO_3_IO_0_W {
        SWAP_SFLASH_IO_3_IO_0_W { w: self }
    }
    #[doc = "Bits 2:7"]
    #[inline(always)]
    pub fn jtag_swap_set(&mut self) -> JTAG_SWAP_SET_W {
        JTAG_SWAP_SET_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_ext_rst_smt(&mut self) -> REG_EXT_RST_SMT_W {
        REG_EXT_RST_SMT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bd_en(&mut self) -> REG_BD_EN_W {
        REG_BD_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "glb_parm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [glb_parm](index.html) module"]
pub struct GLB_PARM_SPEC;
impl crate::RegisterSpec for GLB_PARM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [glb_parm::R](R) reader structure"]
impl crate::Readable for GLB_PARM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [glb_parm::W](W) writer structure"]
impl crate::Writable for GLB_PARM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets glb_parm to value 0x0001_8300"]
impl crate::Resettable for GLB_PARM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_8300
    }
}
