#[doc = "Register `sf_ctrl_2` reader"]
pub struct R(crate::R<SF_CTRL_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SF_CTRL_2_SPEC>> for R {
    fn from(reader: crate::R<SF_CTRL_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_2` writer"]
pub struct W(crate::W<SF_CTRL_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_2_SPEC>;
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
impl core::convert::From<crate::W<SF_CTRL_2_SPEC>> for W {
    fn from(writer: crate::W<SF_CTRL_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_if_dqs_en` reader - "]
pub struct SF_IF_DQS_EN_R(crate::FieldReader<bool, bool>);
impl SF_IF_DQS_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_DQS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_DQS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_dqs_en` writer - "]
pub struct SF_IF_DQS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_DQS_EN_W<'a> {
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
#[doc = "Field `sf_if_dtr_en` reader - "]
pub struct SF_IF_DTR_EN_R(crate::FieldReader<bool, bool>);
impl SF_IF_DTR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_DTR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_DTR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_dtr_en` writer - "]
pub struct SF_IF_DTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_DTR_EN_W<'a> {
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
#[doc = "Field `sf_if_pad_sel_lock` reader - "]
pub struct SF_IF_PAD_SEL_LOCK_R(crate::FieldReader<bool, bool>);
impl SF_IF_PAD_SEL_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_IF_PAD_SEL_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_PAD_SEL_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_pad_sel_lock` writer - "]
pub struct SF_IF_PAD_SEL_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_PAD_SEL_LOCK_W<'a> {
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
#[doc = "Field `sf_if_pad_sel` reader - "]
pub struct SF_IF_PAD_SEL_R(crate::FieldReader<u8, u8>);
impl SF_IF_PAD_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IF_PAD_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IF_PAD_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_if_pad_sel` writer - "]
pub struct SF_IF_PAD_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IF_PAD_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_if_dqs_en(&self) -> SF_IF_DQS_EN_R {
        SF_IF_DQS_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_if_dtr_en(&self) -> SF_IF_DTR_EN_R {
        SF_IF_DTR_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_if_pad_sel_lock(&self) -> SF_IF_PAD_SEL_LOCK_R {
        SF_IF_PAD_SEL_LOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_if_pad_sel(&self) -> SF_IF_PAD_SEL_R {
        SF_IF_PAD_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sf_if_dqs_en(&mut self) -> SF_IF_DQS_EN_W {
        SF_IF_DQS_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sf_if_dtr_en(&mut self) -> SF_IF_DTR_EN_W {
        SF_IF_DTR_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sf_if_pad_sel_lock(&mut self) -> SF_IF_PAD_SEL_LOCK_W {
        SF_IF_PAD_SEL_LOCK_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_if_pad_sel(&mut self) -> SF_IF_PAD_SEL_W {
        SF_IF_PAD_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_2](index.html) module"]
pub struct SF_CTRL_2_SPEC;
impl crate::RegisterSpec for SF_CTRL_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_2::R](R) reader structure"]
impl crate::Readable for SF_CTRL_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_2::W](W) writer structure"]
impl crate::Writable for SF_CTRL_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_ctrl_2 to value 0"]
impl crate::Resettable for SF_CTRL_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
