#[doc = "Register `MBIST_CTL` reader"]
pub struct R(crate::R<MBIST_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBIST_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MBIST_CTL_SPEC>> for R {
    fn from(reader: crate::R<MBIST_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBIST_CTL` writer"]
pub struct W(crate::W<MBIST_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBIST_CTL_SPEC>;
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
impl core::convert::From<crate::W<MBIST_CTL_SPEC>> for W {
    fn from(writer: crate::W<MBIST_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_mbist_rst_n` reader - "]
pub struct REG_MBIST_RST_N_R(crate::FieldReader<bool, bool>);
impl REG_MBIST_RST_N_R {
    pub(crate) fn new(bits: bool) -> Self {
        REG_MBIST_RST_N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REG_MBIST_RST_N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reg_mbist_rst_n` writer - "]
pub struct REG_MBIST_RST_N_W<'a> {
    w: &'a mut W,
}
impl<'a> REG_MBIST_RST_N_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `wifi_mbist_mode` reader - "]
pub struct WIFI_MBIST_MODE_R(crate::FieldReader<bool, bool>);
impl WIFI_MBIST_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_MBIST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_MBIST_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wifi_mbist_mode` writer - "]
pub struct WIFI_MBIST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_MBIST_MODE_W<'a> {
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
#[doc = "Field `ocram_mbist_mode` reader - "]
pub struct OCRAM_MBIST_MODE_R(crate::FieldReader<bool, bool>);
impl OCRAM_MBIST_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCRAM_MBIST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCRAM_MBIST_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ocram_mbist_mode` writer - "]
pub struct OCRAM_MBIST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM_MBIST_MODE_W<'a> {
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
#[doc = "Field `tag_mbist_mode` reader - "]
pub struct TAG_MBIST_MODE_R(crate::FieldReader<bool, bool>);
impl TAG_MBIST_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAG_MBIST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAG_MBIST_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tag_mbist_mode` writer - "]
pub struct TAG_MBIST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_MBIST_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `hsram_mbist_mode` reader - "]
pub struct HSRAM_MBIST_MODE_R(crate::FieldReader<bool, bool>);
impl HSRAM_MBIST_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSRAM_MBIST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSRAM_MBIST_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hsram_mbist_mode` writer - "]
pub struct HSRAM_MBIST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAM_MBIST_MODE_W<'a> {
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
#[doc = "Field `irom_mbist_mode` reader - "]
pub struct IROM_MBIST_MODE_R(crate::FieldReader<bool, bool>);
impl IROM_MBIST_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IROM_MBIST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IROM_MBIST_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irom_mbist_mode` writer - "]
pub struct IROM_MBIST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IROM_MBIST_MODE_W<'a> {
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_mbist_rst_n(&self) -> REG_MBIST_RST_N_R {
        REG_MBIST_RST_N_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_mode(&self) -> WIFI_MBIST_MODE_R {
        WIFI_MBIST_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_mode(&self) -> OCRAM_MBIST_MODE_R {
        OCRAM_MBIST_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_mode(&self) -> TAG_MBIST_MODE_R {
        TAG_MBIST_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_mode(&self) -> HSRAM_MBIST_MODE_R {
        HSRAM_MBIST_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_mode(&self) -> IROM_MBIST_MODE_R {
        IROM_MBIST_MODE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reg_mbist_rst_n(&mut self) -> REG_MBIST_RST_N_W {
        REG_MBIST_RST_N_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_mode(&mut self) -> WIFI_MBIST_MODE_W {
        WIFI_MBIST_MODE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_mode(&mut self) -> OCRAM_MBIST_MODE_W {
        OCRAM_MBIST_MODE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_mode(&mut self) -> TAG_MBIST_MODE_W {
        TAG_MBIST_MODE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_mode(&mut self) -> HSRAM_MBIST_MODE_W {
        HSRAM_MBIST_MODE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_mode(&mut self) -> IROM_MBIST_MODE_W {
        IROM_MBIST_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MBIST_CTL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_ctl](index.html) module"]
pub struct MBIST_CTL_SPEC;
impl crate::RegisterSpec for MBIST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbist_ctl::R](R) reader structure"]
impl crate::Readable for MBIST_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbist_ctl::W](W) writer structure"]
impl crate::Writable for MBIST_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MBIST_CTL to value 0"]
impl crate::Resettable for MBIST_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
