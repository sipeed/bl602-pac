#[doc = "Register `clkpll_output_en` reader"]
pub struct R(crate::R<CLKPLL_OUTPUT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKPLL_OUTPUT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CLKPLL_OUTPUT_EN_SPEC>> for R {
    fn from(reader: crate::R<CLKPLL_OUTPUT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `clkpll_output_en` writer"]
pub struct W(crate::W<CLKPLL_OUTPUT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKPLL_OUTPUT_EN_SPEC>;
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
impl core::convert::From<crate::W<CLKPLL_OUTPUT_EN_SPEC>> for W {
    fn from(writer: crate::W<CLKPLL_OUTPUT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `clkpll_en_div2_480m` reader - "]
pub struct CLKPLL_EN_DIV2_480M_R(crate::FieldReader<bool, bool>);
impl CLKPLL_EN_DIV2_480M_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_EN_DIV2_480M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_EN_DIV2_480M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_en_div2_480m` writer - "]
pub struct CLKPLL_EN_DIV2_480M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_DIV2_480M_W<'a> {
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
#[doc = "Field `clkpll_en_32m` reader - "]
pub struct CLKPLL_EN_32M_R(crate::FieldReader<bool, bool>);
impl CLKPLL_EN_32M_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_EN_32M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_EN_32M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_en_32m` writer - "]
pub struct CLKPLL_EN_32M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_32M_W<'a> {
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
#[doc = "Field `clkpll_en_48m` reader - "]
pub struct CLKPLL_EN_48M_R(crate::FieldReader<bool, bool>);
impl CLKPLL_EN_48M_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_EN_48M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_EN_48M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_en_48m` writer - "]
pub struct CLKPLL_EN_48M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_48M_W<'a> {
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
#[doc = "Field `clkpll_en_80m` reader - "]
pub struct CLKPLL_EN_80M_R(crate::FieldReader<bool, bool>);
impl CLKPLL_EN_80M_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_EN_80M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_EN_80M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_en_80m` writer - "]
pub struct CLKPLL_EN_80M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_80M_W<'a> {
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
#[doc = "Field `clkpll_en_96m` reader - "]
pub struct CLKPLL_EN_96M_R(crate::FieldReader<bool, bool>);
impl CLKPLL_EN_96M_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_EN_96M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_EN_96M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_en_96m` writer - "]
pub struct CLKPLL_EN_96M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_96M_W<'a> {
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
#[doc = "Field `clkpll_en_120m` reader - "]
pub struct CLKPLL_EN_120M_R(crate::FieldReader<bool, bool>);
impl CLKPLL_EN_120M_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_EN_120M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_EN_120M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_en_120m` writer - "]
pub struct CLKPLL_EN_120M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_120M_W<'a> {
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
#[doc = "Field `clkpll_en_160m` reader - "]
pub struct CLKPLL_EN_160M_R(crate::FieldReader<bool, bool>);
impl CLKPLL_EN_160M_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_EN_160M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_EN_160M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_en_160m` writer - "]
pub struct CLKPLL_EN_160M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_160M_W<'a> {
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
#[doc = "Field `clkpll_en_192m` reader - "]
pub struct CLKPLL_EN_192M_R(crate::FieldReader<bool, bool>);
impl CLKPLL_EN_192M_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_EN_192M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_EN_192M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_en_192m` writer - "]
pub struct CLKPLL_EN_192M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_192M_W<'a> {
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
#[doc = "Field `clkpll_en_240m` reader - "]
pub struct CLKPLL_EN_240M_R(crate::FieldReader<bool, bool>);
impl CLKPLL_EN_240M_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_EN_240M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_EN_240M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_en_240m` writer - "]
pub struct CLKPLL_EN_240M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_240M_W<'a> {
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
#[doc = "Field `clkpll_en_480m` reader - "]
pub struct CLKPLL_EN_480M_R(crate::FieldReader<bool, bool>);
impl CLKPLL_EN_480M_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKPLL_EN_480M_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKPLL_EN_480M_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clkpll_en_480m` writer - "]
pub struct CLKPLL_EN_480M_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKPLL_EN_480M_W<'a> {
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
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_en_div2_480m(&self) -> CLKPLL_EN_DIV2_480M_R {
        CLKPLL_EN_DIV2_480M_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_en_32m(&self) -> CLKPLL_EN_32M_R {
        CLKPLL_EN_32M_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_en_48m(&self) -> CLKPLL_EN_48M_R {
        CLKPLL_EN_48M_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_en_80m(&self) -> CLKPLL_EN_80M_R {
        CLKPLL_EN_80M_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_en_96m(&self) -> CLKPLL_EN_96M_R {
        CLKPLL_EN_96M_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_en_120m(&self) -> CLKPLL_EN_120M_R {
        CLKPLL_EN_120M_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_en_160m(&self) -> CLKPLL_EN_160M_R {
        CLKPLL_EN_160M_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_en_192m(&self) -> CLKPLL_EN_192M_R {
        CLKPLL_EN_192M_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_en_240m(&self) -> CLKPLL_EN_240M_R {
        CLKPLL_EN_240M_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_en_480m(&self) -> CLKPLL_EN_480M_R {
        CLKPLL_EN_480M_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn clkpll_en_div2_480m(&mut self) -> CLKPLL_EN_DIV2_480M_W {
        CLKPLL_EN_DIV2_480M_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clkpll_en_32m(&mut self) -> CLKPLL_EN_32M_W {
        CLKPLL_EN_32M_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn clkpll_en_48m(&mut self) -> CLKPLL_EN_48M_W {
        CLKPLL_EN_48M_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn clkpll_en_80m(&mut self) -> CLKPLL_EN_80M_W {
        CLKPLL_EN_80M_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn clkpll_en_96m(&mut self) -> CLKPLL_EN_96M_W {
        CLKPLL_EN_96M_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn clkpll_en_120m(&mut self) -> CLKPLL_EN_120M_W {
        CLKPLL_EN_120M_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn clkpll_en_160m(&mut self) -> CLKPLL_EN_160M_W {
        CLKPLL_EN_160M_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn clkpll_en_192m(&mut self) -> CLKPLL_EN_192M_W {
        CLKPLL_EN_192M_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn clkpll_en_240m(&mut self) -> CLKPLL_EN_240M_W {
        CLKPLL_EN_240M_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn clkpll_en_480m(&mut self) -> CLKPLL_EN_480M_W {
        CLKPLL_EN_480M_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "clkpll_output_en.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkpll_output_en](index.html) module"]
pub struct CLKPLL_OUTPUT_EN_SPEC;
impl crate::RegisterSpec for CLKPLL_OUTPUT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkpll_output_en::R](R) reader structure"]
impl crate::Readable for CLKPLL_OUTPUT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkpll_output_en::W](W) writer structure"]
impl crate::Writable for CLKPLL_OUTPUT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets clkpll_output_en to value 0x0100"]
impl crate::Resettable for CLKPLL_OUTPUT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
