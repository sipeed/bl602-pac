#[doc = "Register `WMER` reader"]
pub struct R(crate::R<WMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WMER` writer"]
pub struct W(crate::W<WMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WMER_SPEC>;
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
impl From<crate::W<WMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wrie` reader - "]
pub struct WRIE_R(crate::FieldReader<bool, bool>);
impl WRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wrie` writer - "]
pub struct WRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRIE_W<'a> {
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
#[doc = "Field `we` reader - "]
pub struct WE_R(crate::FieldReader<bool, bool>);
impl WE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `we` writer - "]
pub struct WE_W<'a> {
    w: &'a mut W,
}
impl<'a> WE_W<'a> {
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wrie(&self) -> WRIE_R {
        WRIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn wrie(&mut self) -> WRIE_W {
        WRIE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn we(&mut self) -> WE_W {
        WE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WMER.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wmer](index.html) module"]
pub struct WMER_SPEC;
impl crate::RegisterSpec for WMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wmer::R](R) reader structure"]
impl crate::Readable for WMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wmer::W](W) writer structure"]
impl crate::Writable for WMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WMER to value 0"]
impl crate::Resettable for WMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
