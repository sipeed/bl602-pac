#[doc = "Register `ef_crc_ctrl_0` reader"]
pub struct R(crate::R<EF_CRC_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_CRC_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EF_CRC_CTRL_0_SPEC>> for R {
    fn from(reader: crate::R<EF_CRC_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_crc_ctrl_0` writer"]
pub struct W(crate::W<EF_CRC_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_CRC_CTRL_0_SPEC>;
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
impl core::convert::From<crate::W<EF_CRC_CTRL_0_SPEC>> for W {
    fn from(writer: crate::W<EF_CRC_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_crc_slp_n` reader - "]
pub struct EF_CRC_SLP_N_R(crate::FieldReader<u16, u16>);
impl EF_CRC_SLP_N_R {
    pub(crate) fn new(bits: u16) -> Self {
        EF_CRC_SLP_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_SLP_N_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_slp_n` writer - "]
pub struct EF_CRC_SLP_N_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_SLP_N_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `ef_crc_lock` reader - "]
pub struct EF_CRC_LOCK_R(crate::FieldReader<bool, bool>);
impl EF_CRC_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_lock` writer - "]
pub struct EF_CRC_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_LOCK_W<'a> {
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
#[doc = "Field `ef_crc_int_set` reader - "]
pub struct EF_CRC_INT_SET_R(crate::FieldReader<bool, bool>);
impl EF_CRC_INT_SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_INT_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_INT_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_int_set` writer - "]
pub struct EF_CRC_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_INT_SET_W<'a> {
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
#[doc = "Field `ef_crc_int_clr` reader - "]
pub struct EF_CRC_INT_CLR_R(crate::FieldReader<bool, bool>);
impl EF_CRC_INT_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_int_clr` writer - "]
pub struct EF_CRC_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_INT_CLR_W<'a> {
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
#[doc = "Field `ef_crc_int` reader - "]
pub struct EF_CRC_INT_R(crate::FieldReader<bool, bool>);
impl EF_CRC_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_din_endian` reader - "]
pub struct EF_CRC_DIN_ENDIAN_R(crate::FieldReader<bool, bool>);
impl EF_CRC_DIN_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_DIN_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_DIN_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_din_endian` writer - "]
pub struct EF_CRC_DIN_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_DIN_ENDIAN_W<'a> {
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
#[doc = "Field `ef_crc_dout_endian` reader - "]
pub struct EF_CRC_DOUT_ENDIAN_R(crate::FieldReader<bool, bool>);
impl EF_CRC_DOUT_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_DOUT_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_DOUT_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_dout_endian` writer - "]
pub struct EF_CRC_DOUT_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_DOUT_ENDIAN_W<'a> {
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
#[doc = "Field `ef_crc_dout_inv_en` reader - "]
pub struct EF_CRC_DOUT_INV_EN_R(crate::FieldReader<bool, bool>);
impl EF_CRC_DOUT_INV_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_DOUT_INV_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_DOUT_INV_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_dout_inv_en` writer - "]
pub struct EF_CRC_DOUT_INV_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_DOUT_INV_EN_W<'a> {
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
#[doc = "Field `ef_crc_error` reader - "]
pub struct EF_CRC_ERROR_R(crate::FieldReader<bool, bool>);
impl EF_CRC_ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_mode` reader - "]
pub struct EF_CRC_MODE_R(crate::FieldReader<bool, bool>);
impl EF_CRC_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_mode` writer - "]
pub struct EF_CRC_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_MODE_W<'a> {
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
#[doc = "Field `ef_crc_en` reader - "]
pub struct EF_CRC_EN_R(crate::FieldReader<bool, bool>);
impl EF_CRC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_en` writer - "]
pub struct EF_CRC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_EN_W<'a> {
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
#[doc = "Field `ef_crc_trig` reader - "]
pub struct EF_CRC_TRIG_R(crate::FieldReader<bool, bool>);
impl EF_CRC_TRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_TRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_crc_trig` writer - "]
pub struct EF_CRC_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CRC_TRIG_W<'a> {
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
#[doc = "Field `ef_crc_busy` reader - "]
pub struct EF_CRC_BUSY_R(crate::FieldReader<bool, bool>);
impl EF_CRC_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CRC_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CRC_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ef_crc_slp_n(&self) -> EF_CRC_SLP_N_R {
        EF_CRC_SLP_N_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_crc_lock(&self) -> EF_CRC_LOCK_R {
        EF_CRC_LOCK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_crc_int_set(&self) -> EF_CRC_INT_SET_R {
        EF_CRC_INT_SET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ef_crc_int_clr(&self) -> EF_CRC_INT_CLR_R {
        EF_CRC_INT_CLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ef_crc_int(&self) -> EF_CRC_INT_R {
        EF_CRC_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_crc_din_endian(&self) -> EF_CRC_DIN_ENDIAN_R {
        EF_CRC_DIN_ENDIAN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_crc_dout_endian(&self) -> EF_CRC_DOUT_ENDIAN_R {
        EF_CRC_DOUT_ENDIAN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_crc_dout_inv_en(&self) -> EF_CRC_DOUT_INV_EN_R {
        EF_CRC_DOUT_INV_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_crc_error(&self) -> EF_CRC_ERROR_R {
        EF_CRC_ERROR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_crc_mode(&self) -> EF_CRC_MODE_R {
        EF_CRC_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_crc_en(&self) -> EF_CRC_EN_R {
        EF_CRC_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_crc_trig(&self) -> EF_CRC_TRIG_R {
        EF_CRC_TRIG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ef_crc_busy(&self) -> EF_CRC_BUSY_R {
        EF_CRC_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ef_crc_slp_n(&mut self) -> EF_CRC_SLP_N_W {
        EF_CRC_SLP_N_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ef_crc_lock(&mut self) -> EF_CRC_LOCK_W {
        EF_CRC_LOCK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ef_crc_int_set(&mut self) -> EF_CRC_INT_SET_W {
        EF_CRC_INT_SET_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ef_crc_int_clr(&mut self) -> EF_CRC_INT_CLR_W {
        EF_CRC_INT_CLR_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_crc_din_endian(&mut self) -> EF_CRC_DIN_ENDIAN_W {
        EF_CRC_DIN_ENDIAN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_crc_dout_endian(&mut self) -> EF_CRC_DOUT_ENDIAN_W {
        EF_CRC_DOUT_ENDIAN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_crc_dout_inv_en(&mut self) -> EF_CRC_DOUT_INV_EN_W {
        EF_CRC_DOUT_INV_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_crc_mode(&mut self) -> EF_CRC_MODE_W {
        EF_CRC_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_crc_en(&mut self) -> EF_CRC_EN_W {
        EF_CRC_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_crc_trig(&mut self) -> EF_CRC_TRIG_W {
        EF_CRC_TRIG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_crc_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_crc_ctrl_0](index.html) module"]
pub struct EF_CRC_CTRL_0_SPEC;
impl crate::RegisterSpec for EF_CRC_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_crc_ctrl_0::R](R) reader structure"]
impl crate::Readable for EF_CRC_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_crc_ctrl_0::W](W) writer structure"]
impl crate::Writable for EF_CRC_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_crc_ctrl_0 to value 0x00ff_0224"]
impl crate::Resettable for EF_CRC_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x00ff_0224
    }
}
