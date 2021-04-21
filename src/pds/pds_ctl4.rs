#[doc = "Register `PDS_CTL4` reader"]
pub struct R(crate::R<PDS_CTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_CTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PDS_CTL4_SPEC>> for R {
    fn from(reader: crate::R<PDS_CTL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDS_CTL4` writer"]
pub struct W(crate::W<PDS_CTL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_CTL4_SPEC>;
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
impl core::convert::From<crate::W<PDS_CTL4_SPEC>> for W {
    fn from(writer: crate::W<PDS_CTL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_pds_misc_gate_clk` reader - "]
pub struct CR_PDS_MISC_GATE_CLK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_MISC_GATE_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_MISC_GATE_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_MISC_GATE_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_misc_gate_clk` writer - "]
pub struct CR_PDS_MISC_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_MISC_GATE_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `cr_pds_misc_mem_stby` reader - "]
pub struct CR_PDS_MISC_MEM_STBY_R(crate::FieldReader<bool, bool>);
impl CR_PDS_MISC_MEM_STBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_MISC_MEM_STBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_MISC_MEM_STBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_misc_mem_stby` writer - "]
pub struct CR_PDS_MISC_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_MISC_MEM_STBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `cr_pds_misc_reset` reader - "]
pub struct CR_PDS_MISC_RESET_R(crate::FieldReader<bool, bool>);
impl CR_PDS_MISC_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_MISC_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_MISC_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_misc_reset` writer - "]
pub struct CR_PDS_MISC_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_MISC_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `cr_pds_misc_pwr_off` reader - "]
pub struct CR_PDS_MISC_PWR_OFF_R(crate::FieldReader<bool, bool>);
impl CR_PDS_MISC_PWR_OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_MISC_PWR_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_MISC_PWR_OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_misc_pwr_off` writer - "]
pub struct CR_PDS_MISC_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_MISC_PWR_OFF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `cr_pds_wb_gate_clk` reader - "]
pub struct CR_PDS_WB_GATE_CLK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_WB_GATE_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_WB_GATE_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_WB_GATE_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_wb_gate_clk` writer - "]
pub struct CR_PDS_WB_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WB_GATE_CLK_W<'a> {
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
#[doc = "Field `cr_pds_wb_mem_stby` reader - "]
pub struct CR_PDS_WB_MEM_STBY_R(crate::FieldReader<bool, bool>);
impl CR_PDS_WB_MEM_STBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_WB_MEM_STBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_WB_MEM_STBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_wb_mem_stby` writer - "]
pub struct CR_PDS_WB_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WB_MEM_STBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `cr_pds_wb_reset` reader - "]
pub struct CR_PDS_WB_RESET_R(crate::FieldReader<bool, bool>);
impl CR_PDS_WB_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_WB_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_WB_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_wb_reset` writer - "]
pub struct CR_PDS_WB_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WB_RESET_W<'a> {
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
#[doc = "Field `cr_pds_wb_pwr_off` reader - "]
pub struct CR_PDS_WB_PWR_OFF_R(crate::FieldReader<bool, bool>);
impl CR_PDS_WB_PWR_OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_WB_PWR_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_WB_PWR_OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_wb_pwr_off` writer - "]
pub struct CR_PDS_WB_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_WB_PWR_OFF_W<'a> {
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
#[doc = "Field `cr_pds_np_gate_clk` reader - "]
pub struct CR_PDS_NP_GATE_CLK_R(crate::FieldReader<bool, bool>);
impl CR_PDS_NP_GATE_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_NP_GATE_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_NP_GATE_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_np_gate_clk` writer - "]
pub struct CR_PDS_NP_GATE_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_NP_GATE_CLK_W<'a> {
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
#[doc = "Field `cr_pds_np_mem_stby` reader - "]
pub struct CR_PDS_NP_MEM_STBY_R(crate::FieldReader<bool, bool>);
impl CR_PDS_NP_MEM_STBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_NP_MEM_STBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_NP_MEM_STBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_np_mem_stby` writer - "]
pub struct CR_PDS_NP_MEM_STBY_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_NP_MEM_STBY_W<'a> {
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
#[doc = "Field `cr_pds_np_reset` reader - "]
pub struct CR_PDS_NP_RESET_R(crate::FieldReader<bool, bool>);
impl CR_PDS_NP_RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_NP_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_NP_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_np_reset` writer - "]
pub struct CR_PDS_NP_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_NP_RESET_W<'a> {
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
#[doc = "Field `cr_pds_np_pwr_off` reader - "]
pub struct CR_PDS_NP_PWR_OFF_R(crate::FieldReader<bool, bool>);
impl CR_PDS_NP_PWR_OFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_PDS_NP_PWR_OFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_PDS_NP_PWR_OFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_pds_np_pwr_off` writer - "]
pub struct CR_PDS_NP_PWR_OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_PDS_NP_PWR_OFF_W<'a> {
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
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_misc_gate_clk(&self) -> CR_PDS_MISC_GATE_CLK_R {
        CR_PDS_MISC_GATE_CLK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_misc_mem_stby(&self) -> CR_PDS_MISC_MEM_STBY_R {
        CR_PDS_MISC_MEM_STBY_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_misc_reset(&self) -> CR_PDS_MISC_RESET_R {
        CR_PDS_MISC_RESET_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_misc_pwr_off(&self) -> CR_PDS_MISC_PWR_OFF_R {
        CR_PDS_MISC_PWR_OFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_wb_gate_clk(&self) -> CR_PDS_WB_GATE_CLK_R {
        CR_PDS_WB_GATE_CLK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_wb_mem_stby(&self) -> CR_PDS_WB_MEM_STBY_R {
        CR_PDS_WB_MEM_STBY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_wb_reset(&self) -> CR_PDS_WB_RESET_R {
        CR_PDS_WB_RESET_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wb_pwr_off(&self) -> CR_PDS_WB_PWR_OFF_R {
        CR_PDS_WB_PWR_OFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_np_gate_clk(&self) -> CR_PDS_NP_GATE_CLK_R {
        CR_PDS_NP_GATE_CLK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_np_mem_stby(&self) -> CR_PDS_NP_MEM_STBY_R {
        CR_PDS_NP_MEM_STBY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_np_reset(&self) -> CR_PDS_NP_RESET_R {
        CR_PDS_NP_RESET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_np_pwr_off(&self) -> CR_PDS_NP_PWR_OFF_R {
        CR_PDS_NP_PWR_OFF_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn cr_pds_misc_gate_clk(&mut self) -> CR_PDS_MISC_GATE_CLK_W {
        CR_PDS_MISC_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn cr_pds_misc_mem_stby(&mut self) -> CR_PDS_MISC_MEM_STBY_W {
        CR_PDS_MISC_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn cr_pds_misc_reset(&mut self) -> CR_PDS_MISC_RESET_W {
        CR_PDS_MISC_RESET_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn cr_pds_misc_pwr_off(&mut self) -> CR_PDS_MISC_PWR_OFF_W {
        CR_PDS_MISC_PWR_OFF_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_wb_gate_clk(&mut self) -> CR_PDS_WB_GATE_CLK_W {
        CR_PDS_WB_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_wb_mem_stby(&mut self) -> CR_PDS_WB_MEM_STBY_W {
        CR_PDS_WB_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_wb_reset(&mut self) -> CR_PDS_WB_RESET_W {
        CR_PDS_WB_RESET_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_wb_pwr_off(&mut self) -> CR_PDS_WB_PWR_OFF_W {
        CR_PDS_WB_PWR_OFF_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_np_gate_clk(&mut self) -> CR_PDS_NP_GATE_CLK_W {
        CR_PDS_NP_GATE_CLK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_pds_np_mem_stby(&mut self) -> CR_PDS_NP_MEM_STBY_W {
        CR_PDS_NP_MEM_STBY_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_np_reset(&mut self) -> CR_PDS_NP_RESET_W {
        CR_PDS_NP_RESET_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_pds_np_pwr_off(&mut self) -> CR_PDS_NP_PWR_OFF_W {
        CR_PDS_NP_PWR_OFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_CTL4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ctl4](index.html) module"]
pub struct PDS_CTL4_SPEC;
impl crate::RegisterSpec for PDS_CTL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ctl4::R](R) reader structure"]
impl crate::Readable for PDS_CTL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ctl4::W](W) writer structure"]
impl crate::Writable for PDS_CTL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDS_CTL4 to value 0x0f00_f00f"]
impl crate::Resettable for PDS_CTL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f00_f00f
    }
}
