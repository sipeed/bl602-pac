#[doc = "Register `DMA_C0Control` reader"]
pub struct R(crate::R<DMA_C0CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_C0CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA_C0CONTROL_SPEC>> for R {
    fn from(reader: crate::R<DMA_C0CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_C0Control` writer"]
pub struct W(crate::W<DMA_C0CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_C0CONTROL_SPEC>;
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
impl core::convert::From<crate::W<DMA_C0CONTROL_SPEC>> for W {
    fn from(writer: crate::W<DMA_C0CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I` reader - "]
pub struct I_R(crate::FieldReader<bool, bool>);
impl I_R {
    pub(crate) fn new(bits: bool) -> Self {
        I_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I` writer - "]
pub struct I_W<'a> {
    w: &'a mut W,
}
impl<'a> I_W<'a> {
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
#[doc = "Field `Prot` reader - "]
pub struct PROT_R(crate::FieldReader<u8, u8>);
impl PROT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PROT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Prot` writer - "]
pub struct PROT_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `DI` reader - "]
pub struct DI_R(crate::FieldReader<bool, bool>);
impl DI_R {
    pub(crate) fn new(bits: bool) -> Self {
        DI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DI` writer - "]
pub struct DI_W<'a> {
    w: &'a mut W,
}
impl<'a> DI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `SI` reader - "]
pub struct SI_R(crate::FieldReader<bool, bool>);
impl SI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SI` writer - "]
pub struct SI_W<'a> {
    w: &'a mut W,
}
impl<'a> SI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `SLargerD` reader - "]
pub struct SLARGERD_R(crate::FieldReader<bool, bool>);
impl SLARGERD_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLARGERD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLARGERD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLargerD` writer - "]
pub struct SLARGERD_W<'a> {
    w: &'a mut W,
}
impl<'a> SLARGERD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `DWidth` reader - "]
pub struct DWIDTH_R(crate::FieldReader<u8, u8>);
impl DWIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DWIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DWIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DWidth` writer - "]
pub struct DWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `SWidth` reader - "]
pub struct SWIDTH_R(crate::FieldReader<u8, u8>);
impl SWIDTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWIDTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWIDTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWidth` writer - "]
pub struct SWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SWIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `DBSize` reader - "]
pub struct DBSIZE_R(crate::FieldReader<u8, u8>);
impl DBSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DBSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DBSize` writer - "]
pub struct DBSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | (((value as u32) & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `SBSize` reader - "]
pub struct SBSIZE_R(crate::FieldReader<u8, u8>);
impl SBSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SBSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SBSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SBSize` writer - "]
pub struct SBSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SBSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `TransferSize` reader - "]
pub struct TRANSFERSIZE_R(crate::FieldReader<u16, u16>);
impl TRANSFERSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        TRANSFERSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSFERSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TransferSize` writer - "]
pub struct TRANSFERSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSFERSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn di(&self) -> DI_R {
        DI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn si(&self) -> SI_R {
        SI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slarger_d(&self) -> SLARGERD_R {
        SLARGERD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn swidth(&self) -> SWIDTH_R {
        SWIDTH_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn dbsize(&self) -> DBSIZE_R {
        DBSIZE_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sbsize(&self) -> SBSIZE_R {
        SBSIZE_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn transfer_size(&self) -> TRANSFERSIZE_R {
        TRANSFERSIZE_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn i(&mut self) -> I_W {
        I_W { w: self }
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn prot(&mut self) -> PROT_W {
        PROT_W { w: self }
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn di(&mut self) -> DI_W {
        DI_W { w: self }
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn si(&mut self) -> SI_W {
        SI_W { w: self }
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn slarger_d(&mut self) -> SLARGERD_W {
        SLARGERD_W { w: self }
    }
    #[doc = "Bits 21:23"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> DWIDTH_W {
        DWIDTH_W { w: self }
    }
    #[doc = "Bits 18:20"]
    #[inline(always)]
    pub fn swidth(&mut self) -> SWIDTH_W {
        SWIDTH_W { w: self }
    }
    #[doc = "Bits 15:17"]
    #[inline(always)]
    pub fn dbsize(&mut self) -> DBSIZE_W {
        DBSIZE_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn sbsize(&mut self) -> SBSIZE_W {
        SBSIZE_W { w: self }
    }
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn transfer_size(&mut self) -> TRANSFERSIZE_W {
        TRANSFERSIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_C0Control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c0control](index.html) module"]
pub struct DMA_C0CONTROL_SPEC;
impl crate::RegisterSpec for DMA_C0CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_c0control::R](R) reader structure"]
impl crate::Readable for DMA_C0CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_c0control::W](W) writer structure"]
impl crate::Writable for DMA_C0CONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_C0Control to value 0"]
impl crate::Resettable for DMA_C0CONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
