#[doc = "Register `acomp_ctrl` reader"]
pub struct R(crate::R<ACOMP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACOMP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ACOMP_CTRL_SPEC>> for R {
    fn from(reader: crate::R<ACOMP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `acomp_ctrl` writer"]
pub struct W(crate::W<ACOMP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACOMP_CTRL_SPEC>;
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
impl core::convert::From<crate::W<ACOMP_CTRL_SPEC>> for W {
    fn from(writer: crate::W<ACOMP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `acomp_reserved` reader - "]
pub struct ACOMP_RESERVED_R(crate::FieldReader<u8, u8>);
impl ACOMP_RESERVED_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACOMP_RESERVED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP_RESERVED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp_reserved` writer - "]
pub struct ACOMP_RESERVED_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP_RESERVED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `acomp0_out_raw` reader - "]
pub struct ACOMP0_OUT_RAW_R(crate::FieldReader<bool, bool>);
impl ACOMP0_OUT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACOMP0_OUT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_OUT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_out_raw` writer - "]
pub struct ACOMP0_OUT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_OUT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `acomp1_out_raw` reader - "]
pub struct ACOMP1_OUT_RAW_R(crate::FieldReader<bool, bool>);
impl ACOMP1_OUT_RAW_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACOMP1_OUT_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP1_OUT_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp1_out_raw` writer - "]
pub struct ACOMP1_OUT_RAW_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_OUT_RAW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `acomp0_test_sel` reader - "]
pub struct ACOMP0_TEST_SEL_R(crate::FieldReader<u8, u8>);
impl ACOMP0_TEST_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACOMP0_TEST_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_TEST_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_test_sel` writer - "]
pub struct ACOMP0_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `acomp1_test_sel` reader - "]
pub struct ACOMP1_TEST_SEL_R(crate::FieldReader<u8, u8>);
impl ACOMP1_TEST_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACOMP1_TEST_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP1_TEST_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp1_test_sel` writer - "]
pub struct ACOMP1_TEST_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_TEST_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Field `acomp0_test_en` reader - "]
pub struct ACOMP0_TEST_EN_R(crate::FieldReader<bool, bool>);
impl ACOMP0_TEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACOMP0_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_test_en` writer - "]
pub struct ACOMP0_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_TEST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `acomp1_test_en` reader - "]
pub struct ACOMP1_TEST_EN_R(crate::FieldReader<bool, bool>);
impl ACOMP1_TEST_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACOMP1_TEST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP1_TEST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp1_test_en` writer - "]
pub struct ACOMP1_TEST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_TEST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `acomp0_rstn_ana` reader - "]
pub struct ACOMP0_RSTN_ANA_R(crate::FieldReader<bool, bool>);
impl ACOMP0_RSTN_ANA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACOMP0_RSTN_ANA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP0_RSTN_ANA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp0_rstn_ana` writer - "]
pub struct ACOMP0_RSTN_ANA_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP0_RSTN_ANA_W<'a> {
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
#[doc = "Field `acomp1_rstn_ana` reader - "]
pub struct ACOMP1_RSTN_ANA_R(crate::FieldReader<bool, bool>);
impl ACOMP1_RSTN_ANA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACOMP1_RSTN_ANA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACOMP1_RSTN_ANA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `acomp1_rstn_ana` writer - "]
pub struct ACOMP1_RSTN_ANA_W<'a> {
    w: &'a mut W,
}
impl<'a> ACOMP1_RSTN_ANA_W<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn acomp_reserved(&self) -> ACOMP_RESERVED_R {
        ACOMP_RESERVED_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn acomp0_out_raw(&self) -> ACOMP0_OUT_RAW_R {
        ACOMP0_OUT_RAW_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn acomp1_out_raw(&self) -> ACOMP1_OUT_RAW_R {
        ACOMP1_OUT_RAW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn acomp0_test_sel(&self) -> ACOMP0_TEST_SEL_R {
        ACOMP0_TEST_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_test_sel(&self) -> ACOMP1_TEST_SEL_R {
        ACOMP1_TEST_SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn acomp0_test_en(&self) -> ACOMP0_TEST_EN_R {
        ACOMP0_TEST_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn acomp1_test_en(&self) -> ACOMP1_TEST_EN_R {
        ACOMP1_TEST_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acomp0_rstn_ana(&self) -> ACOMP0_RSTN_ANA_R {
        ACOMP0_RSTN_ANA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_rstn_ana(&self) -> ACOMP1_RSTN_ANA_R {
        ACOMP1_RSTN_ANA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn acomp_reserved(&mut self) -> ACOMP_RESERVED_W {
        ACOMP_RESERVED_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn acomp0_out_raw(&mut self) -> ACOMP0_OUT_RAW_W {
        ACOMP0_OUT_RAW_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn acomp1_out_raw(&mut self) -> ACOMP1_OUT_RAW_W {
        ACOMP1_OUT_RAW_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn acomp0_test_sel(&mut self) -> ACOMP0_TEST_SEL_W {
        ACOMP0_TEST_SEL_W { w: self }
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn acomp1_test_sel(&mut self) -> ACOMP1_TEST_SEL_W {
        ACOMP1_TEST_SEL_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn acomp0_test_en(&mut self) -> ACOMP0_TEST_EN_W {
        ACOMP0_TEST_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn acomp1_test_en(&mut self) -> ACOMP1_TEST_EN_W {
        ACOMP1_TEST_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn acomp0_rstn_ana(&mut self) -> ACOMP0_RSTN_ANA_W {
        ACOMP0_RSTN_ANA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn acomp1_rstn_ana(&mut self) -> ACOMP1_RSTN_ANA_W {
        ACOMP1_RSTN_ANA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "acomp_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acomp_ctrl](index.html) module"]
pub struct ACOMP_CTRL_SPEC;
impl crate::RegisterSpec for ACOMP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acomp_ctrl::R](R) reader structure"]
impl crate::Readable for ACOMP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acomp_ctrl::W](W) writer structure"]
impl crate::Writable for ACOMP_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets acomp_ctrl to value 0"]
impl crate::Resettable for ACOMP_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
