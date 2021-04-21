#[doc = "Register `ef_if_ctrl_0` reader"]
pub struct R(crate::R<EF_IF_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_IF_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<EF_IF_CTRL_0_SPEC>> for R {
    fn from(reader: crate::R<EF_IF_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_if_ctrl_0` writer"]
pub struct W(crate::W<EF_IF_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_IF_CTRL_0_SPEC>;
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
impl core::convert::From<crate::W<EF_IF_CTRL_0_SPEC>> for W {
    fn from(writer: crate::W<EF_IF_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_if_prot_code_cyc` reader - "]
pub struct EF_IF_PROT_CODE_CYC_R(crate::FieldReader<u8, u8>);
impl EF_IF_PROT_CODE_CYC_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_PROT_CODE_CYC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_PROT_CODE_CYC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_prot_code_cyc` writer - "]
pub struct EF_IF_PROT_CODE_CYC_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PROT_CODE_CYC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `ef_if_0_int_set` reader - "]
pub struct EF_IF_0_INT_SET_R(crate::FieldReader<bool, bool>);
impl EF_IF_0_INT_SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_0_INT_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_INT_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_0_int_set` writer - "]
pub struct EF_IF_0_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_INT_SET_W<'a> {
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
#[doc = "Field `ef_if_0_int_clr` reader - "]
pub struct EF_IF_0_INT_CLR_R(crate::FieldReader<bool, bool>);
impl EF_IF_0_INT_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_0_INT_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_INT_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_0_int_clr` writer - "]
pub struct EF_IF_0_INT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_INT_CLR_W<'a> {
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
#[doc = "Field `ef_if_0_int` reader - "]
pub struct EF_IF_0_INT_R(crate::FieldReader<bool, bool>);
impl EF_IF_0_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_0_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_modify_lock` reader - "]
pub struct EF_IF_CYC_MODIFY_LOCK_R(crate::FieldReader<bool, bool>);
impl EF_IF_CYC_MODIFY_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_CYC_MODIFY_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_CYC_MODIFY_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_cyc_modify_lock` writer - "]
pub struct EF_IF_CYC_MODIFY_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_CYC_MODIFY_LOCK_W<'a> {
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
#[doc = "Field `ef_if_auto_rd_en` reader - "]
pub struct EF_IF_AUTO_RD_EN_R(crate::FieldReader<bool, bool>);
impl EF_IF_AUTO_RD_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_AUTO_RD_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_AUTO_RD_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_auto_rd_en` writer - "]
pub struct EF_IF_AUTO_RD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_AUTO_RD_EN_W<'a> {
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
#[doc = "Field `ef_clk_sahb_data_gate` reader - "]
pub struct EF_CLK_SAHB_DATA_GATE_R(crate::FieldReader<bool, bool>);
impl EF_CLK_SAHB_DATA_GATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CLK_SAHB_DATA_GATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CLK_SAHB_DATA_GATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_clk_sahb_data_gate` writer - "]
pub struct EF_CLK_SAHB_DATA_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CLK_SAHB_DATA_GATE_W<'a> {
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
#[doc = "Field `ef_if_por_dig` reader - "]
pub struct EF_IF_POR_DIG_R(crate::FieldReader<bool, bool>);
impl EF_IF_POR_DIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_POR_DIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_POR_DIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_por_dig` writer - "]
pub struct EF_IF_POR_DIG_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_POR_DIG_W<'a> {
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
#[doc = "Field `ef_if_prot_code_ctrl` reader - "]
pub struct EF_IF_PROT_CODE_CTRL_R(crate::FieldReader<u8, u8>);
impl EF_IF_PROT_CODE_CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EF_IF_PROT_CODE_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_PROT_CODE_CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_prot_code_ctrl` writer - "]
pub struct EF_IF_PROT_CODE_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_PROT_CODE_CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `ef_clk_sahb_data_sel` reader - "]
pub struct EF_CLK_SAHB_DATA_SEL_R(crate::FieldReader<bool, bool>);
impl EF_CLK_SAHB_DATA_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_CLK_SAHB_DATA_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_CLK_SAHB_DATA_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_clk_sahb_data_sel` writer - "]
pub struct EF_CLK_SAHB_DATA_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_CLK_SAHB_DATA_SEL_W<'a> {
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
#[doc = "Field `ef_if_0_cyc_modify` reader - "]
pub struct EF_IF_0_CYC_MODIFY_R(crate::FieldReader<bool, bool>);
impl EF_IF_0_CYC_MODIFY_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_0_CYC_MODIFY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_CYC_MODIFY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_0_cyc_modify` writer - "]
pub struct EF_IF_0_CYC_MODIFY_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_CYC_MODIFY_W<'a> {
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
#[doc = "Field `ef_if_0_manual_en` reader - "]
pub struct EF_IF_0_MANUAL_EN_R(crate::FieldReader<bool, bool>);
impl EF_IF_0_MANUAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_0_MANUAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_MANUAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_0_manual_en` writer - "]
pub struct EF_IF_0_MANUAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_MANUAL_EN_W<'a> {
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
#[doc = "Field `ef_if_0_trig` reader - "]
pub struct EF_IF_0_TRIG_R(crate::FieldReader<bool, bool>);
impl EF_IF_0_TRIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_0_TRIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_TRIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_0_trig` writer - "]
pub struct EF_IF_0_TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_TRIG_W<'a> {
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
#[doc = "Field `ef_if_0_rw` reader - "]
pub struct EF_IF_0_RW_R(crate::FieldReader<bool, bool>);
impl EF_IF_0_RW_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_0_RW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_RW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_0_rw` writer - "]
pub struct EF_IF_0_RW_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_IF_0_RW_W<'a> {
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
#[doc = "Field `ef_if_0_busy` reader - "]
pub struct EF_IF_0_BUSY_R(crate::FieldReader<bool, bool>);
impl EF_IF_0_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_0_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_0_autoload_done` reader - "]
pub struct EF_IF_0_AUTOLOAD_DONE_R(crate::FieldReader<bool, bool>);
impl EF_IF_0_AUTOLOAD_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_0_AUTOLOAD_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_AUTOLOAD_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_if_0_autoload_p1_done` reader - "]
pub struct EF_IF_0_AUTOLOAD_P1_DONE_R(crate::FieldReader<bool, bool>);
impl EF_IF_0_AUTOLOAD_P1_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        EF_IF_0_AUTOLOAD_P1_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_IF_0_AUTOLOAD_P1_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_cyc(&self) -> EF_IF_PROT_CODE_CYC_R {
        EF_IF_PROT_CODE_CYC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_0_int_set(&self) -> EF_IF_0_INT_SET_R {
        EF_IF_0_INT_SET_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_0_int_clr(&self) -> EF_IF_0_INT_CLR_R {
        EF_IF_0_INT_CLR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ef_if_0_int(&self) -> EF_IF_0_INT_R {
        EF_IF_0_INT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_cyc_modify_lock(&self) -> EF_IF_CYC_MODIFY_LOCK_R {
        EF_IF_CYC_MODIFY_LOCK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_auto_rd_en(&self) -> EF_IF_AUTO_RD_EN_R {
        EF_IF_AUTO_RD_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_gate(&self) -> EF_CLK_SAHB_DATA_GATE_R {
        EF_CLK_SAHB_DATA_GATE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_por_dig(&self) -> EF_IF_POR_DIG_R {
        EF_IF_POR_DIG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ef_if_prot_code_ctrl(&self) -> EF_IF_PROT_CODE_CTRL_R {
        EF_IF_PROT_CODE_CTRL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_sel(&self) -> EF_CLK_SAHB_DATA_SEL_R {
        EF_CLK_SAHB_DATA_SEL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_0_cyc_modify(&self) -> EF_IF_0_CYC_MODIFY_R {
        EF_IF_0_CYC_MODIFY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_if_0_manual_en(&self) -> EF_IF_0_MANUAL_EN_R {
        EF_IF_0_MANUAL_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_if_0_trig(&self) -> EF_IF_0_TRIG_R {
        EF_IF_0_TRIG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_if_0_rw(&self) -> EF_IF_0_RW_R {
        EF_IF_0_RW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ef_if_0_busy(&self) -> EF_IF_0_BUSY_R {
        EF_IF_0_BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ef_if_0_autoload_done(&self) -> EF_IF_0_AUTOLOAD_DONE_R {
        EF_IF_0_AUTOLOAD_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ef_if_0_autoload_p1_done(&self) -> EF_IF_0_AUTOLOAD_P1_DONE_R {
        EF_IF_0_AUTOLOAD_P1_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn ef_if_prot_code_cyc(&mut self) -> EF_IF_PROT_CODE_CYC_W {
        EF_IF_PROT_CODE_CYC_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn ef_if_0_int_set(&mut self) -> EF_IF_0_INT_SET_W {
        EF_IF_0_INT_SET_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ef_if_0_int_clr(&mut self) -> EF_IF_0_INT_CLR_W {
        EF_IF_0_INT_CLR_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ef_if_cyc_modify_lock(&mut self) -> EF_IF_CYC_MODIFY_LOCK_W {
        EF_IF_CYC_MODIFY_LOCK_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ef_if_auto_rd_en(&mut self) -> EF_IF_AUTO_RD_EN_W {
        EF_IF_AUTO_RD_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_gate(&mut self) -> EF_CLK_SAHB_DATA_GATE_W {
        EF_CLK_SAHB_DATA_GATE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ef_if_por_dig(&mut self) -> EF_IF_POR_DIG_W {
        EF_IF_POR_DIG_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn ef_if_prot_code_ctrl(&mut self) -> EF_IF_PROT_CODE_CTRL_W {
        EF_IF_PROT_CODE_CTRL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ef_clk_sahb_data_sel(&mut self) -> EF_CLK_SAHB_DATA_SEL_W {
        EF_CLK_SAHB_DATA_SEL_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ef_if_0_cyc_modify(&mut self) -> EF_IF_0_CYC_MODIFY_W {
        EF_IF_0_CYC_MODIFY_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ef_if_0_manual_en(&mut self) -> EF_IF_0_MANUAL_EN_W {
        EF_IF_0_MANUAL_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ef_if_0_trig(&mut self) -> EF_IF_0_TRIG_W {
        EF_IF_0_TRIG_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ef_if_0_rw(&mut self) -> EF_IF_0_RW_W {
        EF_IF_0_RW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_if_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_if_ctrl_0](index.html) module"]
pub struct EF_IF_CTRL_0_SPEC;
impl crate::RegisterSpec for EF_IF_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_if_ctrl_0::R](R) reader structure"]
impl crate::Readable for EF_IF_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_if_ctrl_0::W](W) writer structure"]
impl crate::Writable for EF_IF_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_if_ctrl_0 to value 0x0024_0003"]
impl crate::Resettable for EF_IF_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0024_0003
    }
}
