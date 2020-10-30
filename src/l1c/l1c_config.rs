#[doc = "Register `l1c_config` reader"]
pub struct R(crate::R<L1C_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1C_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<L1C_CONFIG_SPEC>> for R {
    fn from(reader: crate::R<L1C_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `l1c_config` writer"]
pub struct W(crate::W<L1C_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1C_CONFIG_SPEC>;
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
impl core::convert::From<crate::W<L1C_CONFIG_SPEC>> for W {
    fn from(writer: crate::W<L1C_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wrap_dis` reader - "]
pub struct WRAP_DIS_R(crate::FieldReader<bool, bool>);
impl WRAP_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRAP_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRAP_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wrap_dis` writer - "]
pub struct WRAP_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WRAP_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `early_resp_dis` reader - "]
pub struct EARLY_RESP_DIS_R(crate::FieldReader<bool, bool>);
impl EARLY_RESP_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EARLY_RESP_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EARLY_RESP_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `early_resp_dis` writer - "]
pub struct EARLY_RESP_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EARLY_RESP_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `l1c_bmx_busy_option_dis` reader - "]
pub struct L1C_BMX_BUSY_OPTION_DIS_R(crate::FieldReader<bool, bool>);
impl L1C_BMX_BUSY_OPTION_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1C_BMX_BUSY_OPTION_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_BMX_BUSY_OPTION_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_bmx_busy_option_dis` writer - "]
pub struct L1C_BMX_BUSY_OPTION_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_BUSY_OPTION_DIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `l1c_bmx_timeout_en` reader - "]
pub struct L1C_BMX_TIMEOUT_EN_R(crate::FieldReader<u8, u8>);
impl L1C_BMX_TIMEOUT_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        L1C_BMX_TIMEOUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_BMX_TIMEOUT_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_bmx_timeout_en` writer - "]
pub struct L1C_BMX_TIMEOUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_TIMEOUT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `l1c_bmx_arb_mode` reader - "]
pub struct L1C_BMX_ARB_MODE_R(crate::FieldReader<u8, u8>);
impl L1C_BMX_ARB_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        L1C_BMX_ARB_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_BMX_ARB_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_bmx_arb_mode` writer - "]
pub struct L1C_BMX_ARB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_ARB_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `l1c_bmx_err_en` reader - "]
pub struct L1C_BMX_ERR_EN_R(crate::FieldReader<bool, bool>);
impl L1C_BMX_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1C_BMX_ERR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_BMX_ERR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_bmx_err_en` writer - "]
pub struct L1C_BMX_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BMX_ERR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `l1c_bypass` reader - "]
pub struct L1C_BYPASS_R(crate::FieldReader<bool, bool>);
impl L1C_BYPASS_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1C_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_bypass` writer - "]
pub struct L1C_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_BYPASS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `irom_2t_access` reader - "]
pub struct IROM_2T_ACCESS_R(crate::FieldReader<bool, bool>);
impl IROM_2T_ACCESS_R {
    pub(crate) fn new(bits: bool) -> Self {
        IROM_2T_ACCESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IROM_2T_ACCESS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irom_2t_access` writer - "]
pub struct IROM_2T_ACCESS_W<'a> {
    w: &'a mut W,
}
impl<'a> IROM_2T_ACCESS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `l1c_way_dis` reader - "]
pub struct L1C_WAY_DIS_R(crate::FieldReader<u8, u8>);
impl L1C_WAY_DIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        L1C_WAY_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_WAY_DIS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_way_dis` writer - "]
pub struct L1C_WAY_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_WAY_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `l1c_invalid_done` reader - "]
pub struct L1C_INVALID_DONE_R(crate::FieldReader<bool, bool>);
impl L1C_INVALID_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1C_INVALID_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_INVALID_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_invalid_done` writer - "]
pub struct L1C_INVALID_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_INVALID_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `l1c_invalid_en` reader - "]
pub struct L1C_INVALID_EN_R(crate::FieldReader<bool, bool>);
impl L1C_INVALID_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1C_INVALID_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_INVALID_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_invalid_en` writer - "]
pub struct L1C_INVALID_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_INVALID_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `l1c_cnt_en` reader - "]
pub struct L1C_CNT_EN_R(crate::FieldReader<bool, bool>);
impl L1C_CNT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1C_CNT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_CNT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_cnt_en` writer - "]
pub struct L1C_CNT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_CNT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `l1c_cacheable` reader - "]
pub struct L1C_CACHEABLE_R(crate::FieldReader<bool, bool>);
impl L1C_CACHEABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        L1C_CACHEABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L1C_CACHEABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `l1c_cacheable` writer - "]
pub struct L1C_CACHEABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> L1C_CACHEABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wrap_dis(&self) -> WRAP_DIS_R {
        WRAP_DIS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn early_resp_dis(&self) -> EARLY_RESP_DIS_R {
        EARLY_RESP_DIS_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn l1c_bmx_busy_option_dis(&self) -> L1C_BMX_BUSY_OPTION_DIS_R {
        L1C_BMX_BUSY_OPTION_DIS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn l1c_bmx_timeout_en(&self) -> L1C_BMX_TIMEOUT_EN_R {
        L1C_BMX_TIMEOUT_EN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn l1c_bmx_arb_mode(&self) -> L1C_BMX_ARB_MODE_R {
        L1C_BMX_ARB_MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn l1c_bmx_err_en(&self) -> L1C_BMX_ERR_EN_R {
        L1C_BMX_ERR_EN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn l1c_bypass(&self) -> L1C_BYPASS_R {
        L1C_BYPASS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn irom_2t_access(&self) -> IROM_2T_ACCESS_R {
        IROM_2T_ACCESS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn l1c_way_dis(&self) -> L1C_WAY_DIS_R {
        L1C_WAY_DIS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn l1c_invalid_done(&self) -> L1C_INVALID_DONE_R {
        L1C_INVALID_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn l1c_invalid_en(&self) -> L1C_INVALID_EN_R {
        L1C_INVALID_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn l1c_cnt_en(&self) -> L1C_CNT_EN_R {
        L1C_CNT_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_cacheable(&self) -> L1C_CACHEABLE_R {
        L1C_CACHEABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn wrap_dis(&mut self) -> WRAP_DIS_W {
        WRAP_DIS_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn early_resp_dis(&mut self) -> EARLY_RESP_DIS_W {
        EARLY_RESP_DIS_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn l1c_bmx_busy_option_dis(&mut self) -> L1C_BMX_BUSY_OPTION_DIS_W {
        L1C_BMX_BUSY_OPTION_DIS_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn l1c_bmx_timeout_en(&mut self) -> L1C_BMX_TIMEOUT_EN_W {
        L1C_BMX_TIMEOUT_EN_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn l1c_bmx_arb_mode(&mut self) -> L1C_BMX_ARB_MODE_W {
        L1C_BMX_ARB_MODE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn l1c_bmx_err_en(&mut self) -> L1C_BMX_ERR_EN_W {
        L1C_BMX_ERR_EN_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn l1c_bypass(&mut self) -> L1C_BYPASS_W {
        L1C_BYPASS_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn irom_2t_access(&mut self) -> IROM_2T_ACCESS_W {
        IROM_2T_ACCESS_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn l1c_way_dis(&mut self) -> L1C_WAY_DIS_W {
        L1C_WAY_DIS_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn l1c_invalid_done(&mut self) -> L1C_INVALID_DONE_W {
        L1C_INVALID_DONE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn l1c_invalid_en(&mut self) -> L1C_INVALID_EN_W {
        L1C_INVALID_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn l1c_cnt_en(&mut self) -> L1C_CNT_EN_W {
        L1C_CNT_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn l1c_cacheable(&mut self) -> L1C_CACHEABLE_W {
        L1C_CACHEABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "l1c_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1c_config](index.html) module"]
pub struct L1C_CONFIG_SPEC;
impl crate::RegisterSpec for L1C_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1c_config::R](R) reader structure"]
impl crate::Readable for L1C_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1c_config::W](W) writer structure"]
impl crate::Writable for L1C_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets l1c_config to value 0"]
impl crate::Resettable for L1C_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
