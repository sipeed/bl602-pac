#[doc = "Register `singen_ctrl2` reader"]
pub struct R(crate::R<SINGEN_CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SINGEN_CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SINGEN_CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SINGEN_CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `singen_ctrl2` writer"]
pub struct W(crate::W<SINGEN_CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SINGEN_CTRL2_SPEC>;
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
impl From<crate::W<SINGEN_CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SINGEN_CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `singen_start_addr0_i` reader - "]
pub struct SINGEN_START_ADDR0_I_R(crate::FieldReader<u16, u16>);
impl SINGEN_START_ADDR0_I_R {
    pub(crate) fn new(bits: u16) -> Self {
        SINGEN_START_ADDR0_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_START_ADDR0_I_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_start_addr0_i` writer - "]
pub struct SINGEN_START_ADDR0_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_START_ADDR0_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 22)) | ((value as u32 & 0x03ff) << 22);
        self.w
    }
}
#[doc = "Field `singen_start_addr1_i` reader - "]
pub struct SINGEN_START_ADDR1_I_R(crate::FieldReader<u16, u16>);
impl SINGEN_START_ADDR1_I_R {
    pub(crate) fn new(bits: u16) -> Self {
        SINGEN_START_ADDR1_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_START_ADDR1_I_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_start_addr1_i` writer - "]
pub struct SINGEN_START_ADDR1_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_START_ADDR1_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 12)) | ((value as u32 & 0x03ff) << 12);
        self.w
    }
}
#[doc = "Field `singen_gain_i` reader - "]
pub struct SINGEN_GAIN_I_R(crate::FieldReader<u16, u16>);
impl SINGEN_GAIN_I_R {
    pub(crate) fn new(bits: u16) -> Self {
        SINGEN_GAIN_I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGEN_GAIN_I_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `singen_gain_i` writer - "]
pub struct SINGEN_GAIN_I_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGEN_GAIN_I_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn singen_start_addr0_i(&self) -> SINGEN_START_ADDR0_I_R {
        SINGEN_START_ADDR0_I_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn singen_start_addr1_i(&self) -> SINGEN_START_ADDR1_I_R {
        SINGEN_START_ADDR1_I_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn singen_gain_i(&self) -> SINGEN_GAIN_I_R {
        SINGEN_GAIN_I_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn singen_start_addr0_i(&mut self) -> SINGEN_START_ADDR0_I_W {
        SINGEN_START_ADDR0_I_W { w: self }
    }
    #[doc = "Bits 12:21"]
    #[inline(always)]
    pub fn singen_start_addr1_i(&mut self) -> SINGEN_START_ADDR1_I_W {
        SINGEN_START_ADDR1_I_W { w: self }
    }
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn singen_gain_i(&mut self) -> SINGEN_GAIN_I_W {
        SINGEN_GAIN_I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "singen_ctrl2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [singen_ctrl2](index.html) module"]
pub struct SINGEN_CTRL2_SPEC;
impl crate::RegisterSpec for SINGEN_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [singen_ctrl2::R](R) reader structure"]
impl crate::Readable for SINGEN_CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [singen_ctrl2::W](W) writer structure"]
impl crate::Writable for SINGEN_CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets singen_ctrl2 to value 0"]
impl crate::Resettable for SINGEN_CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
