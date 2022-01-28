#[doc = "Register `TCCR` reader"]
pub struct R(crate::R<TCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR` writer"]
pub struct W(crate::W<TCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR_SPEC>;
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
impl From<crate::W<TCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cs_wdt` reader - "]
pub struct CS_WDT_R(crate::FieldReader<u8, u8>);
impl CS_WDT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS_WDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_WDT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cs_wdt` writer - "]
pub struct CS_WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_WDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `RESERVED_7` reader - "]
pub struct RESERVED_7_R(crate::FieldReader<bool, bool>);
impl RESERVED_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED_7` writer - "]
pub struct RESERVED_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_7_W<'a> {
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
#[doc = "Field `cs_2` reader - "]
pub struct CS_2_R(crate::FieldReader<u8, u8>);
impl CS_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cs_2` writer - "]
pub struct CS_2_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Field `RESERVED_4` reader - "]
pub struct RESERVED_4_R(crate::FieldReader<bool, bool>);
impl RESERVED_4_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESERVED_4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED_4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED_4` writer - "]
pub struct RESERVED_4_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_4_W<'a> {
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
#[doc = "Field `cs_1` reader - "]
pub struct CS_1_R(crate::FieldReader<u8, u8>);
impl CS_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        CS_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cs_1` writer - "]
pub struct CS_1_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cs_wdt(&self) -> CS_WDT_R {
        CS_WDT_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&self) -> RESERVED_7_R {
        RESERVED_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cs_2(&self) -> CS_2_R {
        CS_2_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reserved_4(&self) -> RESERVED_4_R {
        RESERVED_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cs_1(&self) -> CS_1_R {
        CS_1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn cs_wdt(&mut self) -> CS_WDT_W {
        CS_WDT_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reserved_7(&mut self) -> RESERVED_7_W {
        RESERVED_7_W { w: self }
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cs_2(&mut self) -> CS_2_W {
        CS_2_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reserved_4(&mut self) -> RESERVED_4_W {
        RESERVED_4_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cs_1(&mut self) -> CS_1_W {
        CS_1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCCR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr](index.html) module"]
pub struct TCCR_SPEC;
impl crate::RegisterSpec for TCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tccr::R](R) reader structure"]
impl crate::Readable for TCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr::W](W) writer structure"]
impl crate::Writable for TCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TCCR to value 0"]
impl crate::Resettable for TCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
