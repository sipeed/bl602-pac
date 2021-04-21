#[doc = "Register `lo_reg_ctrl_hw1` reader"]
pub struct R(crate::R<LO_REG_CTRL_HW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_REG_CTRL_HW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LO_REG_CTRL_HW1_SPEC>> for R {
    fn from(reader: crate::R<LO_REG_CTRL_HW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo_reg_ctrl_hw1` writer"]
pub struct W(crate::W<LO_REG_CTRL_HW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_REG_CTRL_HW1_SPEC>;
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
impl core::convert::From<crate::W<LO_REG_CTRL_HW1_SPEC>> for W {
    fn from(writer: crate::W<LO_REG_CTRL_HW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_lf_r4_tx` reader - "]
pub struct LO_LF_R4_TX_R(crate::FieldReader<u8, u8>);
impl LO_LF_R4_TX_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_R4_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_R4_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_r4_tx` writer - "]
pub struct LO_LF_R4_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_R4_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `lo_lf_r4_rx` reader - "]
pub struct LO_LF_R4_RX_R(crate::FieldReader<u8, u8>);
impl LO_LF_R4_RX_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_R4_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_R4_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_r4_rx` writer - "]
pub struct LO_LF_R4_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_R4_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `lo_lf_rz_tx` reader - "]
pub struct LO_LF_RZ_TX_R(crate::FieldReader<u8, u8>);
impl LO_LF_RZ_TX_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_RZ_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_RZ_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_rz_tx` writer - "]
pub struct LO_LF_RZ_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_RZ_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `lo_lf_rz_rx` reader - "]
pub struct LO_LF_RZ_RX_R(crate::FieldReader<u8, u8>);
impl LO_LF_RZ_RX_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_RZ_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_RZ_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_rz_rx` writer - "]
pub struct LO_LF_RZ_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_RZ_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `lo_lf_cz_tx` reader - "]
pub struct LO_LF_CZ_TX_R(crate::FieldReader<u8, u8>);
impl LO_LF_CZ_TX_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_CZ_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_CZ_TX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_cz_tx` writer - "]
pub struct LO_LF_CZ_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_CZ_TX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `lo_lf_cz_rx` reader - "]
pub struct LO_LF_CZ_RX_R(crate::FieldReader<u8, u8>);
impl LO_LF_CZ_RX_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_CZ_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_CZ_RX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_cz_rx` writer - "]
pub struct LO_LF_CZ_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_CZ_RX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `lo_cp_sel_tx` reader - "]
pub struct LO_CP_SEL_TX_R(crate::FieldReader<bool, bool>);
impl LO_CP_SEL_TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_CP_SEL_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_CP_SEL_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_cp_sel_tx` writer - "]
pub struct LO_CP_SEL_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_SEL_TX_W<'a> {
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
#[doc = "Field `lo_cp_sel_rx` reader - "]
pub struct LO_CP_SEL_RX_R(crate::FieldReader<bool, bool>);
impl LO_CP_SEL_RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_CP_SEL_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_CP_SEL_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_cp_sel_rx` writer - "]
pub struct LO_CP_SEL_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_CP_SEL_RX_W<'a> {
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
#[doc = "Field `lo_fbdv_halfstep_en_tx` reader - "]
pub struct LO_FBDV_HALFSTEP_EN_TX_R(crate::FieldReader<bool, bool>);
impl LO_FBDV_HALFSTEP_EN_TX_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_FBDV_HALFSTEP_EN_TX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_FBDV_HALFSTEP_EN_TX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_fbdv_halfstep_en_tx` writer - "]
pub struct LO_FBDV_HALFSTEP_EN_TX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_HALFSTEP_EN_TX_W<'a> {
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
#[doc = "Field `lo_fbdv_halfstep_en_rx` reader - "]
pub struct LO_FBDV_HALFSTEP_EN_RX_R(crate::FieldReader<bool, bool>);
impl LO_FBDV_HALFSTEP_EN_RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_FBDV_HALFSTEP_EN_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_FBDV_HALFSTEP_EN_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_fbdv_halfstep_en_rx` writer - "]
pub struct LO_FBDV_HALFSTEP_EN_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_FBDV_HALFSTEP_EN_RX_W<'a> {
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
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_lf_r4_tx(&self) -> LO_LF_R4_TX_R {
        LO_LF_R4_TX_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_lf_r4_rx(&self) -> LO_LF_R4_RX_R {
        LO_LF_R4_RX_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_rz_tx(&self) -> LO_LF_RZ_TX_R {
        LO_LF_RZ_TX_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz_rx(&self) -> LO_LF_RZ_RX_R {
        LO_LF_RZ_RX_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_tx(&self) -> LO_LF_CZ_TX_R {
        LO_LF_CZ_TX_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_cz_rx(&self) -> LO_LF_CZ_RX_R {
        LO_LF_CZ_RX_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lo_cp_sel_tx(&self) -> LO_CP_SEL_TX_R {
        LO_CP_SEL_TX_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lo_cp_sel_rx(&self) -> LO_CP_SEL_RX_R {
        LO_CP_SEL_RX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_tx(&self) -> LO_FBDV_HALFSTEP_EN_TX_R {
        LO_FBDV_HALFSTEP_EN_TX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_rx(&self) -> LO_FBDV_HALFSTEP_EN_RX_R {
        LO_FBDV_HALFSTEP_EN_RX_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn lo_lf_r4_tx(&mut self) -> LO_LF_R4_TX_W {
        LO_LF_R4_TX_W { w: self }
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn lo_lf_r4_rx(&mut self) -> LO_LF_R4_RX_W {
        LO_LF_R4_RX_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_rz_tx(&mut self) -> LO_LF_RZ_TX_W {
        LO_LF_RZ_TX_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz_rx(&mut self) -> LO_LF_RZ_RX_W {
        LO_LF_RZ_RX_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_tx(&mut self) -> LO_LF_CZ_TX_W {
        LO_LF_CZ_TX_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_cz_rx(&mut self) -> LO_LF_CZ_RX_W {
        LO_LF_CZ_RX_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lo_cp_sel_tx(&mut self) -> LO_CP_SEL_TX_W {
        LO_CP_SEL_TX_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn lo_cp_sel_rx(&mut self) -> LO_CP_SEL_RX_W {
        LO_CP_SEL_RX_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_tx(&mut self) -> LO_FBDV_HALFSTEP_EN_TX_W {
        LO_FBDV_HALFSTEP_EN_TX_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn lo_fbdv_halfstep_en_rx(&mut self) -> LO_FBDV_HALFSTEP_EN_RX_W {
        LO_FBDV_HALFSTEP_EN_RX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo_reg_ctrl_hw1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo_reg_ctrl_hw1](index.html) module"]
pub struct LO_REG_CTRL_HW1_SPEC;
impl crate::RegisterSpec for LO_REG_CTRL_HW1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo_reg_ctrl_hw1::R](R) reader structure"]
impl crate::Readable for LO_REG_CTRL_HW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo_reg_ctrl_hw1::W](W) writer structure"]
impl crate::Writable for LO_REG_CTRL_HW1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo_reg_ctrl_hw1 to value 0"]
impl crate::Resettable for LO_REG_CTRL_HW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
