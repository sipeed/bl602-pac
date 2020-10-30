#[doc = "Register `irtx_pw` reader"]
pub struct R(crate::R<IRTX_PW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRTX_PW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IRTX_PW_SPEC>> for R {
    fn from(reader: crate::R<IRTX_PW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `irtx_pw` writer"]
pub struct W(crate::W<IRTX_PW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRTX_PW_SPEC>;
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
impl core::convert::From<crate::W<IRTX_PW_SPEC>> for W {
    fn from(writer: crate::W<IRTX_PW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_irtx_tail_ph1_w` reader - "]
pub struct CR_IRTX_TAIL_PH1_W_R(crate::FieldReader<u8, u8>);
impl CR_IRTX_TAIL_PH1_W_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRTX_TAIL_PH1_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_TAIL_PH1_W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_tail_ph1_w` writer - "]
pub struct CR_IRTX_TAIL_PH1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_TAIL_PH1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
#[doc = "Field `cr_irtx_tail_ph0_w` reader - "]
pub struct CR_IRTX_TAIL_PH0_W_R(crate::FieldReader<u8, u8>);
impl CR_IRTX_TAIL_PH0_W_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRTX_TAIL_PH0_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_TAIL_PH0_W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_tail_ph0_w` writer - "]
pub struct CR_IRTX_TAIL_PH0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_TAIL_PH0_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `cr_irtx_head_ph1_w` reader - "]
pub struct CR_IRTX_HEAD_PH1_W_R(crate::FieldReader<u8, u8>);
impl CR_IRTX_HEAD_PH1_W_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRTX_HEAD_PH1_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_HEAD_PH1_W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_head_ph1_w` writer - "]
pub struct CR_IRTX_HEAD_PH1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_HEAD_PH1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `cr_irtx_head_ph0_w` reader - "]
pub struct CR_IRTX_HEAD_PH0_W_R(crate::FieldReader<u8, u8>);
impl CR_IRTX_HEAD_PH0_W_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRTX_HEAD_PH0_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_HEAD_PH0_W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_head_ph0_w` writer - "]
pub struct CR_IRTX_HEAD_PH0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_HEAD_PH0_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `cr_irtx_logic1_ph1_w` reader - "]
pub struct CR_IRTX_LOGIC1_PH1_W_R(crate::FieldReader<u8, u8>);
impl CR_IRTX_LOGIC1_PH1_W_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRTX_LOGIC1_PH1_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_LOGIC1_PH1_W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_logic1_ph1_w` writer - "]
pub struct CR_IRTX_LOGIC1_PH1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC1_PH1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `cr_irtx_logic1_ph0_w` reader - "]
pub struct CR_IRTX_LOGIC1_PH0_W_R(crate::FieldReader<u8, u8>);
impl CR_IRTX_LOGIC1_PH0_W_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRTX_LOGIC1_PH0_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_LOGIC1_PH0_W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_logic1_ph0_w` writer - "]
pub struct CR_IRTX_LOGIC1_PH0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC1_PH0_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `cr_irtx_logic0_ph1_w` reader - "]
pub struct CR_IRTX_LOGIC0_PH1_W_R(crate::FieldReader<u8, u8>);
impl CR_IRTX_LOGIC0_PH1_W_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRTX_LOGIC0_PH1_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_LOGIC0_PH1_W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_logic0_ph1_w` writer - "]
pub struct CR_IRTX_LOGIC0_PH1_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC0_PH1_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `cr_irtx_logic0_ph0_w` reader - "]
pub struct CR_IRTX_LOGIC0_PH0_W_R(crate::FieldReader<u8, u8>);
impl CR_IRTX_LOGIC0_PH0_W_R {
    pub(crate) fn new(bits: u8) -> Self {
        CR_IRTX_LOGIC0_PH0_W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CR_IRTX_LOGIC0_PH0_W_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cr_irtx_logic0_ph0_w` writer - "]
pub struct CR_IRTX_LOGIC0_PH0_W_W<'a> {
    w: &'a mut W,
}
impl<'a> CR_IRTX_LOGIC0_PH0_W_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph1_w(&self) -> CR_IRTX_TAIL_PH1_W_R {
        CR_IRTX_TAIL_PH1_W_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph0_w(&self) -> CR_IRTX_TAIL_PH0_W_R {
        CR_IRTX_TAIL_PH0_W_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn cr_irtx_head_ph1_w(&self) -> CR_IRTX_HEAD_PH1_W_R {
        CR_IRTX_HEAD_PH1_W_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cr_irtx_head_ph0_w(&self) -> CR_IRTX_HEAD_PH0_W_R {
        CR_IRTX_HEAD_PH0_W_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph1_w(&self) -> CR_IRTX_LOGIC1_PH1_W_R {
        CR_IRTX_LOGIC1_PH1_W_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph0_w(&self) -> CR_IRTX_LOGIC1_PH0_W_R {
        CR_IRTX_LOGIC1_PH0_W_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph1_w(&self) -> CR_IRTX_LOGIC0_PH1_W_R {
        CR_IRTX_LOGIC0_PH1_W_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph0_w(&self) -> CR_IRTX_LOGIC0_PH0_W_R {
        CR_IRTX_LOGIC0_PH0_W_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph1_w(&mut self) -> CR_IRTX_TAIL_PH1_W_W {
        CR_IRTX_TAIL_PH1_W_W { w: self }
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn cr_irtx_tail_ph0_w(&mut self) -> CR_IRTX_TAIL_PH0_W_W {
        CR_IRTX_TAIL_PH0_W_W { w: self }
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn cr_irtx_head_ph1_w(&mut self) -> CR_IRTX_HEAD_PH1_W_W {
        CR_IRTX_HEAD_PH1_W_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn cr_irtx_head_ph0_w(&mut self) -> CR_IRTX_HEAD_PH0_W_W {
        CR_IRTX_HEAD_PH0_W_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph1_w(&mut self) -> CR_IRTX_LOGIC1_PH1_W_W {
        CR_IRTX_LOGIC1_PH1_W_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn cr_irtx_logic1_ph0_w(&mut self) -> CR_IRTX_LOGIC1_PH0_W_W {
        CR_IRTX_LOGIC1_PH0_W_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph1_w(&mut self) -> CR_IRTX_LOGIC0_PH1_W_W {
        CR_IRTX_LOGIC0_PH1_W_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn cr_irtx_logic0_ph0_w(&mut self) -> CR_IRTX_LOGIC0_PH0_W_W {
        CR_IRTX_LOGIC0_PH0_W_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "irtx_pw.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irtx_pw](index.html) module"]
pub struct IRTX_PW_SPEC;
impl crate::RegisterSpec for IRTX_PW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irtx_pw::R](R) reader structure"]
impl crate::Readable for IRTX_PW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irtx_pw::W](W) writer structure"]
impl crate::Writable for IRTX_PW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets irtx_pw to value 0"]
impl crate::Resettable for IRTX_PW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
