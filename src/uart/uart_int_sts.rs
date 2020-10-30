#[doc = "Register `uart_int_sts` reader"]
pub struct R(crate::R<UART_INT_STS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_INT_STS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_INT_STS_SPEC>> for R {
    fn from(reader: crate::R<UART_INT_STS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `uart_int_sts` writer"]
pub struct W(crate::W<UART_INT_STS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_INT_STS_SPEC>;
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
impl core::convert::From<crate::W<UART_INT_STS_SPEC>> for W {
    fn from(writer: crate::W<UART_INT_STS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `urx_fer_int` reader - "]
pub struct URX_FER_INT_R(crate::FieldReader<bool, bool>);
impl URX_FER_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        URX_FER_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URX_FER_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `urx_fer_int` writer - "]
pub struct URX_FER_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> URX_FER_INT_W<'a> {
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
#[doc = "Field `utx_fer_int` reader - "]
pub struct UTX_FER_INT_R(crate::FieldReader<bool, bool>);
impl UTX_FER_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTX_FER_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTX_FER_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `utx_fer_int` writer - "]
pub struct UTX_FER_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> UTX_FER_INT_W<'a> {
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
#[doc = "Field `urx_pce_int` reader - "]
pub struct URX_PCE_INT_R(crate::FieldReader<bool, bool>);
impl URX_PCE_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        URX_PCE_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URX_PCE_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `urx_pce_int` writer - "]
pub struct URX_PCE_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> URX_PCE_INT_W<'a> {
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
#[doc = "Field `urx_rto_int` reader - "]
pub struct URX_RTO_INT_R(crate::FieldReader<bool, bool>);
impl URX_RTO_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        URX_RTO_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URX_RTO_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `urx_rto_int` writer - "]
pub struct URX_RTO_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> URX_RTO_INT_W<'a> {
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
#[doc = "Field `urx_fifo_int` reader - "]
pub struct URX_FIFO_INT_R(crate::FieldReader<bool, bool>);
impl URX_FIFO_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        URX_FIFO_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URX_FIFO_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `urx_fifo_int` writer - "]
pub struct URX_FIFO_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> URX_FIFO_INT_W<'a> {
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
#[doc = "Field `utx_fifo_int` reader - "]
pub struct UTX_FIFO_INT_R(crate::FieldReader<bool, bool>);
impl UTX_FIFO_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTX_FIFO_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTX_FIFO_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `utx_fifo_int` writer - "]
pub struct UTX_FIFO_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> UTX_FIFO_INT_W<'a> {
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
#[doc = "Field `urx_end_int` reader - "]
pub struct URX_END_INT_R(crate::FieldReader<bool, bool>);
impl URX_END_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        URX_END_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URX_END_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `urx_end_int` writer - "]
pub struct URX_END_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> URX_END_INT_W<'a> {
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
#[doc = "Field `utx_end_int` reader - "]
pub struct UTX_END_INT_R(crate::FieldReader<bool, bool>);
impl UTX_END_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        UTX_END_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UTX_END_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `utx_end_int` writer - "]
pub struct UTX_END_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> UTX_END_INT_W<'a> {
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
    pub fn urx_fer_int(&self) -> URX_FER_INT_R {
        URX_FER_INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn utx_fer_int(&self) -> UTX_FER_INT_R {
        UTX_FER_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn urx_pce_int(&self) -> URX_PCE_INT_R {
        URX_PCE_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn urx_rto_int(&self) -> URX_RTO_INT_R {
        URX_RTO_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn urx_fifo_int(&self) -> URX_FIFO_INT_R {
        URX_FIFO_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn utx_fifo_int(&self) -> UTX_FIFO_INT_R {
        UTX_FIFO_INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn urx_end_int(&self) -> URX_END_INT_R {
        URX_END_INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utx_end_int(&self) -> UTX_END_INT_R {
        UTX_END_INT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn urx_fer_int(&mut self) -> URX_FER_INT_W {
        URX_FER_INT_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn utx_fer_int(&mut self) -> UTX_FER_INT_W {
        UTX_FER_INT_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn urx_pce_int(&mut self) -> URX_PCE_INT_W {
        URX_PCE_INT_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn urx_rto_int(&mut self) -> URX_RTO_INT_W {
        URX_RTO_INT_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn urx_fifo_int(&mut self) -> URX_FIFO_INT_W {
        URX_FIFO_INT_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn utx_fifo_int(&mut self) -> UTX_FIFO_INT_W {
        UTX_FIFO_INT_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn urx_end_int(&mut self) -> URX_END_INT_W {
        URX_END_INT_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn utx_end_int(&mut self) -> UTX_END_INT_W {
        UTX_END_INT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_int_sts](index.html) module"]
pub struct UART_INT_STS_SPEC;
impl crate::RegisterSpec for UART_INT_STS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_int_sts::R](R) reader structure"]
impl crate::Readable for UART_INT_STS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_int_sts::W](W) writer structure"]
impl crate::Writable for UART_INT_STS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets uart_int_sts to value 0"]
impl crate::Resettable for UART_INT_STS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
