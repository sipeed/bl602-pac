#[doc = "Register `sd_status` reader"]
pub struct R(crate::R<SD_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SD_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SD_STATUS_SPEC>> for R {
    fn from(reader: crate::R<SD_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sd_status` writer"]
pub struct W(crate::W<SD_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SD_STATUS_SPEC>;
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
impl core::convert::From<crate::W<SD_STATUS_SPEC>> for W {
    fn from(writer: crate::W<SD_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sd_dbg_ena` reader - "]
pub struct SD_DBG_ENA_R(crate::FieldReader<u8, u8>);
impl SD_DBG_ENA_R {
    pub(crate) fn new(bits: u8) -> Self {
        SD_DBG_ENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD_DBG_ENA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sd_dbg_ena` writer - "]
pub struct SD_DBG_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `sd_dbg_mode` reader - "]
pub struct SD_DBG_MODE_R(crate::FieldReader<u8, u8>);
impl SD_DBG_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SD_DBG_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD_DBG_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sd_dbg_mode` writer - "]
pub struct SD_DBG_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `sd_dbg_pwd_cnt` reader - "]
pub struct SD_DBG_PWD_CNT_R(crate::FieldReader<u32, u32>);
impl SD_DBG_PWD_CNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        SD_DBG_PWD_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD_DBG_PWD_CNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sd_dbg_pwd_cnt` writer - "]
pub struct SD_DBG_PWD_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_PWD_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x000f_ffff << 4)) | (((value as u32) & 0x000f_ffff) << 4);
        self.w
    }
}
#[doc = "Field `sd_dbg_cci_clk_sel` reader - "]
pub struct SD_DBG_CCI_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl SD_DBG_CCI_CLK_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SD_DBG_CCI_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD_DBG_CCI_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sd_dbg_cci_clk_sel` writer - "]
pub struct SD_DBG_CCI_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_CCI_CLK_SEL_W<'a> {
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
#[doc = "Field `sd_dbg_cci_read_en` reader - "]
pub struct SD_DBG_CCI_READ_EN_R(crate::FieldReader<bool, bool>);
impl SD_DBG_CCI_READ_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SD_DBG_CCI_READ_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD_DBG_CCI_READ_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sd_dbg_cci_read_en` writer - "]
pub struct SD_DBG_CCI_READ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_CCI_READ_EN_W<'a> {
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
#[doc = "Field `sd_dbg_pwd_trig` reader - "]
pub struct SD_DBG_PWD_TRIG_R(crate::FieldReader<bool, bool>);
impl SD_DBG_PWD_TRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SD_DBG_PWD_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD_DBG_PWD_TRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sd_dbg_pwd_trig` writer - "]
pub struct SD_DBG_PWD_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_PWD_TRIG_W<'a> {
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
#[doc = "Field `sd_dbg_pwd_busy` reader - "]
pub struct SD_DBG_PWD_BUSY_R(crate::FieldReader<bool, bool>);
impl SD_DBG_PWD_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SD_DBG_PWD_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SD_DBG_PWD_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sd_dbg_pwd_busy` writer - "]
pub struct SD_DBG_PWD_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SD_DBG_PWD_BUSY_W<'a> {
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
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sd_dbg_ena(&self) -> SD_DBG_ENA_R {
        SD_DBG_ENA_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sd_dbg_mode(&self) -> SD_DBG_MODE_R {
        SD_DBG_MODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 4:23"]
    #[inline(always)]
    pub fn sd_dbg_pwd_cnt(&self) -> SD_DBG_PWD_CNT_R {
        SD_DBG_PWD_CNT_R::new(((self.bits >> 4) & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sd_dbg_cci_clk_sel(&self) -> SD_DBG_CCI_CLK_SEL_R {
        SD_DBG_CCI_CLK_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sd_dbg_cci_read_en(&self) -> SD_DBG_CCI_READ_EN_R {
        SD_DBG_CCI_READ_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sd_dbg_pwd_trig(&self) -> SD_DBG_PWD_TRIG_R {
        SD_DBG_PWD_TRIG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sd_dbg_pwd_busy(&self) -> SD_DBG_PWD_BUSY_R {
        SD_DBG_PWD_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sd_dbg_ena(&mut self) -> SD_DBG_ENA_W {
        SD_DBG_ENA_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn sd_dbg_mode(&mut self) -> SD_DBG_MODE_W {
        SD_DBG_MODE_W { w: self }
    }
    #[doc = "Bits 4:23"]
    #[inline(always)]
    pub fn sd_dbg_pwd_cnt(&mut self) -> SD_DBG_PWD_CNT_W {
        SD_DBG_PWD_CNT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sd_dbg_cci_clk_sel(&mut self) -> SD_DBG_CCI_CLK_SEL_W {
        SD_DBG_CCI_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sd_dbg_cci_read_en(&mut self) -> SD_DBG_CCI_READ_EN_W {
        SD_DBG_CCI_READ_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sd_dbg_pwd_trig(&mut self) -> SD_DBG_PWD_TRIG_W {
        SD_DBG_PWD_TRIG_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sd_dbg_pwd_busy(&mut self) -> SD_DBG_PWD_BUSY_W {
        SD_DBG_PWD_BUSY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sd_status.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sd_status](index.html) module"]
pub struct SD_STATUS_SPEC;
impl crate::RegisterSpec for SD_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sd_status::R](R) reader structure"]
impl crate::Readable for SD_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sd_status::W](W) writer structure"]
impl crate::Writable for SD_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sd_status to value 0"]
impl crate::Resettable for SD_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
