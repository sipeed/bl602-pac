#[doc = "Register `PDS_CTL2` reader"]
pub struct R(crate::R<PDS_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDS_CTL2_SPEC>> for R {
    fn from(reader: crate::R<PDS_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDS_CTL2` writer"]
pub struct W(crate::W<PDS_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_CTL2_SPEC>;
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
impl core::convert::From<crate::W<PDS_CTL2_SPEC>> for W {
    fn from(writer: crate::W<PDS_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_pds_force_wb_gate_clk` reader - "]
pub struct CR_PDS_FORCE_WB_GATE_CLK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_FORCE_WB_GATE_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_FORCE_WB_GATE_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_FORCE_WB_GATE_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_force_wb_gate_clk` writer - "]
pub struct CR_PDS_FORCE_WB_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_WB_GATE_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `cr_pds_force_np_gate_clk` reader - "]
pub struct CR_PDS_FORCE_NP_GATE_CLK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_FORCE_NP_GATE_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_FORCE_NP_GATE_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_FORCE_NP_GATE_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_force_np_gate_clk` writer - "]
pub struct CR_PDS_FORCE_NP_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_NP_GATE_CLK_W<'a> {
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
#[doc = "Field `cr_pds_force_wb_mem_stby` reader - "]
pub struct CR_PDS_FORCE_WB_MEM_STBY_R(crate::FieldReader<bool, bool>);
impl CR_PDS_FORCE_WB_MEM_STBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_FORCE_WB_MEM_STBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_FORCE_WB_MEM_STBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_force_wb_mem_stby` writer - "]
pub struct CR_PDS_FORCE_WB_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_WB_MEM_STBY_W<'a> {
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
#[doc = "Field `cr_pds_force_np_mem_stby` reader - "]
pub struct CR_PDS_FORCE_NP_MEM_STBY_R(crate::FieldReader<bool, bool>);
impl CR_PDS_FORCE_NP_MEM_STBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_FORCE_NP_MEM_STBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_FORCE_NP_MEM_STBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_force_np_mem_stby` writer - "]
pub struct CR_PDS_FORCE_NP_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_NP_MEM_STBY_W<'a> {
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
#[doc = "Field `cr_pds_force_wb_pds_rst` reader - "]
pub struct CR_PDS_FORCE_WB_PDS_RST_R(crate::FieldReader<bool, bool>);
impl CR_PDS_FORCE_WB_PDS_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_FORCE_WB_PDS_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_FORCE_WB_PDS_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_force_wb_pds_rst` writer - "]
pub struct CR_PDS_FORCE_WB_PDS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_WB_PDS_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `cr_pds_force_np_pds_rst` reader - "]
pub struct CR_PDS_FORCE_NP_PDS_RST_R(crate::FieldReader<bool, bool>);
impl CR_PDS_FORCE_NP_PDS_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_FORCE_NP_PDS_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_FORCE_NP_PDS_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_force_np_pds_rst` writer - "]
pub struct CR_PDS_FORCE_NP_PDS_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_NP_PDS_RST_W<'a> {
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
#[doc = "Field `cr_pds_force_wb_iso_en` reader - "]
pub struct CR_PDS_FORCE_WB_ISO_EN_R(crate::FieldReader<bool, bool>);
impl CR_PDS_FORCE_WB_ISO_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_FORCE_WB_ISO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_FORCE_WB_ISO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_force_wb_iso_en` writer - "]
pub struct CR_PDS_FORCE_WB_ISO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_WB_ISO_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `cr_pds_force_np_iso_en` reader - "]
pub struct CR_PDS_FORCE_NP_ISO_EN_R(crate::FieldReader<bool, bool>);
impl CR_PDS_FORCE_NP_ISO_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_FORCE_NP_ISO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_FORCE_NP_ISO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_force_np_iso_en` writer - "]
pub struct CR_PDS_FORCE_NP_ISO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_NP_ISO_EN_W<'a> {
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
#[doc = "Field `cr_pds_force_wb_pwr_off` reader - "]
pub struct CR_PDS_FORCE_WB_PWR_OFF_R(crate::FieldReader<bool, bool>);
impl CR_PDS_FORCE_WB_PWR_OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_FORCE_WB_PWR_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_FORCE_WB_PWR_OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_force_wb_pwr_off` writer - "]
pub struct CR_PDS_FORCE_WB_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_WB_PWR_OFF_W<'a> {
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
#[doc = "Field `cr_pds_force_np_pwr_off` reader - "]
pub struct CR_PDS_FORCE_NP_PWR_OFF_R(crate::FieldReader<bool, bool>);
impl CR_PDS_FORCE_NP_PWR_OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_FORCE_NP_PWR_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_FORCE_NP_PWR_OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_force_np_pwr_off` writer - "]
pub struct CR_PDS_FORCE_NP_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_FORCE_NP_PWR_OFF_W<'a> {
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
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_force_wb_gate_clk(&self) -> CR_PDS_FORCE_WB_GATE_CLK_R {
        CR_PDS_FORCE_WB_GATE_CLK_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_force_np_gate_clk(&self) -> CR_PDS_FORCE_NP_GATE_CLK_R {
        CR_PDS_FORCE_NP_GATE_CLK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_force_wb_mem_stby(&self) -> CR_PDS_FORCE_WB_MEM_STBY_R {
        CR_PDS_FORCE_WB_MEM_STBY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_force_np_mem_stby(&self) -> CR_PDS_FORCE_NP_MEM_STBY_R {
        CR_PDS_FORCE_NP_MEM_STBY_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pds_rst(&self) -> CR_PDS_FORCE_WB_PDS_RST_R {
        CR_PDS_FORCE_WB_PDS_RST_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_force_np_pds_rst(&self) -> CR_PDS_FORCE_NP_PDS_RST_R {
        CR_PDS_FORCE_NP_PDS_RST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_force_wb_iso_en(&self) -> CR_PDS_FORCE_WB_ISO_EN_R {
        CR_PDS_FORCE_WB_ISO_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_np_iso_en(&self) -> CR_PDS_FORCE_NP_ISO_EN_R {
        CR_PDS_FORCE_NP_ISO_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pwr_off(&self) -> CR_PDS_FORCE_WB_PWR_OFF_R {
        CR_PDS_FORCE_WB_PWR_OFF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_force_np_pwr_off(&self) -> CR_PDS_FORCE_NP_PWR_OFF_R {
        CR_PDS_FORCE_NP_PWR_OFF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_force_wb_gate_clk(&mut self) -> CR_PDS_FORCE_WB_GATE_CLK_W {
        CR_PDS_FORCE_WB_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_force_np_gate_clk(&mut self) -> CR_PDS_FORCE_NP_GATE_CLK_W {
        CR_PDS_FORCE_NP_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_force_wb_mem_stby(&mut self) -> CR_PDS_FORCE_WB_MEM_STBY_W {
        CR_PDS_FORCE_WB_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_force_np_mem_stby(&mut self) -> CR_PDS_FORCE_NP_MEM_STBY_W {
        CR_PDS_FORCE_NP_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pds_rst(&mut self) -> CR_PDS_FORCE_WB_PDS_RST_W {
        CR_PDS_FORCE_WB_PDS_RST_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_force_np_pds_rst(&mut self) -> CR_PDS_FORCE_NP_PDS_RST_W {
        CR_PDS_FORCE_NP_PDS_RST_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_pds_force_wb_iso_en(&mut self) -> CR_PDS_FORCE_WB_ISO_EN_W {
        CR_PDS_FORCE_WB_ISO_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_pds_force_np_iso_en(&mut self) -> CR_PDS_FORCE_NP_ISO_EN_W {
        CR_PDS_FORCE_NP_ISO_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pwr_off(&mut self) -> CR_PDS_FORCE_WB_PWR_OFF_W {
        CR_PDS_FORCE_WB_PWR_OFF_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_force_np_pwr_off(&mut self) -> CR_PDS_FORCE_NP_PWR_OFF_W {
        CR_PDS_FORCE_NP_PWR_OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_CTL2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ctl2](index.html) module"]
pub struct PDS_CTL2_SPEC;
impl crate::RegisterSpec for PDS_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ctl2::R](R) reader structure"]
impl crate::Readable for PDS_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ctl2::W](W) writer structure"]
impl crate::Writable for PDS_CTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDS_CTL2 to value 0"]
impl crate::Resettable for PDS_CTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
