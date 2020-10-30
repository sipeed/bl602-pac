#[doc = "Register `debug` reader"]
pub struct R(crate::R<DEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DEBUG_SPEC>> for R {
    fn from(reader: crate::R<DEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `debug` writer"]
pub struct W(crate::W<DEBUG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_SPEC>;
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
impl core::convert::From<crate::W<DEBUG_SPEC>> for W {
    fn from(writer: crate::W<DEBUG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `debug_i` reader - "]
pub struct DEBUG_I_R(crate::FieldReader<u32, u32>);
impl DEBUG_I_R {
    pub(crate) fn new(bits: u32) -> Self {
        DEBUG_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_I_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `debug_i` writer - "]
pub struct DEBUG_I_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7fff_ffff << 1)) | (((value as u32) & 0x7fff_ffff) << 1);
        self.w
    }
}
#[doc = "Field `debug_oe` reader - "]
pub struct DEBUG_OE_R(crate::FieldReader<bool, bool>);
impl DEBUG_OE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEBUG_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `debug_oe` writer - "]
pub struct DEBUG_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_OE_W<'a> {
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
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn debug_i(&self) -> DEBUG_I_R {
        DEBUG_I_R::new(((self.bits >> 1) & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn debug_oe(&self) -> DEBUG_OE_R {
        DEBUG_OE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn debug_i(&mut self) -> DEBUG_I_W {
        DEBUG_I_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn debug_oe(&mut self) -> DEBUG_OE_W {
        DEBUG_OE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "debug.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug](index.html) module"]
pub struct DEBUG_SPEC;
impl crate::RegisterSpec for DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug::R](R) reader structure"]
impl crate::Readable for DEBUG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug::W](W) writer structure"]
impl crate::Writable for DEBUG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets debug to value 0"]
impl crate::Resettable for DEBUG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
