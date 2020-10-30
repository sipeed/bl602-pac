#[doc = "Register `MBIST_STAT` reader"]
pub struct R(crate::R<MBIST_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MBIST_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<MBIST_STAT_SPEC>> for R {
    fn from(reader: crate::R<MBIST_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MBIST_STAT` writer"]
pub struct W(crate::W<MBIST_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MBIST_STAT_SPEC>;
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
impl core::convert::From<crate::W<MBIST_STAT_SPEC>> for W {
    fn from(writer: crate::W<MBIST_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wifi_mbist_fail` reader - "]
pub struct WIFI_MBIST_FAIL_R(crate::FieldReader<bool, bool>);
impl WIFI_MBIST_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_MBIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_MBIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wifi_mbist_fail` writer - "]
pub struct WIFI_MBIST_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_MBIST_FAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `ocram_mbist_fail` reader - "]
pub struct OCRAM_MBIST_FAIL_R(crate::FieldReader<bool, bool>);
impl OCRAM_MBIST_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCRAM_MBIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCRAM_MBIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ocram_mbist_fail` writer - "]
pub struct OCRAM_MBIST_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM_MBIST_FAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `tag_mbist_fail` reader - "]
pub struct TAG_MBIST_FAIL_R(crate::FieldReader<bool, bool>);
impl TAG_MBIST_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAG_MBIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAG_MBIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tag_mbist_fail` writer - "]
pub struct TAG_MBIST_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_MBIST_FAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `hsram_mbist_fail` reader - "]
pub struct HSRAM_MBIST_FAIL_R(crate::FieldReader<bool, bool>);
impl HSRAM_MBIST_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSRAM_MBIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSRAM_MBIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hsram_mbist_fail` writer - "]
pub struct HSRAM_MBIST_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAM_MBIST_FAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `irom_mbist_fail` reader - "]
pub struct IROM_MBIST_FAIL_R(crate::FieldReader<bool, bool>);
impl IROM_MBIST_FAIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        IROM_MBIST_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IROM_MBIST_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irom_mbist_fail` writer - "]
pub struct IROM_MBIST_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> IROM_MBIST_FAIL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `wifi_mbist_done` reader - "]
pub struct WIFI_MBIST_DONE_R(crate::FieldReader<bool, bool>);
impl WIFI_MBIST_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIFI_MBIST_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIFI_MBIST_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wifi_mbist_done` writer - "]
pub struct WIFI_MBIST_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> WIFI_MBIST_DONE_W<'a> {
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
#[doc = "Field `ocram_mbist_done` reader - "]
pub struct OCRAM_MBIST_DONE_R(crate::FieldReader<bool, bool>);
impl OCRAM_MBIST_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OCRAM_MBIST_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OCRAM_MBIST_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ocram_mbist_done` writer - "]
pub struct OCRAM_MBIST_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> OCRAM_MBIST_DONE_W<'a> {
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
#[doc = "Field `tag_mbist_done` reader - "]
pub struct TAG_MBIST_DONE_R(crate::FieldReader<bool, bool>);
impl TAG_MBIST_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAG_MBIST_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAG_MBIST_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tag_mbist_done` writer - "]
pub struct TAG_MBIST_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_MBIST_DONE_W<'a> {
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
#[doc = "Field `hsram_mbist_done` reader - "]
pub struct HSRAM_MBIST_DONE_R(crate::FieldReader<bool, bool>);
impl HSRAM_MBIST_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSRAM_MBIST_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSRAM_MBIST_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `hsram_mbist_done` writer - "]
pub struct HSRAM_MBIST_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSRAM_MBIST_DONE_W<'a> {
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
#[doc = "Field `irom_mbist_done` reader - "]
pub struct IROM_MBIST_DONE_R(crate::FieldReader<bool, bool>);
impl IROM_MBIST_DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IROM_MBIST_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IROM_MBIST_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `irom_mbist_done` writer - "]
pub struct IROM_MBIST_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> IROM_MBIST_DONE_W<'a> {
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
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_mbist_fail(&self) -> WIFI_MBIST_FAIL_R {
        WIFI_MBIST_FAIL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&self) -> OCRAM_MBIST_FAIL_R {
        OCRAM_MBIST_FAIL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tag_mbist_fail(&self) -> TAG_MBIST_FAIL_R {
        TAG_MBIST_FAIL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hsram_mbist_fail(&self) -> HSRAM_MBIST_FAIL_R {
        HSRAM_MBIST_FAIL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irom_mbist_fail(&self) -> IROM_MBIST_FAIL_R {
        IROM_MBIST_FAIL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_done(&self) -> WIFI_MBIST_DONE_R {
        WIFI_MBIST_DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_done(&self) -> OCRAM_MBIST_DONE_R {
        OCRAM_MBIST_DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_done(&self) -> TAG_MBIST_DONE_R {
        TAG_MBIST_DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_done(&self) -> HSRAM_MBIST_DONE_R {
        HSRAM_MBIST_DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_done(&self) -> IROM_MBIST_DONE_R {
        IROM_MBIST_DONE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wifi_mbist_fail(&mut self) -> WIFI_MBIST_FAIL_W {
        WIFI_MBIST_FAIL_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ocram_mbist_fail(&mut self) -> OCRAM_MBIST_FAIL_W {
        OCRAM_MBIST_FAIL_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tag_mbist_fail(&mut self) -> TAG_MBIST_FAIL_W {
        TAG_MBIST_FAIL_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn hsram_mbist_fail(&mut self) -> HSRAM_MBIST_FAIL_W {
        HSRAM_MBIST_FAIL_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn irom_mbist_fail(&mut self) -> IROM_MBIST_FAIL_W {
        IROM_MBIST_FAIL_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wifi_mbist_done(&mut self) -> WIFI_MBIST_DONE_W {
        WIFI_MBIST_DONE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ocram_mbist_done(&mut self) -> OCRAM_MBIST_DONE_W {
        OCRAM_MBIST_DONE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tag_mbist_done(&mut self) -> TAG_MBIST_DONE_W {
        TAG_MBIST_DONE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn hsram_mbist_done(&mut self) -> HSRAM_MBIST_DONE_W {
        HSRAM_MBIST_DONE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn irom_mbist_done(&mut self) -> IROM_MBIST_DONE_W {
        IROM_MBIST_DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MBIST_STAT.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mbist_stat](index.html) module"]
pub struct MBIST_STAT_SPEC;
impl crate::RegisterSpec for MBIST_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mbist_stat::R](R) reader structure"]
impl crate::Readable for MBIST_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mbist_stat::W](W) writer structure"]
impl crate::Writable for MBIST_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MBIST_STAT to value 0"]
impl crate::Resettable for MBIST_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
