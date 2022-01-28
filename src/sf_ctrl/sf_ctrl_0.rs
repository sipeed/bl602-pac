#[doc = "Register `sf_ctrl_0` reader"]
pub struct R(crate::R<SF_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_0` writer"]
pub struct W(crate::W<SF_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_0_SPEC>;
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
impl From<crate::W<SF_CTRL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_id` reader - "]
pub struct SF_ID_R(crate::FieldReader<u8, u8>);
impl SF_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_id` writer - "]
pub struct SF_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `sf_aes_iv_endian` reader - "]
pub struct SF_AES_IV_ENDIAN_R(crate::FieldReader<bool, bool>);
impl SF_AES_IV_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AES_IV_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_IV_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_iv_endian` writer - "]
pub struct SF_AES_IV_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_IV_ENDIAN_W<'a> {
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
#[doc = "Field `sf_aes_key_endian` reader - "]
pub struct SF_AES_KEY_ENDIAN_R(crate::FieldReader<bool, bool>);
impl SF_AES_KEY_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AES_KEY_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_KEY_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_key_endian` writer - "]
pub struct SF_AES_KEY_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_KEY_ENDIAN_W<'a> {
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
#[doc = "Field `sf_aes_ctr_plus_en` reader - "]
pub struct SF_AES_CTR_PLUS_EN_R(crate::FieldReader<bool, bool>);
impl SF_AES_CTR_PLUS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AES_CTR_PLUS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_CTR_PLUS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_ctr_plus_en` writer - "]
pub struct SF_AES_CTR_PLUS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_CTR_PLUS_EN_W<'a> {
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
#[doc = "Field `sf_aes_dout_endian` reader - "]
pub struct SF_AES_DOUT_ENDIAN_R(crate::FieldReader<bool, bool>);
impl SF_AES_DOUT_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AES_DOUT_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_DOUT_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_dout_endian` writer - "]
pub struct SF_AES_DOUT_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_DOUT_ENDIAN_W<'a> {
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
#[doc = "Field `sf_aes_dly_mode` reader - "]
pub struct SF_AES_DLY_MODE_R(crate::FieldReader<bool, bool>);
impl SF_AES_DLY_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_AES_DLY_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_AES_DLY_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_aes_dly_mode` writer - "]
pub struct SF_AES_DLY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_AES_DLY_MODE_W<'a> {
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
#[doc = "Field `sf_if_int_set` reader - "]
pub struct SF_IF_INT_SET_R(crate::FieldReader<bool, bool>);
impl SF_IF_INT_SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_INT_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_INT_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_int_set` writer - "]
pub struct SF_IF_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_INT_SET_W<'a> {
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
#[doc = "Field `sf_if_int_clr` reader - "]
pub struct SF_IF_INT_CLR_R(crate::FieldReader<bool, bool>);
impl SF_IF_INT_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_int_clr` writer - "]
pub struct SF_IF_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_INT_CLR_W<'a> {
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
#[doc = "Field `sf_if_int` reader - "]
pub struct SF_IF_INT_R(crate::FieldReader<bool, bool>);
impl SF_IF_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_read_dly_en` reader - "]
pub struct SF_IF_READ_DLY_EN_R(crate::FieldReader<bool, bool>);
impl SF_IF_READ_DLY_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_READ_DLY_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_READ_DLY_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_read_dly_en` writer - "]
pub struct SF_IF_READ_DLY_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_READ_DLY_EN_W<'a> {
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
#[doc = "Field `sf_if_read_dly_n` reader - "]
pub struct SF_IF_READ_DLY_N_R(crate::FieldReader<u8, u8>);
impl SF_IF_READ_DLY_N_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IF_READ_DLY_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_READ_DLY_N_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_read_dly_n` writer - "]
pub struct SF_IF_READ_DLY_N_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_READ_DLY_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `sf_clk_sahb_sram_sel` reader - "]
pub struct SF_CLK_SAHB_SRAM_SEL_R(crate::FieldReader<bool, bool>);
impl SF_CLK_SAHB_SRAM_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CLK_SAHB_SRAM_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CLK_SAHB_SRAM_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_clk_sahb_sram_sel` writer - "]
pub struct SF_CLK_SAHB_SRAM_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_SAHB_SRAM_SEL_W<'a> {
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
#[doc = "Field `sf_clk_out_inv_sel` reader - "]
pub struct SF_CLK_OUT_INV_SEL_R(crate::FieldReader<bool, bool>);
impl SF_CLK_OUT_INV_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CLK_OUT_INV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CLK_OUT_INV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_clk_out_inv_sel` writer - "]
pub struct SF_CLK_OUT_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_OUT_INV_SEL_W<'a> {
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
#[doc = "Field `sf_clk_out_gate_en` reader - "]
pub struct SF_CLK_OUT_GATE_EN_R(crate::FieldReader<bool, bool>);
impl SF_CLK_OUT_GATE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CLK_OUT_GATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CLK_OUT_GATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_clk_out_gate_en` writer - "]
pub struct SF_CLK_OUT_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_OUT_GATE_EN_W<'a> {
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
#[doc = "Field `sf_clk_sf_rx_inv_sel` reader - "]
pub struct SF_CLK_SF_RX_INV_SEL_R(crate::FieldReader<bool, bool>);
impl SF_CLK_SF_RX_INV_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CLK_SF_RX_INV_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CLK_SF_RX_INV_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_clk_sf_rx_inv_sel` writer - "]
pub struct SF_CLK_SF_RX_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CLK_SF_RX_INV_SEL_W<'a> {
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
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sf_id(&self) -> SF_ID_R {
        SF_ID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_aes_iv_endian(&self) -> SF_AES_IV_ENDIAN_R {
        SF_AES_IV_ENDIAN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sf_aes_key_endian(&self) -> SF_AES_KEY_ENDIAN_R {
        SF_AES_KEY_ENDIAN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sf_aes_ctr_plus_en(&self) -> SF_AES_CTR_PLUS_EN_R {
        SF_AES_CTR_PLUS_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sf_aes_dout_endian(&self) -> SF_AES_DOUT_ENDIAN_R {
        SF_AES_DOUT_ENDIAN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sf_aes_dly_mode(&self) -> SF_AES_DLY_MODE_R {
        SF_AES_DLY_MODE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_int_set(&self) -> SF_IF_INT_SET_R {
        SF_IF_INT_SET_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_int_clr(&self) -> SF_IF_INT_CLR_R {
        SF_IF_INT_CLR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn sf_if_int(&self) -> SF_IF_INT_R {
        SF_IF_INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_if_read_dly_en(&self) -> SF_IF_READ_DLY_EN_R {
        SF_IF_READ_DLY_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_if_read_dly_n(&self) -> SF_IF_READ_DLY_N_R {
        SF_IF_READ_DLY_N_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_clk_sahb_sram_sel(&self) -> SF_CLK_SAHB_SRAM_SEL_R {
        SF_CLK_SAHB_SRAM_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_clk_out_inv_sel(&self) -> SF_CLK_OUT_INV_SEL_R {
        SF_CLK_OUT_INV_SEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_clk_out_gate_en(&self) -> SF_CLK_OUT_GATE_EN_R {
        SF_CLK_OUT_GATE_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_clk_sf_rx_inv_sel(&self) -> SF_CLK_SF_RX_INV_SEL_R {
        SF_CLK_SF_RX_INV_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sf_id(&mut self) -> SF_ID_W {
        SF_ID_W { w: self }
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sf_aes_iv_endian(&mut self) -> SF_AES_IV_ENDIAN_W {
        SF_AES_IV_ENDIAN_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn sf_aes_key_endian(&mut self) -> SF_AES_KEY_ENDIAN_W {
        SF_AES_KEY_ENDIAN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn sf_aes_ctr_plus_en(&mut self) -> SF_AES_CTR_PLUS_EN_W {
        SF_AES_CTR_PLUS_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn sf_aes_dout_endian(&mut self) -> SF_AES_DOUT_ENDIAN_W {
        SF_AES_DOUT_ENDIAN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn sf_aes_dly_mode(&mut self) -> SF_AES_DLY_MODE_W {
        SF_AES_DLY_MODE_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sf_if_int_set(&mut self) -> SF_IF_INT_SET_W {
        SF_IF_INT_SET_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sf_if_int_clr(&mut self) -> SF_IF_INT_CLR_W {
        SF_IF_INT_CLR_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sf_if_read_dly_en(&mut self) -> SF_IF_READ_DLY_EN_W {
        SF_IF_READ_DLY_EN_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn sf_if_read_dly_n(&mut self) -> SF_IF_READ_DLY_N_W {
        SF_IF_READ_DLY_N_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_clk_sahb_sram_sel(&mut self) -> SF_CLK_SAHB_SRAM_SEL_W {
        SF_CLK_SAHB_SRAM_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_clk_out_inv_sel(&mut self) -> SF_CLK_OUT_INV_SEL_W {
        SF_CLK_OUT_INV_SEL_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_clk_out_gate_en(&mut self) -> SF_CLK_OUT_GATE_EN_W {
        SF_CLK_OUT_GATE_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_clk_sf_rx_inv_sel(&mut self) -> SF_CLK_SF_RX_INV_SEL_W {
        SF_CLK_SF_RX_INV_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_0](index.html) module"]
pub struct SF_CTRL_0_SPEC;
impl crate::RegisterSpec for SF_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_0::R](R) reader structure"]
impl crate::Readable for SF_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_0::W](W) writer structure"]
impl crate::Writable for SF_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_ctrl_0 to value 0x1ad2_001c"]
impl crate::Resettable for SF_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1ad2_001c
    }
}
