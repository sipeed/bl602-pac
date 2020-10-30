#[doc = "Register `DMA_C2Config` reader"]
pub struct R(crate::R<DMA_C2CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_C2CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DMA_C2CONFIG_SPEC>> for R {
    fn from(reader: crate::R<DMA_C2CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_C2Config` writer"]
pub struct W(crate::W<DMA_C2CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_C2CONFIG_SPEC>;
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
impl core::convert::From<crate::W<DMA_C2CONFIG_SPEC>> for W {
    fn from(writer: crate::W<DMA_C2CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `H` reader - "]
pub struct H_R(crate::FieldReader<bool, bool>);
impl H_R {
    pub(crate) fn new(bits: bool) -> Self {
        H_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for H_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `H` writer - "]
pub struct H_W<'a> {
    w: &'a mut W,
}
impl<'a> H_W<'a> {
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
#[doc = "Field `A` reader - "]
pub struct A_R(crate::FieldReader<bool, bool>);
impl A_R {
    pub(crate) fn new(bits: bool) -> Self {
        A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A` writer - "]
pub struct A_W<'a> {
    w: &'a mut W,
}
impl<'a> A_W<'a> {
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
#[doc = "Field `L` reader - "]
pub struct L_R(crate::FieldReader<bool, bool>);
impl L_R {
    pub(crate) fn new(bits: bool) -> Self {
        L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L` writer - "]
pub struct L_W<'a> {
    w: &'a mut W,
}
impl<'a> L_W<'a> {
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
#[doc = "Field `ITC` reader - "]
pub struct ITC_R(crate::FieldReader<bool, bool>);
impl ITC_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITC` writer - "]
pub struct ITC_W<'a> {
    w: &'a mut W,
}
impl<'a> ITC_W<'a> {
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
#[doc = "Field `IE` reader - "]
pub struct IE_R(crate::FieldReader<bool, bool>);
impl IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE` writer - "]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
#[doc = "Field `FlowCntrl` reader - "]
pub struct FLOWCNTRL_R(crate::FieldReader<u8, u8>);
impl FLOWCNTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FLOWCNTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLOWCNTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FlowCntrl` writer - "]
pub struct FLOWCNTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> FLOWCNTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Field `DstPeripheral` reader - "]
pub struct DSTPERIPHERAL_R(crate::FieldReader<u8, u8>);
impl DSTPERIPHERAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DSTPERIPHERAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSTPERIPHERAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DstPeripheral` writer - "]
pub struct DSTPERIPHERAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTPERIPHERAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | (((value as u32) & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `SrcPeripheral` reader - "]
pub struct SRCPERIPHERAL_R(crate::FieldReader<u8, u8>);
impl SRCPERIPHERAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SRCPERIPHERAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRCPERIPHERAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SrcPeripheral` writer - "]
pub struct SRCPERIPHERAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCPERIPHERAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | (((value as u32) & 0x1f) << 1);
        self.w
    }
}
#[doc = "Field `E` reader - "]
pub struct E_R(crate::FieldReader<bool, bool>);
impl E_R {
    pub(crate) fn new(bits: bool) -> Self {
        E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `E` writer - "]
pub struct E_W<'a> {
    w: &'a mut W,
}
impl<'a> E_W<'a> {
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
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn itc(&self) -> ITC_R {
        ITC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn flow_cntrl(&self) -> FLOWCNTRL_R {
        FLOWCNTRL_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn dst_peripheral(&self) -> DSTPERIPHERAL_R {
        DSTPERIPHERAL_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn src_peripheral(&self) -> SRCPERIPHERAL_R {
        SRCPERIPHERAL_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&self) -> E_R {
        E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn h(&mut self) -> H_W {
        H_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn a(&mut self) -> A_W {
        A_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn l(&mut self) -> L_W {
        L_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn itc(&mut self) -> ITC_W {
        ITC_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn flow_cntrl(&mut self) -> FLOWCNTRL_W {
        FLOWCNTRL_W { w: self }
    }
    #[doc = "Bits 6:10"]
    #[inline(always)]
    pub fn dst_peripheral(&mut self) -> DSTPERIPHERAL_W {
        DSTPERIPHERAL_W { w: self }
    }
    #[doc = "Bits 1:5"]
    #[inline(always)]
    pub fn src_peripheral(&mut self) -> SRCPERIPHERAL_W {
        SRCPERIPHERAL_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn e(&mut self) -> E_W {
        E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA_C2Config.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_c2config](index.html) module"]
pub struct DMA_C2CONFIG_SPEC;
impl crate::RegisterSpec for DMA_C2CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_c2config::R](R) reader structure"]
impl crate::Readable for DMA_C2CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_c2config::W](W) writer structure"]
impl crate::Writable for DMA_C2CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_C2Config to value 0"]
impl crate::Resettable for DMA_C2CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
