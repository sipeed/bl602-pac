#[doc = "Register `se_pka_0_ctrl_0` reader"]
pub struct R(crate::R<SE_PKA_0_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_PKA_0_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_PKA_0_CTRL_0_SPEC>> for R {
    fn from(reader: crate::R<SE_PKA_0_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_pka_0_ctrl_0` writer"]
pub struct W(crate::W<SE_PKA_0_CTRL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_PKA_0_CTRL_0_SPEC>;
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
impl core::convert::From<crate::W<SE_PKA_0_CTRL_0_SPEC>> for W {
    fn from(writer: crate::W<SE_PKA_0_CTRL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_pka_0_status` reader - "]
pub struct SE_PKA_0_STATUS_R(crate::FieldReader<u16, u16>);
impl SE_PKA_0_STATUS_R {
    pub(crate) fn new(bits: u16) -> Self {
        SE_PKA_0_STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_STATUS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_status_clr_1t` reader - "]
pub struct SE_PKA_0_STATUS_CLR_1T_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_STATUS_CLR_1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_STATUS_CLR_1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_STATUS_CLR_1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_status_clr_1t` writer - "]
pub struct SE_PKA_0_STATUS_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_STATUS_CLR_1T_W<'a> {
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
#[doc = "Field `se_pka_0_ram_clr_md` reader - "]
pub struct SE_PKA_0_RAM_CLR_MD_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_RAM_CLR_MD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_RAM_CLR_MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_RAM_CLR_MD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_ram_clr_md` writer - "]
pub struct SE_PKA_0_RAM_CLR_MD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_RAM_CLR_MD_W<'a> {
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
#[doc = "Field `se_pka_0_endian` reader - "]
pub struct SE_PKA_0_ENDIAN_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_endian` writer - "]
pub struct SE_PKA_0_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_ENDIAN_W<'a> {
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
#[doc = "Field `se_pka_0_int_mask` reader - "]
pub struct SE_PKA_0_INT_MASK_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_INT_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_int_mask` writer - "]
pub struct SE_PKA_0_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_INT_MASK_W<'a> {
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
#[doc = "Field `se_pka_0_int_set` reader - "]
pub struct SE_PKA_0_INT_SET_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_INT_SET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_INT_SET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_INT_SET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_int_set` writer - "]
pub struct SE_PKA_0_INT_SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_INT_SET_W<'a> {
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
#[doc = "Field `se_pka_0_int_clr_1t` reader - "]
pub struct SE_PKA_0_INT_CLR_1T_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_INT_CLR_1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_INT_CLR_1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_INT_CLR_1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_int_clr_1t` writer - "]
pub struct SE_PKA_0_INT_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_INT_CLR_1T_W<'a> {
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
#[doc = "Field `se_pka_0_int` reader - "]
pub struct SE_PKA_0_INT_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_prot_md` reader - "]
pub struct SE_PKA_0_PROT_MD_R(crate::FieldReader<u8, u8>);
impl SE_PKA_0_PROT_MD_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE_PKA_0_PROT_MD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_PROT_MD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_prot_md` writer - "]
pub struct SE_PKA_0_PROT_MD_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_PROT_MD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `se_pka_0_en` reader - "]
pub struct SE_PKA_0_EN_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_en` writer - "]
pub struct SE_PKA_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_EN_W<'a> {
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
#[doc = "Field `se_pka_0_busy` reader - "]
pub struct SE_PKA_0_BUSY_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_done_clr_1t` reader - "]
pub struct SE_PKA_0_DONE_CLR_1T_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_DONE_CLR_1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_DONE_CLR_1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_DONE_CLR_1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_pka_0_done_clr_1t` writer - "]
pub struct SE_PKA_0_DONE_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_PKA_0_DONE_CLR_1T_W<'a> {
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
#[doc = "Field `se_pka_0_done` reader - "]
pub struct SE_PKA_0_DONE_R(crate::FieldReader<bool, bool>);
impl SE_PKA_0_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_PKA_0_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_PKA_0_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn se_pka_0_status(&self) -> SE_PKA_0_STATUS_R {
        SE_PKA_0_STATUS_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn se_pka_0_status_clr_1t(&self) -> SE_PKA_0_STATUS_CLR_1T_R {
        SE_PKA_0_STATUS_CLR_1T_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_0_ram_clr_md(&self) -> SE_PKA_0_RAM_CLR_MD_R {
        SE_PKA_0_RAM_CLR_MD_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_0_endian(&self) -> SE_PKA_0_ENDIAN_R {
        SE_PKA_0_ENDIAN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_pka_0_int_mask(&self) -> SE_PKA_0_INT_MASK_R {
        SE_PKA_0_INT_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_pka_0_int_set(&self) -> SE_PKA_0_INT_SET_R {
        SE_PKA_0_INT_SET_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_pka_0_int_clr_1t(&self) -> SE_PKA_0_INT_CLR_1T_R {
        SE_PKA_0_INT_CLR_1T_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_pka_0_int(&self) -> SE_PKA_0_INT_R {
        SE_PKA_0_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn se_pka_0_prot_md(&self) -> SE_PKA_0_PROT_MD_R {
        SE_PKA_0_PROT_MD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_en(&self) -> SE_PKA_0_EN_R {
        SE_PKA_0_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_pka_0_busy(&self) -> SE_PKA_0_BUSY_R {
        SE_PKA_0_BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_pka_0_done_clr_1t(&self) -> SE_PKA_0_DONE_CLR_1T_R {
        SE_PKA_0_DONE_CLR_1T_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_pka_0_done(&self) -> SE_PKA_0_DONE_R {
        SE_PKA_0_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn se_pka_0_status_clr_1t(&mut self) -> SE_PKA_0_STATUS_CLR_1T_W {
        SE_PKA_0_STATUS_CLR_1T_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn se_pka_0_ram_clr_md(&mut self) -> SE_PKA_0_RAM_CLR_MD_W {
        SE_PKA_0_RAM_CLR_MD_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn se_pka_0_endian(&mut self) -> SE_PKA_0_ENDIAN_W {
        SE_PKA_0_ENDIAN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_pka_0_int_mask(&mut self) -> SE_PKA_0_INT_MASK_W {
        SE_PKA_0_INT_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_pka_0_int_set(&mut self) -> SE_PKA_0_INT_SET_W {
        SE_PKA_0_INT_SET_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_pka_0_int_clr_1t(&mut self) -> SE_PKA_0_INT_CLR_1T_W {
        SE_PKA_0_INT_CLR_1T_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn se_pka_0_prot_md(&mut self) -> SE_PKA_0_PROT_MD_W {
        SE_PKA_0_PROT_MD_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_pka_0_en(&mut self) -> SE_PKA_0_EN_W {
        SE_PKA_0_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_pka_0_done_clr_1t(&mut self) -> SE_PKA_0_DONE_CLR_1T_W {
        SE_PKA_0_DONE_CLR_1T_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_pka_0_ctrl_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_pka_0_ctrl_0](index.html) module"]
pub struct SE_PKA_0_CTRL_0_SPEC;
impl crate::RegisterSpec for SE_PKA_0_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_pka_0_ctrl_0::R](R) reader structure"]
impl crate::Readable for SE_PKA_0_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_pka_0_ctrl_0::W](W) writer structure"]
impl crate::Writable for SE_PKA_0_CTRL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_pka_0_ctrl_0 to value 0"]
impl crate::Resettable for SE_PKA_0_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
