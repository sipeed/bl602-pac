#[doc = "Register `i2c_config` reader"]
pub struct R(crate::R<I2C_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `i2c_config` writer"]
pub struct W(crate::W<I2C_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C_CONFIG_SPEC>;
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
impl From<crate::W<I2C_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_i2c_deg_cnt` reader - "]
pub struct CR_I2C_DEG_CNT_R(crate::FieldReader<u8, u8>);
impl CR_I2C_DEG_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_I2C_DEG_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_DEG_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_deg_cnt` writer - "]
pub struct CR_I2C_DEG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_DEG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `cr_i2c_pkt_len` reader - "]
pub struct CR_I2C_PKT_LEN_R(crate::FieldReader<u8, u8>);
impl CR_I2C_PKT_LEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_I2C_PKT_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_PKT_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_pkt_len` writer - "]
pub struct CR_I2C_PKT_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_PKT_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `cr_i2c_slv_addr` reader - "]
pub struct CR_I2C_SLV_ADDR_R(crate::FieldReader<u8, u8>);
impl CR_I2C_SLV_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_I2C_SLV_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_SLV_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_slv_addr` writer - "]
pub struct CR_I2C_SLV_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SLV_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `cr_i2c_sub_addr_bc` reader - "]
pub struct CR_I2C_SUB_ADDR_BC_R(crate::FieldReader<u8, u8>);
impl CR_I2C_SUB_ADDR_BC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_I2C_SUB_ADDR_BC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_SUB_ADDR_BC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_sub_addr_bc` writer - "]
pub struct CR_I2C_SUB_ADDR_BC_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SUB_ADDR_BC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `cr_i2c_sub_addr_en` reader - "]
pub struct CR_I2C_SUB_ADDR_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2C_SUB_ADDR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_SUB_ADDR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_SUB_ADDR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_sub_addr_en` writer - "]
pub struct CR_I2C_SUB_ADDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SUB_ADDR_EN_W<'a> {
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
#[doc = "Field `cr_i2c_scl_sync_en` reader - "]
pub struct CR_I2C_SCL_SYNC_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2C_SCL_SYNC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_SCL_SYNC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_SCL_SYNC_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_scl_sync_en` writer - "]
pub struct CR_I2C_SCL_SYNC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_SCL_SYNC_EN_W<'a> {
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
#[doc = "Field `cr_i2c_deg_en` reader - "]
pub struct CR_I2C_DEG_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2C_DEG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_DEG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_DEG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_deg_en` writer - "]
pub struct CR_I2C_DEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_DEG_EN_W<'a> {
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
#[doc = "Field `cr_i2c_pkt_dir` reader - "]
pub struct CR_I2C_PKT_DIR_R(crate::FieldReader<bool, bool>);
impl CR_I2C_PKT_DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_PKT_DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_PKT_DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_pkt_dir` writer - "]
pub struct CR_I2C_PKT_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_PKT_DIR_W<'a> {
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
#[doc = "Field `cr_i2c_m_en` reader - "]
pub struct CR_I2C_M_EN_R(crate::FieldReader<bool, bool>);
impl CR_I2C_M_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_I2C_M_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_I2C_M_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_i2c_m_en` writer - "]
pub struct CR_I2C_M_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_I2C_M_EN_W<'a> {
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
    pub fn cr_i2c_deg_cnt(&self) -> CR_I2C_DEG_CNT_R {
        CR_I2C_DEG_CNT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_pkt_len(&self) -> CR_I2C_PKT_LEN_R {
        CR_I2C_PKT_LEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn cr_i2c_slv_addr(&self) -> CR_I2C_SLV_ADDR_R {
        CR_I2C_SLV_ADDR_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_bc(&self) -> CR_I2C_SUB_ADDR_BC_R {
        CR_I2C_SUB_ADDR_BC_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_en(&self) -> CR_I2C_SUB_ADDR_EN_R {
        CR_I2C_SUB_ADDR_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2c_scl_sync_en(&self) -> CR_I2C_SCL_SYNC_EN_R {
        CR_I2C_SCL_SYNC_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2c_deg_en(&self) -> CR_I2C_DEG_EN_R {
        CR_I2C_DEG_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_pkt_dir(&self) -> CR_I2C_PKT_DIR_R {
        CR_I2C_PKT_DIR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2c_m_en(&self) -> CR_I2C_M_EN_R {
        CR_I2C_M_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_i2c_deg_cnt(&mut self) -> CR_I2C_DEG_CNT_W {
        CR_I2C_DEG_CNT_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_i2c_pkt_len(&mut self) -> CR_I2C_PKT_LEN_W {
        CR_I2C_PKT_LEN_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn cr_i2c_slv_addr(&mut self) -> CR_I2C_SLV_ADDR_W {
        CR_I2C_SLV_ADDR_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_bc(&mut self) -> CR_I2C_SUB_ADDR_BC_W {
        CR_I2C_SUB_ADDR_BC_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_i2c_sub_addr_en(&mut self) -> CR_I2C_SUB_ADDR_EN_W {
        CR_I2C_SUB_ADDR_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_i2c_scl_sync_en(&mut self) -> CR_I2C_SCL_SYNC_EN_W {
        CR_I2C_SCL_SYNC_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_i2c_deg_en(&mut self) -> CR_I2C_DEG_EN_W {
        CR_I2C_DEG_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_i2c_pkt_dir(&mut self) -> CR_I2C_PKT_DIR_W {
        CR_I2C_PKT_DIR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_i2c_m_en(&mut self) -> CR_I2C_M_EN_W {
        CR_I2C_M_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "i2c_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_config](index.html) module"]
pub struct I2C_CONFIG_SPEC;
impl crate::RegisterSpec for I2C_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_config::R](R) reader structure"]
impl crate::Readable for I2C_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c_config::W](W) writer structure"]
impl crate::Writable for I2C_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets i2c_config to value 0x0a"]
impl crate::Resettable for I2C_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a
    }
}
