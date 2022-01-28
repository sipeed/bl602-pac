#[doc = "Register `sf_ctrl_prot_en` reader"]
pub struct R(crate::R<SF_CTRL_PROT_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_CTRL_PROT_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_CTRL_PROT_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_CTRL_PROT_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_ctrl_prot_en` writer"]
pub struct W(crate::W<SF_CTRL_PROT_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_CTRL_PROT_EN_SPEC>;
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
impl From<crate::W<SF_CTRL_PROT_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_CTRL_PROT_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_ctrl_id1_en` reader - "]
pub struct SF_CTRL_ID1_EN_R(crate::FieldReader<bool, bool>);
impl SF_CTRL_ID1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CTRL_ID1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CTRL_ID1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_ctrl_id1_en` writer - "]
pub struct SF_CTRL_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_ID1_EN_W<'a> {
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
#[doc = "Field `sf_ctrl_id0_en` reader - "]
pub struct SF_CTRL_ID0_EN_R(crate::FieldReader<bool, bool>);
impl SF_CTRL_ID0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CTRL_ID0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CTRL_ID0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_ctrl_id0_en` writer - "]
pub struct SF_CTRL_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_ID0_EN_W<'a> {
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
#[doc = "Field `sf_ctrl_prot_en` reader - "]
pub struct SF_CTRL_PROT_EN_R(crate::FieldReader<bool, bool>);
impl SF_CTRL_PROT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SF_CTRL_PROT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_CTRL_PROT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_ctrl_prot_en` writer - "]
pub struct SF_CTRL_PROT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_CTRL_PROT_EN_W<'a> {
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
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en(&self) -> SF_CTRL_ID1_EN_R {
        SF_CTRL_ID1_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en(&self) -> SF_CTRL_ID0_EN_R {
        SF_CTRL_ID0_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en(&self) -> SF_CTRL_PROT_EN_R {
        SF_CTRL_PROT_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sf_ctrl_id1_en(&mut self) -> SF_CTRL_ID1_EN_W {
        SF_CTRL_ID1_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sf_ctrl_id0_en(&mut self) -> SF_CTRL_ID0_EN_W {
        SF_CTRL_ID0_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sf_ctrl_prot_en(&mut self) -> SF_CTRL_PROT_EN_W {
        SF_CTRL_PROT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_ctrl_prot_en.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_ctrl_prot_en](index.html) module"]
pub struct SF_CTRL_PROT_EN_SPEC;
impl crate::RegisterSpec for SF_CTRL_PROT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_ctrl_prot_en::R](R) reader structure"]
impl crate::Readable for SF_CTRL_PROT_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_ctrl_prot_en::W](W) writer structure"]
impl crate::Writable for SF_CTRL_PROT_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_ctrl_prot_en to value 0x07"]
impl crate::Resettable for SF_CTRL_PROT_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
