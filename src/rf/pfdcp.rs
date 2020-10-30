#[doc = "Register `pfdcp` reader"]
pub struct R(crate::R<PFDCP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFDCP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PFDCP_SPEC>> for R {
    fn from(reader: crate::R<PFDCP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pfdcp` writer"]
pub struct W(crate::W<PFDCP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFDCP_SPEC>;
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
impl core::convert::From<crate::W<PFDCP_SPEC>> for W {
    fn from(writer: crate::W<PFDCP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_pfd_rst_csd_hw` reader - "]
pub struct LO_PFD_RST_CSD_HW_R(crate::FieldReader<bool, bool>);
impl LO_PFD_RST_CSD_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_PFD_RST_CSD_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_PFD_RST_CSD_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_pfd_rst_csd_hw` writer - "]
pub struct LO_PFD_RST_CSD_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_PFD_RST_CSD_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `lo_pfd_rst_csd` reader - "]
pub struct LO_PFD_RST_CSD_R(crate::FieldReader<bool, bool>);
impl LO_PFD_RST_CSD_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_PFD_RST_CSD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_PFD_RST_CSD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_pfd_rst_csd` writer - "]
pub struct LO_PFD_RST_CSD_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_PFD_RST_CSD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `lo_pfd_rvdd_boost` reader - "]
pub struct LO_PFD_RVDD_BOOST_R(crate::FieldReader<bool, bool>);
impl LO_PFD_RVDD_BOOST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_PFD_RVDD_BOOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_PFD_RVDD_BOOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_pfd_rvdd_boost` writer - "]
pub struct LO_PFD_RVDD_BOOST_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_PFD_RVDD_BOOST_W<'a> {
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
#[doc = "Field `lo_cp_hiz` reader - "]
pub struct LO_CP_HIZ_R(crate::FieldReader<bool, bool>);
impl LO_CP_HIZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_CP_HIZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_CP_HIZ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_cp_hiz` writer - "]
pub struct LO_CP_HIZ_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_HIZ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `lo_cp_opamp_en` reader - "]
pub struct LO_CP_OPAMP_EN_R(crate::FieldReader<bool, bool>);
impl LO_CP_OPAMP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_CP_OPAMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_CP_OPAMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_cp_opamp_en` writer - "]
pub struct LO_CP_OPAMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_OPAMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `lo_cp_ota_en` reader - "]
pub struct LO_CP_OTA_EN_R(crate::FieldReader<bool, bool>);
impl LO_CP_OTA_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_CP_OTA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_CP_OTA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_cp_ota_en` writer - "]
pub struct LO_CP_OTA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_OTA_EN_W<'a> {
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
#[doc = "Field `lo_cp_startup_en` reader - "]
pub struct LO_CP_STARTUP_EN_R(crate::FieldReader<bool, bool>);
impl LO_CP_STARTUP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_CP_STARTUP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_CP_STARTUP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_cp_startup_en` writer - "]
pub struct LO_CP_STARTUP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_STARTUP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `lo_cp_sel_hw` reader - "]
pub struct LO_CP_SEL_HW_R(crate::FieldReader<bool, bool>);
impl LO_CP_SEL_HW_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_CP_SEL_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_CP_SEL_HW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_cp_sel_hw` writer - "]
pub struct LO_CP_SEL_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_SEL_HW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `lo_cp_sel` reader - "]
pub struct LO_CP_SEL_R(crate::FieldReader<bool, bool>);
impl LO_CP_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_CP_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_CP_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_cp_sel` writer - "]
pub struct LO_CP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_SEL_W<'a> {
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
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd_hw(&self) -> LO_PFD_RST_CSD_HW_R {
        LO_PFD_RST_CSD_HW_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd(&self) -> LO_PFD_RST_CSD_R {
        LO_PFD_RST_CSD_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_pfd_rvdd_boost(&self) -> LO_PFD_RVDD_BOOST_R {
        LO_PFD_RVDD_BOOST_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_cp_hiz(&self) -> LO_CP_HIZ_R {
        LO_CP_HIZ_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_cp_opamp_en(&self) -> LO_CP_OPAMP_EN_R {
        LO_CP_OPAMP_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_cp_ota_en(&self) -> LO_CP_OTA_EN_R {
        LO_CP_OTA_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_cp_startup_en(&self) -> LO_CP_STARTUP_EN_R {
        LO_CP_STARTUP_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_cp_sel_hw(&self) -> LO_CP_SEL_HW_R {
        LO_CP_SEL_HW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_cp_sel(&self) -> LO_CP_SEL_R {
        LO_CP_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd_hw(&mut self) -> LO_PFD_RST_CSD_HW_W {
        LO_PFD_RST_CSD_HW_W { w: self }
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn lo_pfd_rst_csd(&mut self) -> LO_PFD_RST_CSD_W {
        LO_PFD_RST_CSD_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_pfd_rvdd_boost(&mut self) -> LO_PFD_RVDD_BOOST_W {
        LO_PFD_RVDD_BOOST_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_cp_hiz(&mut self) -> LO_CP_HIZ_W {
        LO_CP_HIZ_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn lo_cp_opamp_en(&mut self) -> LO_CP_OPAMP_EN_W {
        LO_CP_OPAMP_EN_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn lo_cp_ota_en(&mut self) -> LO_CP_OTA_EN_W {
        LO_CP_OTA_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn lo_cp_startup_en(&mut self) -> LO_CP_STARTUP_EN_W {
        LO_CP_STARTUP_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn lo_cp_sel_hw(&mut self) -> LO_CP_SEL_HW_W {
        LO_CP_SEL_HW_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_cp_sel(&mut self) -> LO_CP_SEL_W {
        LO_CP_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pfdcp.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfdcp](index.html) module"]
pub struct PFDCP_SPEC;
impl crate::RegisterSpec for PFDCP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfdcp::R](R) reader structure"]
impl crate::Readable for PFDCP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfdcp::W](W) writer structure"]
impl crate::Writable for PFDCP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pfdcp to value 0"]
impl crate::Resettable for PFDCP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
