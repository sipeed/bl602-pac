#[doc = "Register `UART_SIG_SEL_0` reader"]
pub struct R(crate::R<UART_SIG_SEL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_SIG_SEL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UART_SIG_SEL_0_SPEC>> for R {
    fn from(reader: crate::R<UART_SIG_SEL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART_SIG_SEL_0` writer"]
pub struct W(crate::W<UART_SIG_SEL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART_SIG_SEL_0_SPEC>;
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
impl core::convert::From<crate::W<UART_SIG_SEL_0_SPEC>> for W {
    fn from(writer: crate::W<UART_SIG_SEL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart_sig_7_sel` reader - "]
pub struct UART_SIG_7_SEL_R(crate::FieldReader<u8, u8>);
impl UART_SIG_7_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_SIG_7_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SIG_7_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_sig_7_sel` writer - "]
pub struct UART_SIG_7_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_7_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `uart_sig_6_sel` reader - "]
pub struct UART_SIG_6_SEL_R(crate::FieldReader<u8, u8>);
impl UART_SIG_6_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_SIG_6_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SIG_6_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_sig_6_sel` writer - "]
pub struct UART_SIG_6_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_6_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `uart_sig_5_sel` reader - "]
pub struct UART_SIG_5_SEL_R(crate::FieldReader<u8, u8>);
impl UART_SIG_5_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_SIG_5_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SIG_5_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_sig_5_sel` writer - "]
pub struct UART_SIG_5_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_5_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `uart_sig_4_sel` reader - "]
pub struct UART_SIG_4_SEL_R(crate::FieldReader<u8, u8>);
impl UART_SIG_4_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_SIG_4_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SIG_4_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_sig_4_sel` writer - "]
pub struct UART_SIG_4_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_4_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `uart_sig_3_sel` reader - "]
pub struct UART_SIG_3_SEL_R(crate::FieldReader<u8, u8>);
impl UART_SIG_3_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_SIG_3_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SIG_3_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_sig_3_sel` writer - "]
pub struct UART_SIG_3_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_3_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `uart_sig_2_sel` reader - "]
pub struct UART_SIG_2_SEL_R(crate::FieldReader<u8, u8>);
impl UART_SIG_2_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_SIG_2_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SIG_2_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_sig_2_sel` writer - "]
pub struct UART_SIG_2_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_2_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `uart_sig_1_sel` reader - "]
pub struct UART_SIG_1_SEL_R(crate::FieldReader<u8, u8>);
impl UART_SIG_1_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_SIG_1_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SIG_1_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_sig_1_sel` writer - "]
pub struct UART_SIG_1_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_1_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `uart_sig_0_sel` reader - "]
pub struct UART_SIG_0_SEL_R(crate::FieldReader<u8, u8>);
impl UART_SIG_0_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART_SIG_0_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_SIG_0_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_sig_0_sel` writer - "]
pub struct UART_SIG_0_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_SIG_0_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn uart_sig_7_sel(&self) -> UART_SIG_7_SEL_R {
        UART_SIG_7_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn uart_sig_6_sel(&self) -> UART_SIG_6_SEL_R {
        UART_SIG_6_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn uart_sig_5_sel(&self) -> UART_SIG_5_SEL_R {
        UART_SIG_5_SEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn uart_sig_4_sel(&self) -> UART_SIG_4_SEL_R {
        UART_SIG_4_SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn uart_sig_3_sel(&self) -> UART_SIG_3_SEL_R {
        UART_SIG_3_SEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uart_sig_2_sel(&self) -> UART_SIG_2_SEL_R {
        UART_SIG_2_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn uart_sig_1_sel(&self) -> UART_SIG_1_SEL_R {
        UART_SIG_1_SEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn uart_sig_0_sel(&self) -> UART_SIG_0_SEL_R {
        UART_SIG_0_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn uart_sig_7_sel(&mut self) -> UART_SIG_7_SEL_W {
        UART_SIG_7_SEL_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn uart_sig_6_sel(&mut self) -> UART_SIG_6_SEL_W {
        UART_SIG_6_SEL_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn uart_sig_5_sel(&mut self) -> UART_SIG_5_SEL_W {
        UART_SIG_5_SEL_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn uart_sig_4_sel(&mut self) -> UART_SIG_4_SEL_W {
        UART_SIG_4_SEL_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn uart_sig_3_sel(&mut self) -> UART_SIG_3_SEL_W {
        UART_SIG_3_SEL_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn uart_sig_2_sel(&mut self) -> UART_SIG_2_SEL_W {
        UART_SIG_2_SEL_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn uart_sig_1_sel(&mut self) -> UART_SIG_1_SEL_W {
        UART_SIG_1_SEL_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn uart_sig_0_sel(&mut self) -> UART_SIG_0_SEL_W {
        UART_SIG_0_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART_SIG_SEL_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_sig_sel_0](index.html) module"]
pub struct UART_SIG_SEL_0_SPEC;
impl crate::RegisterSpec for UART_SIG_SEL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_sig_sel_0::R](R) reader structure"]
impl crate::Readable for UART_SIG_SEL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart_sig_sel_0::W](W) writer structure"]
impl crate::Writable for UART_SIG_SEL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART_SIG_SEL_0 to value 0x7654_3210"]
impl crate::Resettable for UART_SIG_SEL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7654_3210
    }
}
