#[doc = "Register `urx_config` reader"]
pub struct R(crate::R<URX_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<URX_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<URX_CONFIG_SPEC>> for R {
    fn from(reader: crate::R<URX_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `urx_config` writer"]
pub struct W(crate::W<URX_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<URX_CONFIG_SPEC>;
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
impl core::convert::From<crate::W<URX_CONFIG_SPEC>> for W {
    fn from(writer: crate::W<URX_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_urx_len` reader - "]
pub struct CR_URX_LEN_R(crate::FieldReader<u16, u16>);
impl CR_URX_LEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        CR_URX_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_LEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_len` writer - "]
pub struct CR_URX_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `cr_urx_deg_cnt` reader - "]
pub struct CR_URX_DEG_CNT_R(crate::FieldReader<u8, u8>);
impl CR_URX_DEG_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_URX_DEG_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_DEG_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_deg_cnt` writer - "]
pub struct CR_URX_DEG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_DEG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `cr_urx_deg_en` reader - "]
pub struct CR_URX_DEG_EN_R(crate::FieldReader<bool, bool>);
impl CR_URX_DEG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_DEG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_DEG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_deg_en` writer - "]
pub struct CR_URX_DEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_DEG_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `cr_urx_bit_cnt_d` reader - "]
pub struct CR_URX_BIT_CNT_D_R(crate::FieldReader<u8, u8>);
impl CR_URX_BIT_CNT_D_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_URX_BIT_CNT_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_BIT_CNT_D_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_bit_cnt_d` writer - "]
pub struct CR_URX_BIT_CNT_D_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_BIT_CNT_D_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `cr_urx_ir_inv` reader - "]
pub struct CR_URX_IR_INV_R(crate::FieldReader<bool, bool>);
impl CR_URX_IR_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_IR_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_IR_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_ir_inv` writer - "]
pub struct CR_URX_IR_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_IR_INV_W<'a> {
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
#[doc = "Field `cr_urx_ir_en` reader - "]
pub struct CR_URX_IR_EN_R(crate::FieldReader<bool, bool>);
impl CR_URX_IR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_IR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_IR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_ir_en` writer - "]
pub struct CR_URX_IR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_IR_EN_W<'a> {
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
#[doc = "Field `cr_urx_prt_sel` reader - "]
pub struct CR_URX_PRT_SEL_R(crate::FieldReader<bool, bool>);
impl CR_URX_PRT_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_PRT_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_PRT_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_prt_sel` writer - "]
pub struct CR_URX_PRT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_PRT_SEL_W<'a> {
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
#[doc = "Field `cr_urx_prt_en` reader - "]
pub struct CR_URX_PRT_EN_R(crate::FieldReader<bool, bool>);
impl CR_URX_PRT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_PRT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_PRT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_prt_en` writer - "]
pub struct CR_URX_PRT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_PRT_EN_W<'a> {
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
#[doc = "Field `cr_urx_abr_en` reader - "]
pub struct CR_URX_ABR_EN_R(crate::FieldReader<bool, bool>);
impl CR_URX_ABR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_ABR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_ABR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_abr_en` writer - "]
pub struct CR_URX_ABR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_ABR_EN_W<'a> {
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
#[doc = "Field `cr_urx_rts_sw_val` reader - "]
pub struct CR_URX_RTS_SW_VAL_R(crate::FieldReader<bool, bool>);
impl CR_URX_RTS_SW_VAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_RTS_SW_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_RTS_SW_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_rts_sw_val` writer - "]
pub struct CR_URX_RTS_SW_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_RTS_SW_VAL_W<'a> {
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
#[doc = "Field `cr_urx_rts_sw_mode` reader - "]
pub struct CR_URX_RTS_SW_MODE_R(crate::FieldReader<bool, bool>);
impl CR_URX_RTS_SW_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_RTS_SW_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_RTS_SW_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_rts_sw_mode` writer - "]
pub struct CR_URX_RTS_SW_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_RTS_SW_MODE_W<'a> {
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
#[doc = "Field `cr_urx_en` reader - "]
pub struct CR_URX_EN_R(crate::FieldReader<bool, bool>);
impl CR_URX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_en` writer - "]
pub struct CR_URX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_EN_W<'a> {
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
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_urx_len(&self) -> CR_URX_LEN_R {
        CR_URX_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_urx_deg_cnt(&self) -> CR_URX_DEG_CNT_R {
        CR_URX_DEG_CNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_deg_en(&self) -> CR_URX_DEG_EN_R {
        CR_URX_DEG_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_urx_bit_cnt_d(&self) -> CR_URX_BIT_CNT_D_R {
        CR_URX_BIT_CNT_D_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_ir_inv(&self) -> CR_URX_IR_INV_R {
        CR_URX_IR_INV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_urx_ir_en(&self) -> CR_URX_IR_EN_R {
        CR_URX_IR_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_prt_sel(&self) -> CR_URX_PRT_SEL_R {
        CR_URX_PRT_SEL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_prt_en(&self) -> CR_URX_PRT_EN_R {
        CR_URX_PRT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_abr_en(&self) -> CR_URX_ABR_EN_R {
        CR_URX_ABR_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_val(&self) -> CR_URX_RTS_SW_VAL_R {
        CR_URX_RTS_SW_VAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_mode(&self) -> CR_URX_RTS_SW_MODE_R {
        CR_URX_RTS_SW_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_urx_en(&self) -> CR_URX_EN_R {
        CR_URX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cr_urx_len(&mut self) -> CR_URX_LEN_W {
        CR_URX_LEN_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_urx_deg_cnt(&mut self) -> CR_URX_DEG_CNT_W {
        CR_URX_DEG_CNT_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_urx_deg_en(&mut self) -> CR_URX_DEG_EN_W {
        CR_URX_DEG_EN_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn cr_urx_bit_cnt_d(&mut self) -> CR_URX_BIT_CNT_D_W {
        CR_URX_BIT_CNT_D_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_ir_inv(&mut self) -> CR_URX_IR_INV_W {
        CR_URX_IR_INV_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_urx_ir_en(&mut self) -> CR_URX_IR_EN_W {
        CR_URX_IR_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_prt_sel(&mut self) -> CR_URX_PRT_SEL_W {
        CR_URX_PRT_SEL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_prt_en(&mut self) -> CR_URX_PRT_EN_W {
        CR_URX_PRT_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_abr_en(&mut self) -> CR_URX_ABR_EN_W {
        CR_URX_ABR_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_val(&mut self) -> CR_URX_RTS_SW_VAL_W {
        CR_URX_RTS_SW_VAL_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_rts_sw_mode(&mut self) -> CR_URX_RTS_SW_MODE_W {
        CR_URX_RTS_SW_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_urx_en(&mut self) -> CR_URX_EN_W {
        CR_URX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "urx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [urx_config](index.html) module"]
pub struct URX_CONFIG_SPEC;
impl crate::RegisterSpec for URX_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [urx_config::R](R) reader structure"]
impl crate::Readable for URX_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [urx_config::W](W) writer structure"]
impl crate::Writable for URX_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets urx_config to value 0x0700"]
impl crate::Resettable for URX_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0700
    }
}
