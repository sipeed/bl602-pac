#[doc = "Register `cci_ctl` reader"]
pub struct R(crate::R<CCI_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCI_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CCI_CTL_SPEC>> for R {
    fn from(reader: crate::R<CCI_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cci_ctl` writer"]
pub struct W(crate::W<CCI_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCI_CTL_SPEC>;
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
impl core::convert::From<crate::W<CCI_CTL_SPEC>> for W {
    fn from(writer: crate::W<CCI_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ahb_state` reader - "]
pub struct AHB_STATE_R(crate::FieldReader<u8, u8>);
impl AHB_STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        AHB_STATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHB_STATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ahb_state` writer - "]
pub struct AHB_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `cci_read_flag` reader - "]
pub struct CCI_READ_FLAG_R(crate::FieldReader<bool, bool>);
impl CCI_READ_FLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCI_READ_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCI_READ_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cci_read_flag` writer - "]
pub struct CCI_READ_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_READ_FLAG_W<'a> {
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
#[doc = "Field `cci_write_flag` reader - "]
pub struct CCI_WRITE_FLAG_R(crate::FieldReader<bool, bool>);
impl CCI_WRITE_FLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CCI_WRITE_FLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCI_WRITE_FLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cci_write_flag` writer - "]
pub struct CCI_WRITE_FLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> CCI_WRITE_FLAG_W<'a> {
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
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ahb_state(&self) -> AHB_STATE_R {
        AHB_STATE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_read_flag(&self) -> CCI_READ_FLAG_R {
        CCI_READ_FLAG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_write_flag(&self) -> CCI_WRITE_FLAG_R {
        CCI_WRITE_FLAG_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ahb_state(&mut self) -> AHB_STATE_W {
        AHB_STATE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cci_read_flag(&mut self) -> CCI_READ_FLAG_W {
        CCI_READ_FLAG_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cci_write_flag(&mut self) -> CCI_WRITE_FLAG_W {
        CCI_WRITE_FLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "cci_ctl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cci_ctl](index.html) module"]
pub struct CCI_CTL_SPEC;
impl crate::RegisterSpec for CCI_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cci_ctl::R](R) reader structure"]
impl crate::Readable for CCI_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cci_ctl::W](W) writer structure"]
impl crate::Writable for CCI_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cci_ctl to value 0"]
impl crate::Resettable for CCI_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
