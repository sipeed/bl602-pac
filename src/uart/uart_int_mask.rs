#[doc = "Register `uart_int_mask` reader"]
pub struct R(crate::R<UART_INT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_INT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_INT_MASK_SPEC>> for R {
    fn from(reader: crate::R<UART_INT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_int_mask` writer"]
pub struct W(crate::W<UART_INT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_INT_MASK_SPEC>;
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
impl core::convert::From<crate::W<UART_INT_MASK_SPEC>> for W {
    fn from(writer: crate::W<UART_INT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_urx_fer_mask` reader - "]
pub struct CR_URX_FER_MASK_R(crate::FieldReader<bool, bool>);
impl CR_URX_FER_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_FER_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_FER_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_fer_mask` writer - "]
pub struct CR_URX_FER_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_FER_MASK_W<'a> {
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
#[doc = "Field `cr_utx_fer_mask` reader - "]
pub struct CR_UTX_FER_MASK_R(crate::FieldReader<bool, bool>);
impl CR_UTX_FER_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_UTX_FER_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_FER_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_fer_mask` writer - "]
pub struct CR_UTX_FER_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_FER_MASK_W<'a> {
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
#[doc = "Field `cr_urx_pce_mask` reader - "]
pub struct CR_URX_PCE_MASK_R(crate::FieldReader<bool, bool>);
impl CR_URX_PCE_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_PCE_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_PCE_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_pce_mask` writer - "]
pub struct CR_URX_PCE_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_PCE_MASK_W<'a> {
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
#[doc = "Field `cr_urx_rto_mask` reader - "]
pub struct CR_URX_RTO_MASK_R(crate::FieldReader<bool, bool>);
impl CR_URX_RTO_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_RTO_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_RTO_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_rto_mask` writer - "]
pub struct CR_URX_RTO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_RTO_MASK_W<'a> {
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
#[doc = "Field `cr_urx_fifo_mask` reader - "]
pub struct CR_URX_FIFO_MASK_R(crate::FieldReader<bool, bool>);
impl CR_URX_FIFO_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_FIFO_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_FIFO_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_fifo_mask` writer - "]
pub struct CR_URX_FIFO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_FIFO_MASK_W<'a> {
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
#[doc = "Field `cr_utx_fifo_mask` reader - "]
pub struct CR_UTX_FIFO_MASK_R(crate::FieldReader<bool, bool>);
impl CR_UTX_FIFO_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_UTX_FIFO_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_FIFO_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_fifo_mask` writer - "]
pub struct CR_UTX_FIFO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_FIFO_MASK_W<'a> {
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
#[doc = "Field `cr_urx_end_mask` reader - "]
pub struct CR_URX_END_MASK_R(crate::FieldReader<bool, bool>);
impl CR_URX_END_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_URX_END_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_URX_END_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_urx_end_mask` writer - "]
pub struct CR_URX_END_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_URX_END_MASK_W<'a> {
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
#[doc = "Field `cr_utx_end_mask` reader - "]
pub struct CR_UTX_END_MASK_R(crate::FieldReader<bool, bool>);
impl CR_UTX_END_MASK_R {
    pub(crate) fn new(bits: bool) -> Self {
        CR_UTX_END_MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_UTX_END_MASK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_utx_end_mask` writer - "]
pub struct CR_UTX_END_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_UTX_END_MASK_W<'a> {
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
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_fer_mask(&self) -> CR_URX_FER_MASK_R {
        CR_URX_FER_MASK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_mask(&self) -> CR_UTX_FER_MASK_R {
        CR_UTX_FER_MASK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_mask(&self) -> CR_URX_PCE_MASK_R {
        CR_URX_PCE_MASK_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_mask(&self) -> CR_URX_RTO_MASK_R {
        CR_URX_RTO_MASK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_fifo_mask(&self) -> CR_URX_FIFO_MASK_R {
        CR_URX_FIFO_MASK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_fifo_mask(&self) -> CR_UTX_FIFO_MASK_R {
        CR_UTX_FIFO_MASK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_mask(&self) -> CR_URX_END_MASK_R {
        CR_URX_END_MASK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_mask(&self) -> CR_UTX_END_MASK_R {
        CR_UTX_END_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_urx_fer_mask(&mut self) -> CR_URX_FER_MASK_W {
        CR_URX_FER_MASK_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cr_utx_fer_mask(&mut self) -> CR_UTX_FER_MASK_W {
        CR_UTX_FER_MASK_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_urx_pce_mask(&mut self) -> CR_URX_PCE_MASK_W {
        CR_URX_PCE_MASK_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn cr_urx_rto_mask(&mut self) -> CR_URX_RTO_MASK_W {
        CR_URX_RTO_MASK_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_urx_fifo_mask(&mut self) -> CR_URX_FIFO_MASK_W {
        CR_URX_FIFO_MASK_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cr_utx_fifo_mask(&mut self) -> CR_UTX_FIFO_MASK_W {
        CR_UTX_FIFO_MASK_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_urx_end_mask(&mut self) -> CR_URX_END_MASK_W {
        CR_URX_END_MASK_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cr_utx_end_mask(&mut self) -> CR_UTX_END_MASK_W {
        CR_UTX_END_MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_mask](index.html) module"]
pub struct UART_INT_MASK_SPEC;
impl crate::RegisterSpec for UART_INT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_int_mask::R](R) reader structure"]
impl crate::Readable for UART_INT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_int_mask::W](W) writer structure"]
impl crate::Writable for UART_INT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets uart_int_mask to value 0"]
impl crate::Resettable for UART_INT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
