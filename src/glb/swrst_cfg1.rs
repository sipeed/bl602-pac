#[doc = "Register `swrst_cfg1` reader"]
pub struct R(crate::R<SWRST_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWRST_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SWRST_CFG1_SPEC>> for R {
    fn from(reader: crate::R<SWRST_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `swrst_cfg1` writer"]
pub struct W(crate::W<SWRST_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWRST_CFG1_SPEC>;
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
impl core::convert::From<crate::W<SWRST_CFG1_SPEC>> for W {
    fn from(writer: crate::W<SWRST_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `swrst_s1a7` reader - "]
pub struct SWRST_S1A7_R(crate::FieldReader<bool, bool>);
impl SWRST_S1A7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1A7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1A7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1a7` writer - "]
pub struct SWRST_S1A7_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `swrst_s1a6` reader - "]
pub struct SWRST_S1A6_R(crate::FieldReader<bool, bool>);
impl SWRST_S1A6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1A6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1A6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1a6` writer - "]
pub struct SWRST_S1A6_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `swrst_s1a5` reader - "]
pub struct SWRST_S1A5_R(crate::FieldReader<bool, bool>);
impl SWRST_S1A5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1A5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1A5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1a5` writer - "]
pub struct SWRST_S1A5_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `swrst_s1a4` reader - "]
pub struct SWRST_S1A4_R(crate::FieldReader<bool, bool>);
impl SWRST_S1A4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1A4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1A4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1a4` writer - "]
pub struct SWRST_S1A4_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A4_W<'a> {
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
#[doc = "Field `swrst_s1a3` reader - "]
pub struct SWRST_S1A3_R(crate::FieldReader<bool, bool>);
impl SWRST_S1A3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1A3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1A3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1a3` writer - "]
pub struct SWRST_S1A3_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A3_W<'a> {
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
#[doc = "Field `swrst_s1a2` reader - "]
pub struct SWRST_S1A2_R(crate::FieldReader<bool, bool>);
impl SWRST_S1A2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1A2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1A2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1a2` writer - "]
pub struct SWRST_S1A2_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A2_W<'a> {
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
#[doc = "Field `swrst_s1a1` reader - "]
pub struct SWRST_S1A1_R(crate::FieldReader<bool, bool>);
impl SWRST_S1A1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1A1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1A1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1a1` writer - "]
pub struct SWRST_S1A1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A1_W<'a> {
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
#[doc = "Field `swrst_s1a0` reader - "]
pub struct SWRST_S1A0_R(crate::FieldReader<bool, bool>);
impl SWRST_S1A0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1A0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1A0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1a0` writer - "]
pub struct SWRST_S1A0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A0_W<'a> {
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
#[doc = "Field `swrst_s1f` reader - "]
pub struct SWRST_S1F_R(crate::FieldReader<bool, bool>);
impl SWRST_S1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1f` writer - "]
pub struct SWRST_S1F_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `swrst_s1e` reader - "]
pub struct SWRST_S1E_R(crate::FieldReader<bool, bool>);
impl SWRST_S1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1e` writer - "]
pub struct SWRST_S1E_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `swrst_s1d` reader - "]
pub struct SWRST_S1D_R(crate::FieldReader<bool, bool>);
impl SWRST_S1D_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1D_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1d` writer - "]
pub struct SWRST_S1D_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1D_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `swrst_s1c` reader - "]
pub struct SWRST_S1C_R(crate::FieldReader<bool, bool>);
impl SWRST_S1C_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1c` writer - "]
pub struct SWRST_S1C_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `swrst_s1b` reader - "]
pub struct SWRST_S1B_R(crate::FieldReader<bool, bool>);
impl SWRST_S1B_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1b` writer - "]
pub struct SWRST_S1B_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `swrst_s1a` reader - "]
pub struct SWRST_S1A_R(crate::FieldReader<bool, bool>);
impl SWRST_S1A_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S1A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S1A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s1a` writer - "]
pub struct SWRST_S1A_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S1A_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `swrst_s19` reader - "]
pub struct SWRST_S19_R(crate::FieldReader<bool, bool>);
impl SWRST_S19_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s19` writer - "]
pub struct SWRST_S19_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `swrst_s18` reader - "]
pub struct SWRST_S18_R(crate::FieldReader<bool, bool>);
impl SWRST_S18_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s18` writer - "]
pub struct SWRST_S18_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `swrst_s17` reader - "]
pub struct SWRST_S17_R(crate::FieldReader<bool, bool>);
impl SWRST_S17_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s17` writer - "]
pub struct SWRST_S17_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `swrst_s16` reader - "]
pub struct SWRST_S16_R(crate::FieldReader<bool, bool>);
impl SWRST_S16_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s16` writer - "]
pub struct SWRST_S16_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `swrst_s15` reader - "]
pub struct SWRST_S15_R(crate::FieldReader<bool, bool>);
impl SWRST_S15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s15` writer - "]
pub struct SWRST_S15_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S15_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `swrst_s14` reader - "]
pub struct SWRST_S14_R(crate::FieldReader<bool, bool>);
impl SWRST_S14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s14` writer - "]
pub struct SWRST_S14_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S14_W<'a> {
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
#[doc = "Field `swrst_s13` reader - "]
pub struct SWRST_S13_R(crate::FieldReader<bool, bool>);
impl SWRST_S13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s13` writer - "]
pub struct SWRST_S13_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S13_W<'a> {
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
#[doc = "Field `swrst_s12` reader - "]
pub struct SWRST_S12_R(crate::FieldReader<bool, bool>);
impl SWRST_S12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s12` writer - "]
pub struct SWRST_S12_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S12_W<'a> {
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
#[doc = "Field `swrst_s11` reader - "]
pub struct SWRST_S11_R(crate::FieldReader<bool, bool>);
impl SWRST_S11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s11` writer - "]
pub struct SWRST_S11_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S11_W<'a> {
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
#[doc = "Field `swrst_s10` reader - "]
pub struct SWRST_S10_R(crate::FieldReader<bool, bool>);
impl SWRST_S10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_S10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_S10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `swrst_s10` writer - "]
pub struct SWRST_S10_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_S10_W<'a> {
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
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn swrst_s1a7(&self) -> SWRST_S1A7_R {
        SWRST_S1A7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn swrst_s1a6(&self) -> SWRST_S1A6_R {
        SWRST_S1A6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn swrst_s1a5(&self) -> SWRST_S1A5_R {
        SWRST_S1A5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn swrst_s1a4(&self) -> SWRST_S1A4_R {
        SWRST_S1A4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn swrst_s1a3(&self) -> SWRST_S1A3_R {
        SWRST_S1A3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn swrst_s1a2(&self) -> SWRST_S1A2_R {
        SWRST_S1A2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swrst_s1a1(&self) -> SWRST_S1A1_R {
        SWRST_S1A1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_s1a0(&self) -> SWRST_S1A0_R {
        SWRST_S1A0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swrst_s1f(&self) -> SWRST_S1F_R {
        SWRST_S1F_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn swrst_s1e(&self) -> SWRST_S1E_R {
        SWRST_S1E_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn swrst_s1d(&self) -> SWRST_S1D_R {
        SWRST_S1D_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn swrst_s1c(&self) -> SWRST_S1C_R {
        SWRST_S1C_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn swrst_s1b(&self) -> SWRST_S1B_R {
        SWRST_S1B_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn swrst_s1a(&self) -> SWRST_S1A_R {
        SWRST_S1A_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_s19(&self) -> SWRST_S19_R {
        SWRST_S19_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s18(&self) -> SWRST_S18_R {
        SWRST_S18_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn swrst_s17(&self) -> SWRST_S17_R {
        SWRST_S17_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swrst_s16(&self) -> SWRST_S16_R {
        SWRST_S16_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn swrst_s15(&self) -> SWRST_S15_R {
        SWRST_S15_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s14(&self) -> SWRST_S14_R {
        SWRST_S14_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrst_s13(&self) -> SWRST_S13_R {
        SWRST_S13_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swrst_s12(&self) -> SWRST_S12_R {
        SWRST_S12_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s11(&self) -> SWRST_S11_R {
        SWRST_S11_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s10(&self) -> SWRST_S10_R {
        SWRST_S10_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn swrst_s1a7(&mut self) -> SWRST_S1A7_W {
        SWRST_S1A7_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn swrst_s1a6(&mut self) -> SWRST_S1A6_W {
        SWRST_S1A6_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn swrst_s1a5(&mut self) -> SWRST_S1A5_W {
        SWRST_S1A5_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn swrst_s1a4(&mut self) -> SWRST_S1A4_W {
        SWRST_S1A4_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn swrst_s1a3(&mut self) -> SWRST_S1A3_W {
        SWRST_S1A3_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn swrst_s1a2(&mut self) -> SWRST_S1A2_W {
        SWRST_S1A2_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn swrst_s1a1(&mut self) -> SWRST_S1A1_W {
        SWRST_S1A1_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn swrst_s1a0(&mut self) -> SWRST_S1A0_W {
        SWRST_S1A0_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn swrst_s1f(&mut self) -> SWRST_S1F_W {
        SWRST_S1F_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn swrst_s1e(&mut self) -> SWRST_S1E_W {
        SWRST_S1E_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn swrst_s1d(&mut self) -> SWRST_S1D_W {
        SWRST_S1D_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn swrst_s1c(&mut self) -> SWRST_S1C_W {
        SWRST_S1C_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn swrst_s1b(&mut self) -> SWRST_S1B_W {
        SWRST_S1B_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn swrst_s1a(&mut self) -> SWRST_S1A_W {
        SWRST_S1A_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn swrst_s19(&mut self) -> SWRST_S19_W {
        SWRST_S19_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn swrst_s18(&mut self) -> SWRST_S18_W {
        SWRST_S18_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn swrst_s17(&mut self) -> SWRST_S17_W {
        SWRST_S17_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn swrst_s16(&mut self) -> SWRST_S16_W {
        SWRST_S16_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn swrst_s15(&mut self) -> SWRST_S15_W {
        SWRST_S15_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn swrst_s14(&mut self) -> SWRST_S14_W {
        SWRST_S14_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrst_s13(&mut self) -> SWRST_S13_W {
        SWRST_S13_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swrst_s12(&mut self) -> SWRST_S12_W {
        SWRST_S12_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn swrst_s11(&mut self) -> SWRST_S11_W {
        SWRST_S11_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn swrst_s10(&mut self) -> SWRST_S10_W {
        SWRST_S10_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "swrst_cfg1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swrst_cfg1](index.html) module"]
pub struct SWRST_CFG1_SPEC;
impl crate::RegisterSpec for SWRST_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swrst_cfg1::R](R) reader structure"]
impl crate::Readable for SWRST_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swrst_cfg1::W](W) writer structure"]
impl crate::Writable for SWRST_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets swrst_cfg1 to value 0"]
impl crate::Resettable for SWRST_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
