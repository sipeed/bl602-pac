#[doc = "Register `lo` reader"]
pub struct R(crate::R<LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `lo` writer"]
pub struct W(crate::W<LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LO_SPEC>;
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
impl From<crate::W<LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `lo_slipped_up` reader - "]
pub struct LO_SLIPPED_UP_R(crate::FieldReader<bool, bool>);
impl LO_SLIPPED_UP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_SLIPPED_UP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SLIPPED_UP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_slipped_up` writer - "]
pub struct LO_SLIPPED_UP_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SLIPPED_UP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `lo_slipped_dn` reader - "]
pub struct LO_SLIPPED_DN_R(crate::FieldReader<bool, bool>);
impl LO_SLIPPED_DN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_SLIPPED_DN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_SLIPPED_DN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_slipped_dn` writer - "]
pub struct LO_SLIPPED_DN_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_SLIPPED_DN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `lo_lf_r4_short` reader - "]
pub struct LO_LF_R4_SHORT_R(crate::FieldReader<bool, bool>);
impl LO_LF_R4_SHORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        LO_LF_R4_SHORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_R4_SHORT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_r4_short` writer - "]
pub struct LO_LF_R4_SHORT_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_R4_SHORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `lo_lf_r4` reader - "]
pub struct LO_LF_R4_R(crate::FieldReader<u8, u8>);
impl LO_LF_R4_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_R4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_R4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_r4` writer - "]
pub struct LO_LF_R4_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_R4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `lo_lf_cz` reader - "]
pub struct LO_LF_CZ_R(crate::FieldReader<u8, u8>);
impl LO_LF_CZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_CZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_CZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_cz` writer - "]
pub struct LO_LF_CZ_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_CZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "Field `lo_lf_rz` reader - "]
pub struct LO_LF_RZ_R(crate::FieldReader<u8, u8>);
impl LO_LF_RZ_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_RZ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_RZ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_rz` writer - "]
pub struct LO_LF_RZ_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_RZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `lo_lf_cz_hw` reader - "]
pub struct LO_LF_CZ_HW_R(crate::FieldReader<u8, u8>);
impl LO_LF_CZ_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_CZ_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_CZ_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_cz_hw` writer - "]
pub struct LO_LF_CZ_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_CZ_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `lo_lf_r4_hw` reader - "]
pub struct LO_LF_R4_HW_R(crate::FieldReader<u8, u8>);
impl LO_LF_R4_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_R4_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_R4_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_r4_hw` writer - "]
pub struct LO_LF_R4_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_R4_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `lo_lf_rz_hw` reader - "]
pub struct LO_LF_RZ_HW_R(crate::FieldReader<u8, u8>);
impl LO_LF_RZ_HW_R {
    pub(crate) fn new(bits: u8) -> Self {
        LO_LF_RZ_HW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LO_LF_RZ_HW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `lo_lf_rz_hw` writer - "]
pub struct LO_LF_RZ_HW_W<'a> {
    w: &'a mut W,
}
impl<'a> LO_LF_RZ_HW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_slipped_up(&self) -> LO_SLIPPED_UP_R {
        LO_SLIPPED_UP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_slipped_dn(&self) -> LO_SLIPPED_DN_R {
        LO_SLIPPED_DN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lo_lf_r4_short(&self) -> LO_LF_R4_SHORT_R {
        LO_LF_R4_SHORT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_r4(&self) -> LO_LF_R4_R {
        LO_LF_R4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_lf_cz(&self) -> LO_LF_CZ_R {
        LO_LF_CZ_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz(&self) -> LO_LF_RZ_R {
        LO_LF_RZ_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_hw(&self) -> LO_LF_CZ_HW_R {
        LO_LF_CZ_HW_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_r4_hw(&self) -> LO_LF_R4_HW_R {
        LO_LF_R4_HW_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_lf_rz_hw(&self) -> LO_LF_RZ_HW_R {
        LO_LF_RZ_HW_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn lo_slipped_up(&mut self) -> LO_SLIPPED_UP_W {
        LO_SLIPPED_UP_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn lo_slipped_dn(&mut self) -> LO_SLIPPED_DN_W {
        LO_SLIPPED_DN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn lo_lf_r4_short(&mut self) -> LO_LF_R4_SHORT_W {
        LO_LF_R4_SHORT_W { w: self }
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn lo_lf_r4(&mut self) -> LO_LF_R4_W {
        LO_LF_R4_W { w: self }
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn lo_lf_cz(&mut self) -> LO_LF_CZ_W {
        LO_LF_CZ_W { w: self }
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn lo_lf_rz(&mut self) -> LO_LF_RZ_W {
        LO_LF_RZ_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lo_lf_cz_hw(&mut self) -> LO_LF_CZ_HW_W {
        LO_LF_CZ_HW_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn lo_lf_r4_hw(&mut self) -> LO_LF_R4_HW_W {
        LO_LF_R4_HW_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn lo_lf_rz_hw(&mut self) -> LO_LF_RZ_HW_W {
        LO_LF_RZ_HW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "lo.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lo](index.html) module"]
pub struct LO_SPEC;
impl crate::RegisterSpec for LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lo::R](R) reader structure"]
impl crate::Readable for LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lo::W](W) writer structure"]
impl crate::Writable for LO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets lo to value 0"]
impl crate::Resettable for LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
