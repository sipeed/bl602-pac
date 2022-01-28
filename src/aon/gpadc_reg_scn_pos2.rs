#[doc = "Register `gpadc_reg_scn_pos2` reader"]
pub struct R(crate::R<GPADC_REG_SCN_POS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_REG_SCN_POS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPADC_REG_SCN_POS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPADC_REG_SCN_POS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_reg_scn_pos2` writer"]
pub struct W(crate::W<GPADC_REG_SCN_POS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_REG_SCN_POS2_SPEC>;
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
impl From<crate::W<GPADC_REG_SCN_POS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPADC_REG_SCN_POS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpadc_scan_pos_11` reader - "]
pub struct GPADC_SCAN_POS_11_R(crate::FieldReader<u8, u8>);
impl GPADC_SCAN_POS_11_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_SCAN_POS_11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_SCAN_POS_11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_scan_pos_11` writer - "]
pub struct GPADC_SCAN_POS_11_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
#[doc = "Field `gpadc_scan_pos_10` reader - "]
pub struct GPADC_SCAN_POS_10_R(crate::FieldReader<u8, u8>);
impl GPADC_SCAN_POS_10_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_SCAN_POS_10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_SCAN_POS_10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_scan_pos_10` writer - "]
pub struct GPADC_SCAN_POS_10_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `gpadc_scan_pos_9` reader - "]
pub struct GPADC_SCAN_POS_9_R(crate::FieldReader<u8, u8>);
impl GPADC_SCAN_POS_9_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_SCAN_POS_9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_SCAN_POS_9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_scan_pos_9` writer - "]
pub struct GPADC_SCAN_POS_9_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `gpadc_scan_pos_8` reader - "]
pub struct GPADC_SCAN_POS_8_R(crate::FieldReader<u8, u8>);
impl GPADC_SCAN_POS_8_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_SCAN_POS_8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_SCAN_POS_8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_scan_pos_8` writer - "]
pub struct GPADC_SCAN_POS_8_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `gpadc_scan_pos_7` reader - "]
pub struct GPADC_SCAN_POS_7_R(crate::FieldReader<u8, u8>);
impl GPADC_SCAN_POS_7_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_SCAN_POS_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_SCAN_POS_7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_scan_pos_7` writer - "]
pub struct GPADC_SCAN_POS_7_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `gpadc_scan_pos_6` reader - "]
pub struct GPADC_SCAN_POS_6_R(crate::FieldReader<u8, u8>);
impl GPADC_SCAN_POS_6_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_SCAN_POS_6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_SCAN_POS_6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_scan_pos_6` writer - "]
pub struct GPADC_SCAN_POS_6_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_SCAN_POS_6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_pos_11(&self) -> GPADC_SCAN_POS_11_R {
        GPADC_SCAN_POS_11_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_pos_10(&self) -> GPADC_SCAN_POS_10_R {
        GPADC_SCAN_POS_10_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_pos_9(&self) -> GPADC_SCAN_POS_9_R {
        GPADC_SCAN_POS_9_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_pos_8(&self) -> GPADC_SCAN_POS_8_R {
        GPADC_SCAN_POS_8_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_pos_7(&self) -> GPADC_SCAN_POS_7_R {
        GPADC_SCAN_POS_7_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_pos_6(&self) -> GPADC_SCAN_POS_6_R {
        GPADC_SCAN_POS_6_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29"]
    #[inline(always)]
    pub fn gpadc_scan_pos_11(&mut self) -> GPADC_SCAN_POS_11_W {
        GPADC_SCAN_POS_11_W { w: self }
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn gpadc_scan_pos_10(&mut self) -> GPADC_SCAN_POS_10_W {
        GPADC_SCAN_POS_10_W { w: self }
    }
    #[doc = "Bits 15:19"]
    #[inline(always)]
    pub fn gpadc_scan_pos_9(&mut self) -> GPADC_SCAN_POS_9_W {
        GPADC_SCAN_POS_9_W { w: self }
    }
    #[doc = "Bits 10:14"]
    #[inline(always)]
    pub fn gpadc_scan_pos_8(&mut self) -> GPADC_SCAN_POS_8_W {
        GPADC_SCAN_POS_8_W { w: self }
    }
    #[doc = "Bits 5:9"]
    #[inline(always)]
    pub fn gpadc_scan_pos_7(&mut self) -> GPADC_SCAN_POS_7_W {
        GPADC_SCAN_POS_7_W { w: self }
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn gpadc_scan_pos_6(&mut self) -> GPADC_SCAN_POS_6_W {
        GPADC_SCAN_POS_6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "adc converation sequence 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_reg_scn_pos2](index.html) module"]
pub struct GPADC_REG_SCN_POS2_SPEC;
impl crate::RegisterSpec for GPADC_REG_SCN_POS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_reg_scn_pos2::R](R) reader structure"]
impl crate::Readable for GPADC_REG_SCN_POS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_reg_scn_pos2::W](W) writer structure"]
impl crate::Writable for GPADC_REG_SCN_POS2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_reg_scn_pos2 to value 0x1ef7_bdef"]
impl crate::Resettable for GPADC_REG_SCN_POS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1ef7_bdef
    }
}
