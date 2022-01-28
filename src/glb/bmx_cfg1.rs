#[doc = "Register `bmx_cfg1` reader"]
pub struct R(crate::R<BMX_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_cfg1` writer"]
pub struct W(crate::W<BMX_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_CFG1_SPEC>;
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
impl From<crate::W<BMX_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMX_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `hbn_apb_cfg` reader - "]
pub struct HBN_APB_CFG_R(crate::FieldReader<u8, u8>);
impl HBN_APB_CFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        HBN_APB_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HBN_APB_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hbn_apb_cfg` writer - "]
pub struct HBN_APB_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> HBN_APB_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `pds_apb_cfg` reader - "]
pub struct PDS_APB_CFG_R(crate::FieldReader<u8, u8>);
impl PDS_APB_CFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        PDS_APB_CFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDS_APB_CFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pds_apb_cfg` writer - "]
pub struct PDS_APB_CFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PDS_APB_CFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `hsel_option` reader - "]
pub struct HSEL_OPTION_R(crate::FieldReader<u8, u8>);
impl HSEL_OPTION_R {
    pub(crate) fn new(bits: u8) -> Self {
        HSEL_OPTION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSEL_OPTION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hsel_option` writer - "]
pub struct HSEL_OPTION_W<'a> {
    w: &'a mut W,
}
impl<'a> HSEL_OPTION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `bmx_gating_dis` reader - "]
pub struct BMX_GATING_DIS_R(crate::FieldReader<bool, bool>);
impl BMX_GATING_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMX_GATING_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMX_GATING_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bmx_gating_dis` writer - "]
pub struct BMX_GATING_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_GATING_DIS_W<'a> {
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
#[doc = "Field `bmx_busy_option_dis` reader - "]
pub struct BMX_BUSY_OPTION_DIS_R(crate::FieldReader<bool, bool>);
impl BMX_BUSY_OPTION_DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMX_BUSY_OPTION_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMX_BUSY_OPTION_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bmx_busy_option_dis` writer - "]
pub struct BMX_BUSY_OPTION_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_BUSY_OPTION_DIS_W<'a> {
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
#[doc = "Field `bmx_err_en` reader - "]
pub struct BMX_ERR_EN_R(crate::FieldReader<bool, bool>);
impl BMX_ERR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMX_ERR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMX_ERR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bmx_err_en` writer - "]
pub struct BMX_ERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_ERR_EN_W<'a> {
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
#[doc = "Field `bmx_arb_mode` reader - "]
pub struct BMX_ARB_MODE_R(crate::FieldReader<u8, u8>);
impl BMX_ARB_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BMX_ARB_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMX_ARB_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bmx_arb_mode` writer - "]
pub struct BMX_ARB_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_ARB_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `bmx_timeout_en` reader - "]
pub struct BMX_TIMEOUT_EN_R(crate::FieldReader<u8, u8>);
impl BMX_TIMEOUT_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        BMX_TIMEOUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMX_TIMEOUT_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bmx_timeout_en` writer - "]
pub struct BMX_TIMEOUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BMX_TIMEOUT_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hbn_apb_cfg(&self) -> HBN_APB_CFG_R {
        HBN_APB_CFG_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pds_apb_cfg(&self) -> PDS_APB_CFG_R {
        PDS_APB_CFG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hsel_option(&self) -> HSEL_OPTION_R {
        HSEL_OPTION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bmx_gating_dis(&self) -> BMX_GATING_DIS_R {
        BMX_GATING_DIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bmx_busy_option_dis(&self) -> BMX_BUSY_OPTION_DIS_R {
        BMX_BUSY_OPTION_DIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bmx_err_en(&self) -> BMX_ERR_EN_R {
        BMX_ERR_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn bmx_arb_mode(&self) -> BMX_ARB_MODE_R {
        BMX_ARB_MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn bmx_timeout_en(&self) -> BMX_TIMEOUT_EN_R {
        BMX_TIMEOUT_EN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn hbn_apb_cfg(&mut self) -> HBN_APB_CFG_W {
        HBN_APB_CFG_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn pds_apb_cfg(&mut self) -> PDS_APB_CFG_W {
        PDS_APB_CFG_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hsel_option(&mut self) -> HSEL_OPTION_W {
        HSEL_OPTION_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn bmx_gating_dis(&mut self) -> BMX_GATING_DIS_W {
        BMX_GATING_DIS_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn bmx_busy_option_dis(&mut self) -> BMX_BUSY_OPTION_DIS_W {
        BMX_BUSY_OPTION_DIS_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn bmx_err_en(&mut self) -> BMX_ERR_EN_W {
        BMX_ERR_EN_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn bmx_arb_mode(&mut self) -> BMX_ARB_MODE_W {
        BMX_ARB_MODE_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn bmx_timeout_en(&mut self) -> BMX_TIMEOUT_EN_W {
        BMX_TIMEOUT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bmx_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg1](index.html) module"]
pub struct BMX_CFG1_SPEC;
impl crate::RegisterSpec for BMX_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_cfg1::R](R) reader structure"]
impl crate::Readable for BMX_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_cfg1::W](W) writer structure"]
impl crate::Writable for BMX_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets bmx_cfg1 to value 0"]
impl crate::Resettable for BMX_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
