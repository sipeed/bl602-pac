#[doc = "Register `utx_config` reader"]
pub struct R(crate::R<UTX_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UTX_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UTX_CONFIG_SPEC>> for R {
    fn from(reader: crate::R<UTX_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `utx_config` writer"]
pub struct W(crate::W<UTX_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UTX_CONFIG_SPEC>;
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
impl core::convert::From<crate::W<UTX_CONFIG_SPEC>> for W {
    fn from(writer: crate::W<UTX_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_utx_len` reader - "]
pub struct CR_UTX_LEN_R(crate::FieldReader<u16, u16>);
impl CR_UTX_LEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_UTX_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_LEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_len` writer - "]
pub struct CR_UTX_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `cr_utx_bit_cnt_p` reader - "]
pub struct CR_UTX_BIT_CNT_P_R(crate::FieldReader<u8, u8>);
impl CR_UTX_BIT_CNT_P_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_UTX_BIT_CNT_P_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_BIT_CNT_P_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_bit_cnt_p` writer - "]
pub struct CR_UTX_BIT_CNT_P_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_BIT_CNT_P_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `cr_utx_bit_cnt_d` reader - "]
pub struct CR_UTX_BIT_CNT_D_R(crate::FieldReader<u8, u8>);
impl CR_UTX_BIT_CNT_D_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_UTX_BIT_CNT_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_BIT_CNT_D_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_bit_cnt_d` writer - "]
pub struct CR_UTX_BIT_CNT_D_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_BIT_CNT_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `cr_utx_ir_inv` reader - "]
pub struct CR_UTX_IR_INV_R(crate::FieldReader<bool, bool>);
impl CR_UTX_IR_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_UTX_IR_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_IR_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_ir_inv` writer - "]
pub struct CR_UTX_IR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_IR_INV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `cr_utx_ir_en` reader - "]
pub struct CR_UTX_IR_EN_R(crate::FieldReader<bool, bool>);
impl CR_UTX_IR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_UTX_IR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_IR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_ir_en` writer - "]
pub struct CR_UTX_IR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_IR_EN_W<'a> {
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
#[doc = "Field `cr_utx_prt_sel` reader - "]
pub struct CR_UTX_PRT_SEL_R(crate::FieldReader<bool, bool>);
impl CR_UTX_PRT_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_UTX_PRT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_PRT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_prt_sel` writer - "]
pub struct CR_UTX_PRT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_PRT_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `cr_utx_prt_en` reader - "]
pub struct CR_UTX_PRT_EN_R(crate::FieldReader<bool, bool>);
impl CR_UTX_PRT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_UTX_PRT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_PRT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_prt_en` writer - "]
pub struct CR_UTX_PRT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_PRT_EN_W<'a> {
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
#[doc = "Field `cr_utx_frm_en` reader - "]
pub struct CR_UTX_FRM_EN_R(crate::FieldReader<bool, bool>);
impl CR_UTX_FRM_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_UTX_FRM_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_FRM_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_frm_en` writer - "]
pub struct CR_UTX_FRM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_FRM_EN_W<'a> {
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
#[doc = "Field `cr_utx_cts_en` reader - "]
pub struct CR_UTX_CTS_EN_R(crate::FieldReader<bool, bool>);
impl CR_UTX_CTS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_UTX_CTS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_CTS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_cts_en` writer - "]
pub struct CR_UTX_CTS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_CTS_EN_W<'a> {
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
#[doc = "Field `cr_utx_en` reader - "]
pub struct CR_UTX_EN_R(crate::FieldReader<bool, bool>);
impl CR_UTX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_UTX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_en` writer - "]
pub struct CR_UTX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_EN_W<'a> {
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
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_len(&self) -> CR_UTX_LEN_R {
        CR_UTX_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_p(&self) -> CR_UTX_BIT_CNT_P_R {
        CR_UTX_BIT_CNT_P_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_d(&self) -> CR_UTX_BIT_CNT_D_R {
        CR_UTX_BIT_CNT_D_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_utx_ir_inv(&self) -> CR_UTX_IR_INV_R {
        CR_UTX_IR_INV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_ir_en(&self) -> CR_UTX_IR_EN_R {
        CR_UTX_IR_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_utx_prt_sel(&self) -> CR_UTX_PRT_SEL_R {
        CR_UTX_PRT_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_utx_prt_en(&self) -> CR_UTX_PRT_EN_R {
        CR_UTX_PRT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_frm_en(&self) -> CR_UTX_FRM_EN_R {
        CR_UTX_FRM_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_cts_en(&self) -> CR_UTX_CTS_EN_R {
        CR_UTX_CTS_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_en(&self) -> CR_UTX_EN_R {
        CR_UTX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_utx_len(&mut self) -> CR_UTX_LEN_W {
        CR_UTX_LEN_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_p(&mut self) -> CR_UTX_BIT_CNT_P_W {
        CR_UTX_BIT_CNT_P_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_utx_bit_cnt_d(&mut self) -> CR_UTX_BIT_CNT_D_W {
        CR_UTX_BIT_CNT_D_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_utx_ir_inv(&mut self) -> CR_UTX_IR_INV_W {
        CR_UTX_IR_INV_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_ir_en(&mut self) -> CR_UTX_IR_EN_W {
        CR_UTX_IR_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_utx_prt_sel(&mut self) -> CR_UTX_PRT_SEL_W {
        CR_UTX_PRT_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_utx_prt_en(&mut self) -> CR_UTX_PRT_EN_W {
        CR_UTX_PRT_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_frm_en(&mut self) -> CR_UTX_FRM_EN_W {
        CR_UTX_FRM_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_utx_cts_en(&mut self) -> CR_UTX_CTS_EN_W {
        CR_UTX_CTS_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_en(&mut self) -> CR_UTX_EN_W {
        CR_UTX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "utx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [utx_config](index.html) module"]
pub struct UTX_CONFIG_SPEC;
impl crate::RegisterSpec for UTX_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [utx_config::R](R) reader structure"]
impl crate::Readable for UTX_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [utx_config::W](W) writer structure"]
impl crate::Writable for UTX_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets utx_config to value 0"]
impl crate::Resettable for UTX_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
