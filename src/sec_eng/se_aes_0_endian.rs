#[doc = "Register `se_aes_0_endian` reader"]
pub struct R(crate::R<SE_AES_0_ENDIAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SE_AES_0_ENDIAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SE_AES_0_ENDIAN_SPEC>> for R {
    fn from(reader: crate::R<SE_AES_0_ENDIAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `se_aes_0_endian` writer"]
pub struct W(crate::W<SE_AES_0_ENDIAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SE_AES_0_ENDIAN_SPEC>;
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
impl core::convert::From<crate::W<SE_AES_0_ENDIAN_SPEC>> for W {
    fn from(writer: crate::W<SE_AES_0_ENDIAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `se_aes_0_ctr_len` reader - "]
pub struct SE_AES_0_CTR_LEN_R(crate::FieldReader<u8, u8>);
impl SE_AES_0_CTR_LEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE_AES_0_CTR_LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_0_CTR_LEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_0_ctr_len` writer - "]
pub struct SE_AES_0_CTR_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_CTR_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `se_aes_0_iv_endian` reader - "]
pub struct SE_AES_0_IV_ENDIAN_R(crate::FieldReader<bool, bool>);
impl SE_AES_0_IV_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_AES_0_IV_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_0_IV_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_0_iv_endian` writer - "]
pub struct SE_AES_0_IV_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_IV_ENDIAN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `se_aes_0_key_endian` reader - "]
pub struct SE_AES_0_KEY_ENDIAN_R(crate::FieldReader<bool, bool>);
impl SE_AES_0_KEY_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_AES_0_KEY_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_0_KEY_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_0_key_endian` writer - "]
pub struct SE_AES_0_KEY_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_KEY_ENDIAN_W<'a> {
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
#[doc = "Field `se_aes_0_din_endian` reader - "]
pub struct SE_AES_0_DIN_ENDIAN_R(crate::FieldReader<bool, bool>);
impl SE_AES_0_DIN_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_AES_0_DIN_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_0_DIN_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_0_din_endian` writer - "]
pub struct SE_AES_0_DIN_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_DIN_ENDIAN_W<'a> {
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
#[doc = "Field `se_aes_0_dout_endian` reader - "]
pub struct SE_AES_0_DOUT_ENDIAN_R(crate::FieldReader<bool, bool>);
impl SE_AES_0_DOUT_ENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SE_AES_0_DOUT_ENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE_AES_0_DOUT_ENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `se_aes_0_dout_endian` writer - "]
pub struct SE_AES_0_DOUT_ENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> SE_AES_0_DOUT_ENDIAN_W<'a> {
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
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn se_aes_0_ctr_len(&self) -> SE_AES_0_CTR_LEN_R {
        SE_AES_0_CTR_LEN_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_aes_0_iv_endian(&self) -> SE_AES_0_IV_ENDIAN_R {
        SE_AES_0_IV_ENDIAN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_key_endian(&self) -> SE_AES_0_KEY_ENDIAN_R {
        SE_AES_0_KEY_ENDIAN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_0_din_endian(&self) -> SE_AES_0_DIN_ENDIAN_R {
        SE_AES_0_DIN_ENDIAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_dout_endian(&self) -> SE_AES_0_DOUT_ENDIAN_R {
        SE_AES_0_DOUT_ENDIAN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn se_aes_0_ctr_len(&mut self) -> SE_AES_0_CTR_LEN_W {
        SE_AES_0_CTR_LEN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn se_aes_0_iv_endian(&mut self) -> SE_AES_0_IV_ENDIAN_W {
        SE_AES_0_IV_ENDIAN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn se_aes_0_key_endian(&mut self) -> SE_AES_0_KEY_ENDIAN_W {
        SE_AES_0_KEY_ENDIAN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn se_aes_0_din_endian(&mut self) -> SE_AES_0_DIN_ENDIAN_W {
        SE_AES_0_DIN_ENDIAN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn se_aes_0_dout_endian(&mut self) -> SE_AES_0_DOUT_ENDIAN_W {
        SE_AES_0_DOUT_ENDIAN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "se_aes_0_endian.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [se_aes_0_endian](index.html) module"]
pub struct SE_AES_0_ENDIAN_SPEC;
impl crate::RegisterSpec for SE_AES_0_ENDIAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [se_aes_0_endian::R](R) reader structure"]
impl crate::Readable for SE_AES_0_ENDIAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [se_aes_0_endian::W](W) writer structure"]
impl crate::Writable for SE_AES_0_ENDIAN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets se_aes_0_endian to value 0"]
impl crate::Resettable for SE_AES_0_ENDIAN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
