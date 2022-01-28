#[doc = "Register `gpdac_ctrl` reader"]
pub struct R(crate::R<GPDAC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPDAC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPDAC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPDAC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpdac_ctrl` writer"]
pub struct W(crate::W<GPDAC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPDAC_CTRL_SPEC>;
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
impl From<crate::W<GPDAC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPDAC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpdac_reserved` reader - "]
pub struct GPDAC_RESERVED_R(crate::FieldReader<u8, u8>);
impl GPDAC_RESERVED_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPDAC_RESERVED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_RESERVED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_reserved` writer - "]
pub struct GPDAC_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_RESERVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `gpdac_test_sel` reader - "]
pub struct GPDAC_TEST_SEL_R(crate::FieldReader<u8, u8>);
impl GPDAC_TEST_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPDAC_TEST_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_TEST_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_test_sel` writer - "]
pub struct GPDAC_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `gpdac_ref_sel` reader - "]
pub struct GPDAC_REF_SEL_R(crate::FieldReader<bool, bool>);
impl GPDAC_REF_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPDAC_REF_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_REF_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_ref_sel` writer - "]
pub struct GPDAC_REF_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_REF_SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `gpdac_test_en` reader - "]
pub struct GPDAC_TEST_EN_R(crate::FieldReader<bool, bool>);
impl GPDAC_TEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPDAC_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDAC_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdac_test_en` writer - "]
pub struct GPDAC_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDAC_TEST_EN_W<'a> {
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
#[doc = "Field `gpdacb_rstn_ana` reader - "]
pub struct GPDACB_RSTN_ANA_R(crate::FieldReader<bool, bool>);
impl GPDACB_RSTN_ANA_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPDACB_RSTN_ANA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDACB_RSTN_ANA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdacb_rstn_ana` writer - "]
pub struct GPDACB_RSTN_ANA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDACB_RSTN_ANA_W<'a> {
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
#[doc = "Field `gpdaca_rstn_ana` reader - "]
pub struct GPDACA_RSTN_ANA_R(crate::FieldReader<bool, bool>);
impl GPDACA_RSTN_ANA_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPDACA_RSTN_ANA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPDACA_RSTN_ANA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpdaca_rstn_ana` writer - "]
pub struct GPDACA_RSTN_ANA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPDACA_RSTN_ANA_W<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn gpdac_reserved(&self) -> GPDAC_RESERVED_R {
        GPDAC_RESERVED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn gpdac_test_sel(&self) -> GPDAC_TEST_SEL_R {
        GPDAC_TEST_SEL_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpdac_ref_sel(&self) -> GPDAC_REF_SEL_R {
        GPDAC_REF_SEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpdac_test_en(&self) -> GPDAC_TEST_EN_R {
        GPDAC_TEST_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdacb_rstn_ana(&self) -> GPDACB_RSTN_ANA_R {
        GPDACB_RSTN_ANA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdaca_rstn_ana(&self) -> GPDACA_RSTN_ANA_R {
        GPDACA_RSTN_ANA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn gpdac_reserved(&mut self) -> GPDAC_RESERVED_W {
        GPDAC_RESERVED_W { w: self }
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn gpdac_test_sel(&mut self) -> GPDAC_TEST_SEL_W {
        GPDAC_TEST_SEL_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpdac_ref_sel(&mut self) -> GPDAC_REF_SEL_W {
        GPDAC_REF_SEL_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gpdac_test_en(&mut self) -> GPDAC_TEST_EN_W {
        GPDAC_TEST_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpdacb_rstn_ana(&mut self) -> GPDACB_RSTN_ANA_W {
        GPDACB_RSTN_ANA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpdaca_rstn_ana(&mut self) -> GPDACA_RSTN_ANA_W {
        GPDACA_RSTN_ANA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpdac_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpdac_ctrl](index.html) module"]
pub struct GPDAC_CTRL_SPEC;
impl crate::RegisterSpec for GPDAC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpdac_ctrl::R](R) reader structure"]
impl crate::Readable for GPDAC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpdac_ctrl::W](W) writer structure"]
impl crate::Writable for GPDAC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpdac_ctrl to value 0x03"]
impl crate::Resettable for GPDAC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
