#[doc = "Register `tzc_rom_ctrl` reader"]
pub struct R(crate::R<TZC_ROM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZC_ROM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TZC_ROM_CTRL_SPEC>> for R {
    fn from(reader: crate::R<TZC_ROM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tzc_rom_ctrl` writer"]
pub struct W(crate::W<TZC_ROM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZC_ROM_CTRL_SPEC>;
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
impl core::convert::From<crate::W<TZC_ROM_CTRL_SPEC>> for W {
    fn from(writer: crate::W<TZC_ROM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tzc_sboot_done` reader - "]
pub struct TZC_SBOOT_DONE_R(crate::FieldReader<u8, u8>);
impl TZC_SBOOT_DONE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TZC_SBOOT_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_SBOOT_DONE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_sboot_done` writer - "]
pub struct TZC_SBOOT_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_SBOOT_DONE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `tzc_rom1_r1_lock` reader - "]
pub struct TZC_ROM1_R1_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_ROM1_R1_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM1_R1_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM1_R1_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom1_r1_lock` writer - "]
pub struct TZC_ROM1_R1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R1_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `tzc_rom1_r0_lock` reader - "]
pub struct TZC_ROM1_R0_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_ROM1_R0_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM1_R0_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM1_R0_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom1_r0_lock` writer - "]
pub struct TZC_ROM1_R0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R0_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `tzc_rom0_r1_lock` reader - "]
pub struct TZC_ROM0_R1_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_ROM0_R1_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM0_R1_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM0_R1_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom0_r1_lock` writer - "]
pub struct TZC_ROM0_R1_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R1_LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `tzc_rom0_r0_lock` reader - "]
pub struct TZC_ROM0_R0_LOCK_R(crate::FieldReader<bool, bool>);
impl TZC_ROM0_R0_LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM0_R0_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM0_R0_LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom0_r0_lock` writer - "]
pub struct TZC_ROM0_R0_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_LOCK_W<'a> {
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
#[doc = "Field `tzc_rom1_r1_en` reader - "]
pub struct TZC_ROM1_R1_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM1_R1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM1_R1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM1_R1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom1_r1_en` writer - "]
pub struct TZC_ROM1_R1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `tzc_rom1_r0_en` reader - "]
pub struct TZC_ROM1_R0_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM1_R0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM1_R0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM1_R0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom1_r0_en` writer - "]
pub struct TZC_ROM1_R0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R0_EN_W<'a> {
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
#[doc = "Field `tzc_rom0_r1_en` reader - "]
pub struct TZC_ROM0_R1_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM0_R1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM0_R1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM0_R1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom0_r1_en` writer - "]
pub struct TZC_ROM0_R1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `tzc_rom0_r0_en` reader - "]
pub struct TZC_ROM0_R0_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM0_R0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM0_R0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM0_R0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom0_r0_en` writer - "]
pub struct TZC_ROM0_R0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `tzc_rom1_r1_id1_en` reader - "]
pub struct TZC_ROM1_R1_ID1_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM1_R1_ID1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM1_R1_ID1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM1_R1_ID1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom1_r1_id1_en` writer - "]
pub struct TZC_ROM1_R1_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R1_ID1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `tzc_rom1_r0_id1_en` reader - "]
pub struct TZC_ROM1_R0_ID1_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM1_R0_ID1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM1_R0_ID1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM1_R0_ID1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom1_r0_id1_en` writer - "]
pub struct TZC_ROM1_R0_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R0_ID1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `tzc_rom0_r1_id1_en` reader - "]
pub struct TZC_ROM0_R1_ID1_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM0_R1_ID1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM0_R1_ID1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM0_R1_ID1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom0_r1_id1_en` writer - "]
pub struct TZC_ROM0_R1_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R1_ID1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `tzc_rom0_r0_id1_en` reader - "]
pub struct TZC_ROM0_R0_ID1_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM0_R0_ID1_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM0_R0_ID1_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM0_R0_ID1_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom0_r0_id1_en` writer - "]
pub struct TZC_ROM0_R0_ID1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_ID1_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `tzc_rom1_r1_id0_en` reader - "]
pub struct TZC_ROM1_R1_ID0_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM1_R1_ID0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM1_R1_ID0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM1_R1_ID0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom1_r1_id0_en` writer - "]
pub struct TZC_ROM1_R1_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R1_ID0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `tzc_rom1_r0_id0_en` reader - "]
pub struct TZC_ROM1_R0_ID0_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM1_R0_ID0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM1_R0_ID0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM1_R0_ID0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom1_r0_id0_en` writer - "]
pub struct TZC_ROM1_R0_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM1_R0_ID0_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `tzc_rom0_r1_id0_en` reader - "]
pub struct TZC_ROM0_R1_ID0_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM0_R1_ID0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM0_R1_ID0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM0_R1_ID0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom0_r1_id0_en` writer - "]
pub struct TZC_ROM0_R1_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R1_ID0_EN_W<'a> {
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
#[doc = "Field `tzc_rom0_r0_id0_en` reader - "]
pub struct TZC_ROM0_R0_ID0_EN_R(crate::FieldReader<bool, bool>);
impl TZC_ROM0_R0_ID0_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TZC_ROM0_R0_ID0_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TZC_ROM0_R0_ID0_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tzc_rom0_r0_id0_en` writer - "]
pub struct TZC_ROM0_R0_ID0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC_ROM0_R0_ID0_EN_W<'a> {
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
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn tzc_sboot_done(&self) -> TZC_SBOOT_DONE_R {
        TZC_SBOOT_DONE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_rom1_r1_lock(&self) -> TZC_ROM1_R1_LOCK_R {
        TZC_ROM1_R1_LOCK_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_rom1_r0_lock(&self) -> TZC_ROM1_R0_LOCK_R {
        TZC_ROM1_R0_LOCK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_rom0_r1_lock(&self) -> TZC_ROM0_R1_LOCK_R {
        TZC_ROM0_R1_LOCK_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_rom0_r0_lock(&self) -> TZC_ROM0_R0_LOCK_R {
        TZC_ROM0_R0_LOCK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_rom1_r1_en(&self) -> TZC_ROM1_R1_EN_R {
        TZC_ROM1_R1_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_rom1_r0_en(&self) -> TZC_ROM1_R0_EN_R {
        TZC_ROM1_R0_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_rom0_r1_en(&self) -> TZC_ROM0_R1_EN_R {
        TZC_ROM0_R1_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_rom0_r0_en(&self) -> TZC_ROM0_R0_EN_R {
        TZC_ROM0_R0_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id1_en(&self) -> TZC_ROM1_R1_ID1_EN_R {
        TZC_ROM1_R1_ID1_EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id1_en(&self) -> TZC_ROM1_R0_ID1_EN_R {
        TZC_ROM1_R0_ID1_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id1_en(&self) -> TZC_ROM0_R1_ID1_EN_R {
        TZC_ROM0_R1_ID1_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id1_en(&self) -> TZC_ROM0_R0_ID1_EN_R {
        TZC_ROM0_R0_ID1_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id0_en(&self) -> TZC_ROM1_R1_ID0_EN_R {
        TZC_ROM1_R1_ID0_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id0_en(&self) -> TZC_ROM1_R0_ID0_EN_R {
        TZC_ROM1_R0_ID0_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id0_en(&self) -> TZC_ROM0_R1_ID0_EN_R {
        TZC_ROM0_R1_ID0_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id0_en(&self) -> TZC_ROM0_R0_ID0_EN_R {
        TZC_ROM0_R0_ID0_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn tzc_sboot_done(&mut self) -> TZC_SBOOT_DONE_W {
        TZC_SBOOT_DONE_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn tzc_rom1_r1_lock(&mut self) -> TZC_ROM1_R1_LOCK_W {
        TZC_ROM1_R1_LOCK_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn tzc_rom1_r0_lock(&mut self) -> TZC_ROM1_R0_LOCK_W {
        TZC_ROM1_R0_LOCK_W { w: self }
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn tzc_rom0_r1_lock(&mut self) -> TZC_ROM0_R1_LOCK_W {
        TZC_ROM0_R1_LOCK_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn tzc_rom0_r0_lock(&mut self) -> TZC_ROM0_R0_LOCK_W {
        TZC_ROM0_R0_LOCK_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tzc_rom1_r1_en(&mut self) -> TZC_ROM1_R1_EN_W {
        TZC_ROM1_R1_EN_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tzc_rom1_r0_en(&mut self) -> TZC_ROM1_R0_EN_W {
        TZC_ROM1_R0_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn tzc_rom0_r1_en(&mut self) -> TZC_ROM0_R1_EN_W {
        TZC_ROM0_R1_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tzc_rom0_r0_en(&mut self) -> TZC_ROM0_R0_EN_W {
        TZC_ROM0_R0_EN_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id1_en(&mut self) -> TZC_ROM1_R1_ID1_EN_W {
        TZC_ROM1_R1_ID1_EN_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id1_en(&mut self) -> TZC_ROM1_R0_ID1_EN_W {
        TZC_ROM1_R0_ID1_EN_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id1_en(&mut self) -> TZC_ROM0_R1_ID1_EN_W {
        TZC_ROM0_R1_ID1_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id1_en(&mut self) -> TZC_ROM0_R0_ID1_EN_W {
        TZC_ROM0_R0_ID1_EN_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn tzc_rom1_r1_id0_en(&mut self) -> TZC_ROM1_R1_ID0_EN_W {
        TZC_ROM1_R1_ID0_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tzc_rom1_r0_id0_en(&mut self) -> TZC_ROM1_R0_ID0_EN_W {
        TZC_ROM1_R0_ID0_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tzc_rom0_r1_id0_en(&mut self) -> TZC_ROM0_R1_ID0_EN_W {
        TZC_ROM0_R1_ID0_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tzc_rom0_r0_id0_en(&mut self) -> TZC_ROM0_R0_ID0_EN_W {
        TZC_ROM0_R0_ID0_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tzc_rom_ctrl.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzc_rom_ctrl](index.html) module"]
pub struct TZC_ROM_CTRL_SPEC;
impl crate::RegisterSpec for TZC_ROM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzc_rom_ctrl::R](R) reader structure"]
impl crate::Readable for TZC_ROM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzc_rom_ctrl::W](W) writer structure"]
impl crate::Writable for TZC_ROM_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tzc_rom_ctrl to value 0x0f0f"]
impl crate::Resettable for TZC_ROM_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f0f
    }
}
