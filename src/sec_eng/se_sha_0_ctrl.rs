#[doc = "Register `se_sha_0_ctrl` reader"]
pub struct R(crate::R<SE_SHA_0_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_SHA_0_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_SHA_0_CTRL_SPEC>> for R {
    fn from(reader: crate::R<SE_SHA_0_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_sha_0_ctrl` writer"]
pub struct W(crate::W<SE_SHA_0_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_SHA_0_CTRL_SPEC>;
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
impl core::convert::From<crate::W<SE_SHA_0_CTRL_SPEC>> for W {
    fn from(writer: crate::W<SE_SHA_0_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_sha_0_msg_len` reader - "]
pub struct SE_SHA_0_MSG_LEN_R(crate::FieldReader<u16, u16>);
impl SE_SHA_0_MSG_LEN_R {
    pub(crate) fn new(bits: u16) -> Self {
        SE_SHA_0_MSG_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_MSG_LEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_msg_len` writer - "]
pub struct SE_SHA_0_MSG_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_MSG_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `se_sha_0_link_mode` reader - "]
pub struct SE_SHA_0_LINK_MODE_R(crate::FieldReader<bool, bool>);
impl SE_SHA_0_LINK_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_0_LINK_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_LINK_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_link_mode` writer - "]
pub struct SE_SHA_0_LINK_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_LINK_MODE_W<'a> {
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
#[doc = "Field `se_sha_0_int_mask` reader - "]
pub struct SE_SHA_0_INT_MASK_R(crate::FieldReader<bool, bool>);
impl SE_SHA_0_INT_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_0_INT_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_INT_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_int_mask` writer - "]
pub struct SE_SHA_0_INT_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_INT_MASK_W<'a> {
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
#[doc = "Field `se_sha_0_int_set_1t` reader - "]
pub struct SE_SHA_0_INT_SET_1T_R(crate::FieldReader<bool, bool>);
impl SE_SHA_0_INT_SET_1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_0_INT_SET_1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_INT_SET_1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_int_set_1t` writer - "]
pub struct SE_SHA_0_INT_SET_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_INT_SET_1T_W<'a> {
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
#[doc = "Field `se_sha_0_int_clr_1t` reader - "]
pub struct SE_SHA_0_INT_CLR_1T_R(crate::FieldReader<bool, bool>);
impl SE_SHA_0_INT_CLR_1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_0_INT_CLR_1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_INT_CLR_1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_int_clr_1t` writer - "]
pub struct SE_SHA_0_INT_CLR_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_INT_CLR_1T_W<'a> {
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
#[doc = "Field `se_sha_0_int` reader - "]
pub struct SE_SHA_0_INT_R(crate::FieldReader<bool, bool>);
impl SE_SHA_0_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_0_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_hash_sel` reader - "]
pub struct SE_SHA_0_HASH_SEL_R(crate::FieldReader<bool, bool>);
impl SE_SHA_0_HASH_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_0_HASH_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_HASH_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_hash_sel` writer - "]
pub struct SE_SHA_0_HASH_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_HASH_SEL_W<'a> {
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
#[doc = "Field `se_sha_0_en` reader - "]
pub struct SE_SHA_0_EN_R(crate::FieldReader<bool, bool>);
impl SE_SHA_0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_en` writer - "]
pub struct SE_SHA_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_EN_W<'a> {
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
#[doc = "Field `se_sha_0_mode` reader - "]
pub struct SE_SHA_0_MODE_R(crate::FieldReader<u8, u8>);
impl SE_SHA_0_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE_SHA_0_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_mode` writer - "]
pub struct SE_SHA_0_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Field `se_sha_0_trig_1t` reader - "]
pub struct SE_SHA_0_TRIG_1T_R(crate::FieldReader<bool, bool>);
impl SE_SHA_0_TRIG_1T_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_0_TRIG_1T_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_TRIG_1T_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_sha_0_trig_1t` writer - "]
pub struct SE_SHA_0_TRIG_1T_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_SHA_0_TRIG_1T_W<'a> {
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
#[doc = "Field `se_sha_0_busy` reader - "]
pub struct SE_SHA_0_BUSY_R(crate::FieldReader<bool, bool>);
impl SE_SHA_0_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_SHA_0_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_SHA_0_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_sha_0_msg_len(&self) -> SE_SHA_0_MSG_LEN_R {
        SE_SHA_0_MSG_LEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_sha_0_link_mode(&self) -> SE_SHA_0_LINK_MODE_R {
        SE_SHA_0_LINK_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_sha_0_int_mask(&self) -> SE_SHA_0_INT_MASK_R {
        SE_SHA_0_INT_MASK_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_sha_0_int_set_1t(&self) -> SE_SHA_0_INT_SET_1T_R {
        SE_SHA_0_INT_SET_1T_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_sha_0_int_clr_1t(&self) -> SE_SHA_0_INT_CLR_1T_R {
        SE_SHA_0_INT_CLR_1T_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn se_sha_0_int(&self) -> SE_SHA_0_INT_R {
        SE_SHA_0_INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_sha_0_hash_sel(&self) -> SE_SHA_0_HASH_SEL_R {
        SE_SHA_0_HASH_SEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_sha_0_en(&self) -> SE_SHA_0_EN_R {
        SE_SHA_0_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn se_sha_0_mode(&self) -> SE_SHA_0_MODE_R {
        SE_SHA_0_MODE_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_0_trig_1t(&self) -> SE_SHA_0_TRIG_1T_R {
        SE_SHA_0_TRIG_1T_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_sha_0_busy(&self) -> SE_SHA_0_BUSY_R {
        SE_SHA_0_BUSY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn se_sha_0_msg_len(&mut self) -> SE_SHA_0_MSG_LEN_W {
        SE_SHA_0_MSG_LEN_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn se_sha_0_link_mode(&mut self) -> SE_SHA_0_LINK_MODE_W {
        SE_SHA_0_LINK_MODE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn se_sha_0_int_mask(&mut self) -> SE_SHA_0_INT_MASK_W {
        SE_SHA_0_INT_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn se_sha_0_int_set_1t(&mut self) -> SE_SHA_0_INT_SET_1T_W {
        SE_SHA_0_INT_SET_1T_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn se_sha_0_int_clr_1t(&mut self) -> SE_SHA_0_INT_CLR_1T_W {
        SE_SHA_0_INT_CLR_1T_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn se_sha_0_hash_sel(&mut self) -> SE_SHA_0_HASH_SEL_W {
        SE_SHA_0_HASH_SEL_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn se_sha_0_en(&mut self) -> SE_SHA_0_EN_W {
        SE_SHA_0_EN_W { w: self }
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn se_sha_0_mode(&mut self) -> SE_SHA_0_MODE_W {
        SE_SHA_0_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_sha_0_trig_1t(&mut self) -> SE_SHA_0_TRIG_1T_W {
        SE_SHA_0_TRIG_1T_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_sha_0_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_sha_0_ctrl](index.html) module"]
pub struct SE_SHA_0_CTRL_SPEC;
impl crate::RegisterSpec for SE_SHA_0_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_sha_0_ctrl::R](R) reader structure"]
impl crate::Readable for SE_SHA_0_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_sha_0_ctrl::W](W) writer structure"]
impl crate::Writable for SE_SHA_0_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_sha_0_ctrl to value 0"]
impl crate::Resettable for SE_SHA_0_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
