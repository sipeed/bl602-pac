#[doc = "Register `irrx_config` reader"]
pub struct R(crate::R<IRRX_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRRX_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IRRX_CONFIG_SPEC>> for R {
    fn from(reader: crate::R<IRRX_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irrx_config` writer"]
pub struct W(crate::W<IRRX_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRRX_CONFIG_SPEC>;
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
impl core::convert::From<crate::W<IRRX_CONFIG_SPEC>> for W {
    fn from(writer: crate::W<IRRX_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irrx_deg_cnt` reader - "]
pub struct CR_IRRX_DEG_CNT_R(crate::FieldReader<u8, u8>);
impl CR_IRRX_DEG_CNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRRX_DEG_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRRX_DEG_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irrx_deg_cnt` writer - "]
pub struct CR_IRRX_DEG_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_DEG_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `cr_irrx_deg_en` reader - "]
pub struct CR_IRRX_DEG_EN_R(crate::FieldReader<bool, bool>);
impl CR_IRRX_DEG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRRX_DEG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRRX_DEG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irrx_deg_en` writer - "]
pub struct CR_IRRX_DEG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_DEG_EN_W<'a> {
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
#[doc = "Field `cr_irrx_mode` reader - "]
pub struct CR_IRRX_MODE_R(crate::FieldReader<u8, u8>);
impl CR_IRRX_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRRX_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRRX_MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irrx_mode` writer - "]
pub struct CR_IRRX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `cr_irrx_in_inv` reader - "]
pub struct CR_IRRX_IN_INV_R(crate::FieldReader<bool, bool>);
impl CR_IRRX_IN_INV_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRRX_IN_INV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRRX_IN_INV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irrx_in_inv` writer - "]
pub struct CR_IRRX_IN_INV_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_IN_INV_W<'a> {
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
#[doc = "Field `cr_irrx_en` reader - "]
pub struct CR_IRRX_EN_R(crate::FieldReader<bool, bool>);
impl CR_IRRX_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_IRRX_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRRX_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irrx_en` writer - "]
pub struct CR_IRRX_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRRX_EN_W<'a> {
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
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irrx_deg_cnt(&self) -> CR_IRRX_DEG_CNT_R {
        CR_IRRX_DEG_CNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irrx_deg_en(&self) -> CR_IRRX_DEG_EN_R {
        CR_IRRX_DEG_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_irrx_mode(&self) -> CR_IRRX_MODE_R {
        CR_IRRX_MODE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irrx_in_inv(&self) -> CR_IRRX_IN_INV_R {
        CR_IRRX_IN_INV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irrx_en(&self) -> CR_IRRX_EN_R {
        CR_IRRX_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irrx_deg_cnt(&mut self) -> CR_IRRX_DEG_CNT_W {
        CR_IRRX_DEG_CNT_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_irrx_deg_en(&mut self) -> CR_IRRX_DEG_EN_W {
        CR_IRRX_DEG_EN_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cr_irrx_mode(&mut self) -> CR_IRRX_MODE_W {
        CR_IRRX_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_irrx_in_inv(&mut self) -> CR_IRRX_IN_INV_W {
        CR_IRRX_IN_INV_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_irrx_en(&mut self) -> CR_IRRX_EN_W {
        CR_IRRX_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irrx_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irrx_config](index.html) module"]
pub struct IRRX_CONFIG_SPEC;
impl crate::RegisterSpec for IRRX_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irrx_config::R](R) reader structure"]
impl crate::Readable for IRRX_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irrx_config::W](W) writer structure"]
impl crate::Writable for IRRX_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irrx_config to value 0"]
impl crate::Resettable for IRRX_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
