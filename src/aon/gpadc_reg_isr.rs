#[doc = "Register `gpadc_reg_isr` reader"]
pub struct R(crate::R<GPADC_REG_ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_isr` writer"]
pub struct W(crate::W<GPADC_REG_ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_ISR_SPEC>;
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
impl From<crate::W<GPADC_REG_ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_REG_ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_pos_satur_mask` reader - "]
pub struct GPADC_POS_SATUR_MASK_R(crate::FieldReader<bool, bool>);
impl GPADC_POS_SATUR_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_POS_SATUR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_POS_SATUR_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_pos_satur_mask` writer - "]
pub struct GPADC_POS_SATUR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_POS_SATUR_MASK_W<'a> {
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
#[doc = "Field `gpadc_neg_satur_mask` reader - "]
pub struct GPADC_NEG_SATUR_MASK_R(crate::FieldReader<bool, bool>);
impl GPADC_NEG_SATUR_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_NEG_SATUR_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_NEG_SATUR_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_neg_satur_mask` writer - "]
pub struct GPADC_NEG_SATUR_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_NEG_SATUR_MASK_W<'a> {
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
#[doc = "Field `gpadc_pos_satur_clr` reader - "]
pub struct GPADC_POS_SATUR_CLR_R(crate::FieldReader<bool, bool>);
impl GPADC_POS_SATUR_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_POS_SATUR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_POS_SATUR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_pos_satur_clr` writer - "]
pub struct GPADC_POS_SATUR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_POS_SATUR_CLR_W<'a> {
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
#[doc = "Field `gpadc_neg_satur_clr` reader - "]
pub struct GPADC_NEG_SATUR_CLR_R(crate::FieldReader<bool, bool>);
impl GPADC_NEG_SATUR_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_NEG_SATUR_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_NEG_SATUR_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_neg_satur_clr` writer - "]
pub struct GPADC_NEG_SATUR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_NEG_SATUR_CLR_W<'a> {
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
#[doc = "Field `gpadc_pos_satur` reader - "]
pub struct GPADC_POS_SATUR_R(crate::FieldReader<bool, bool>);
impl GPADC_POS_SATUR_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_POS_SATUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_POS_SATUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_neg_satur` reader - "]
pub struct GPADC_NEG_SATUR_R(crate::FieldReader<bool, bool>);
impl GPADC_NEG_SATUR_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_NEG_SATUR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_NEG_SATUR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_pos_satur_mask(&self) -> GPADC_POS_SATUR_MASK_R {
        GPADC_POS_SATUR_MASK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_neg_satur_mask(&self) -> GPADC_NEG_SATUR_MASK_R {
        GPADC_NEG_SATUR_MASK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_pos_satur_clr(&self) -> GPADC_POS_SATUR_CLR_R {
        GPADC_POS_SATUR_CLR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_neg_satur_clr(&self) -> GPADC_NEG_SATUR_CLR_R {
        GPADC_NEG_SATUR_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_pos_satur(&self) -> GPADC_POS_SATUR_R {
        GPADC_POS_SATUR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_neg_satur(&self) -> GPADC_NEG_SATUR_R {
        GPADC_NEG_SATUR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_pos_satur_mask(&mut self) -> GPADC_POS_SATUR_MASK_W {
        GPADC_POS_SATUR_MASK_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_neg_satur_mask(&mut self) -> GPADC_NEG_SATUR_MASK_W {
        GPADC_NEG_SATUR_MASK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_pos_satur_clr(&mut self) -> GPADC_POS_SATUR_CLR_W {
        GPADC_POS_SATUR_CLR_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_neg_satur_clr(&mut self) -> GPADC_NEG_SATUR_CLR_W {
        GPADC_NEG_SATUR_CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_reg_isr.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_isr](index.html) module"]
pub struct GPADC_REG_ISR_SPEC;
impl crate::RegisterSpec for GPADC_REG_ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_isr::R](R) reader structure"]
impl crate::Readable for GPADC_REG_ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_isr::W](W) writer structure"]
impl crate::Writable for GPADC_REG_ISR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_reg_isr to value 0"]
impl crate::Resettable for GPADC_REG_ISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
