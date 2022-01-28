#[doc = "Register `TIER2` reader"]
pub struct R(crate::R<TIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIER2` writer"]
pub struct W(crate::W<TIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIER2_SPEC>;
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
impl From<crate::W<TIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tier_2` reader - "]
pub struct TIER_2_R(crate::FieldReader<bool, bool>);
impl TIER_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIER_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIER_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tier_2` writer - "]
pub struct TIER_2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIER_2_W<'a> {
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
#[doc = "Field `tier_1` reader - "]
pub struct TIER_1_R(crate::FieldReader<bool, bool>);
impl TIER_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIER_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIER_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tier_1` writer - "]
pub struct TIER_1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIER_1_W<'a> {
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
#[doc = "Field `tier_0` reader - "]
pub struct TIER_0_R(crate::FieldReader<bool, bool>);
impl TIER_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIER_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIER_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tier_0` writer - "]
pub struct TIER_0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIER_0_W<'a> {
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
    pub fn tier_2(&self) -> TIER_2_R {
        TIER_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tier_1(&self) -> TIER_1_R {
        TIER_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tier_0(&self) -> TIER_0_R {
        TIER_0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tier_2(&mut self) -> TIER_2_W {
        TIER_2_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tier_1(&mut self) -> TIER_1_W {
        TIER_1_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tier_0(&mut self) -> TIER_0_W {
        TIER_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIER2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tier2](index.html) module"]
pub struct TIER2_SPEC;
impl crate::RegisterSpec for TIER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tier2::R](R) reader structure"]
impl crate::Readable for TIER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tier2::W](W) writer structure"]
impl crate::Writable for TIER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIER2 to value 0"]
impl crate::Resettable for TIER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
