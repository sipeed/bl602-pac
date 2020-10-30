#[doc = "Register `gpadc_config` reader"]
pub struct R(crate::R<GPADC_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPADC_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<GPADC_CONFIG_SPEC>> for R {
    fn from(reader: crate::R<GPADC_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `gpadc_config` writer"]
pub struct W(crate::W<GPADC_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPADC_CONFIG_SPEC>;
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
impl core::convert::From<crate::W<GPADC_CONFIG_SPEC>> for W {
    fn from(writer: crate::W<GPADC_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rsvd_31_24` reader - "]
pub struct RSVD_31_24_R(crate::FieldReader<u8, u8>);
impl RSVD_31_24_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSVD_31_24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_31_24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rsvd_31_24` writer - "]
pub struct RSVD_31_24_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_31_24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `gpadc_fifo_thl` reader - "]
pub struct GPADC_FIFO_THL_R(crate::FieldReader<u8, u8>);
impl GPADC_FIFO_THL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_FIFO_THL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_FIFO_THL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_fifo_thl` writer - "]
pub struct GPADC_FIFO_THL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_THL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Field `gpadc_fifo_data_count` reader - "]
pub struct GPADC_FIFO_DATA_COUNT_R(crate::FieldReader<u8, u8>);
impl GPADC_FIFO_DATA_COUNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPADC_FIFO_DATA_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_FIFO_DATA_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_fifo_data_count` writer - "]
pub struct GPADC_FIFO_DATA_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_DATA_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `gpadc_fifo_underrun_mask` reader - "]
pub struct GPADC_FIFO_UNDERRUN_MASK_R(crate::FieldReader<bool, bool>);
impl GPADC_FIFO_UNDERRUN_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_FIFO_UNDERRUN_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_FIFO_UNDERRUN_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_fifo_underrun_mask` writer - "]
pub struct GPADC_FIFO_UNDERRUN_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_UNDERRUN_MASK_W<'a> {
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
#[doc = "Field `gpadc_fifo_overrun_mask` reader - "]
pub struct GPADC_FIFO_OVERRUN_MASK_R(crate::FieldReader<bool, bool>);
impl GPADC_FIFO_OVERRUN_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_FIFO_OVERRUN_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_FIFO_OVERRUN_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_fifo_overrun_mask` writer - "]
pub struct GPADC_FIFO_OVERRUN_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_OVERRUN_MASK_W<'a> {
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
#[doc = "Field `gpadc_rdy_mask` reader - "]
pub struct GPADC_RDY_MASK_R(crate::FieldReader<bool, bool>);
impl GPADC_RDY_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_RDY_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_RDY_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_rdy_mask` writer - "]
pub struct GPADC_RDY_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RDY_MASK_W<'a> {
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
#[doc = "Field `gpadc_fifo_underrun_clr` reader - "]
pub struct GPADC_FIFO_UNDERRUN_CLR_R(crate::FieldReader<bool, bool>);
impl GPADC_FIFO_UNDERRUN_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_FIFO_UNDERRUN_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_FIFO_UNDERRUN_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_fifo_underrun_clr` writer - "]
pub struct GPADC_FIFO_UNDERRUN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_UNDERRUN_CLR_W<'a> {
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
#[doc = "Field `gpadc_fifo_overrun_clr` reader - "]
pub struct GPADC_FIFO_OVERRUN_CLR_R(crate::FieldReader<bool, bool>);
impl GPADC_FIFO_OVERRUN_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_FIFO_OVERRUN_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_FIFO_OVERRUN_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_fifo_overrun_clr` writer - "]
pub struct GPADC_FIFO_OVERRUN_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_OVERRUN_CLR_W<'a> {
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
#[doc = "Field `gpadc_rdy_clr` reader - "]
pub struct GPADC_RDY_CLR_R(crate::FieldReader<bool, bool>);
impl GPADC_RDY_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_RDY_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_RDY_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_rdy_clr` writer - "]
pub struct GPADC_RDY_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RDY_CLR_W<'a> {
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
#[doc = "Field `gpadc_fifo_underrun` reader - "]
pub struct GPADC_FIFO_UNDERRUN_R(crate::FieldReader<bool, bool>);
impl GPADC_FIFO_UNDERRUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_FIFO_UNDERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_FIFO_UNDERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_fifo_underrun` writer - "]
pub struct GPADC_FIFO_UNDERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_UNDERRUN_W<'a> {
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
#[doc = "Field `gpadc_fifo_overrun` reader - "]
pub struct GPADC_FIFO_OVERRUN_R(crate::FieldReader<bool, bool>);
impl GPADC_FIFO_OVERRUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_FIFO_OVERRUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_FIFO_OVERRUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_fifo_overrun` writer - "]
pub struct GPADC_FIFO_OVERRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_OVERRUN_W<'a> {
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
#[doc = "Field `gpadc_rdy` reader - "]
pub struct GPADC_RDY_R(crate::FieldReader<bool, bool>);
impl GPADC_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_rdy` writer - "]
pub struct GPADC_RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_RDY_W<'a> {
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
#[doc = "Field `gpadc_fifo_full` reader - "]
pub struct GPADC_FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl GPADC_FIFO_FULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_fifo_full` writer - "]
pub struct GPADC_FIFO_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_FULL_W<'a> {
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
#[doc = "Field `gpadc_fifo_ne` reader - "]
pub struct GPADC_FIFO_NE_R(crate::FieldReader<bool, bool>);
impl GPADC_FIFO_NE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_FIFO_NE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_FIFO_NE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_fifo_ne` writer - "]
pub struct GPADC_FIFO_NE_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_NE_W<'a> {
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
#[doc = "Field `gpadc_fifo_clr` reader - "]
pub struct GPADC_FIFO_CLR_R(crate::FieldReader<bool, bool>);
impl GPADC_FIFO_CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_FIFO_CLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_FIFO_CLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_fifo_clr` writer - "]
pub struct GPADC_FIFO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_FIFO_CLR_W<'a> {
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
#[doc = "Field `gpadc_dma_en` reader - "]
pub struct GPADC_DMA_EN_R(crate::FieldReader<bool, bool>);
impl GPADC_DMA_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GPADC_DMA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPADC_DMA_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpadc_dma_en` writer - "]
pub struct GPADC_DMA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPADC_DMA_EN_W<'a> {
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
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&self) -> RSVD_31_24_R {
        RSVD_31_24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn gpadc_fifo_thl(&self) -> GPADC_FIFO_THL_R {
        GPADC_FIFO_THL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn gpadc_fifo_data_count(&self) -> GPADC_FIFO_DATA_COUNT_R {
        GPADC_FIFO_DATA_COUNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_mask(&self) -> GPADC_FIFO_UNDERRUN_MASK_R {
        GPADC_FIFO_UNDERRUN_MASK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_mask(&self) -> GPADC_FIFO_OVERRUN_MASK_R {
        GPADC_FIFO_OVERRUN_MASK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpadc_rdy_mask(&self) -> GPADC_RDY_MASK_R {
        GPADC_RDY_MASK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_clr(&self) -> GPADC_FIFO_UNDERRUN_CLR_R {
        GPADC_FIFO_UNDERRUN_CLR_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_clr(&self) -> GPADC_FIFO_OVERRUN_CLR_R {
        GPADC_FIFO_OVERRUN_CLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_rdy_clr(&self) -> GPADC_RDY_CLR_R {
        GPADC_RDY_CLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun(&self) -> GPADC_FIFO_UNDERRUN_R {
        GPADC_FIFO_UNDERRUN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun(&self) -> GPADC_FIFO_OVERRUN_R {
        GPADC_FIFO_OVERRUN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_rdy(&self) -> GPADC_RDY_R {
        GPADC_RDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_fifo_full(&self) -> GPADC_FIFO_FULL_R {
        GPADC_FIFO_FULL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_fifo_ne(&self) -> GPADC_FIFO_NE_R {
        GPADC_FIFO_NE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_fifo_clr(&self) -> GPADC_FIFO_CLR_R {
        GPADC_FIFO_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_dma_en(&self) -> GPADC_DMA_EN_R {
        GPADC_DMA_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rsvd_31_24(&mut self) -> RSVD_31_24_W {
        RSVD_31_24_W { w: self }
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn gpadc_fifo_thl(&mut self) -> GPADC_FIFO_THL_W {
        GPADC_FIFO_THL_W { w: self }
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn gpadc_fifo_data_count(&mut self) -> GPADC_FIFO_DATA_COUNT_W {
        GPADC_FIFO_DATA_COUNT_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_mask(&mut self) -> GPADC_FIFO_UNDERRUN_MASK_W {
        GPADC_FIFO_UNDERRUN_MASK_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_mask(&mut self) -> GPADC_FIFO_OVERRUN_MASK_W {
        GPADC_FIFO_OVERRUN_MASK_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gpadc_rdy_mask(&mut self) -> GPADC_RDY_MASK_W {
        GPADC_RDY_MASK_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun_clr(&mut self) -> GPADC_FIFO_UNDERRUN_CLR_W {
        GPADC_FIFO_UNDERRUN_CLR_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun_clr(&mut self) -> GPADC_FIFO_OVERRUN_CLR_W {
        GPADC_FIFO_OVERRUN_CLR_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gpadc_rdy_clr(&mut self) -> GPADC_RDY_CLR_W {
        GPADC_RDY_CLR_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gpadc_fifo_underrun(&mut self) -> GPADC_FIFO_UNDERRUN_W {
        GPADC_FIFO_UNDERRUN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gpadc_fifo_overrun(&mut self) -> GPADC_FIFO_OVERRUN_W {
        GPADC_FIFO_OVERRUN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gpadc_rdy(&mut self) -> GPADC_RDY_W {
        GPADC_RDY_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gpadc_fifo_full(&mut self) -> GPADC_FIFO_FULL_W {
        GPADC_FIFO_FULL_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gpadc_fifo_ne(&mut self) -> GPADC_FIFO_NE_W {
        GPADC_FIFO_NE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gpadc_fifo_clr(&mut self) -> GPADC_FIFO_CLR_W {
        GPADC_FIFO_CLR_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpadc_dma_en(&mut self) -> GPADC_DMA_EN_W {
        GPADC_DMA_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "gpadc_config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpadc_config](index.html) module"]
pub struct GPADC_CONFIG_SPEC;
impl crate::RegisterSpec for GPADC_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpadc_config::R](R) reader structure"]
impl crate::Readable for GPADC_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpadc_config::W](W) writer structure"]
impl crate::Writable for GPADC_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets gpadc_config to value 0"]
impl crate::Resettable for GPADC_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
