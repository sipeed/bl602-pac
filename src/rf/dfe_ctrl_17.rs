#[doc = "Register `dfe_ctrl_17` reader"]
pub struct R(crate::R<DFE_CTRL_17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DFE_CTRL_17_SPEC>> for R {
    fn from(reader: crate::R<DFE_CTRL_17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_17` writer"]
pub struct W(crate::W<DFE_CTRL_17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_17_SPEC>;
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
impl core::convert::From<crate::W<DFE_CTRL_17_SPEC>> for W {
    fn from(writer: crate::W<DFE_CTRL_17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `rf_tbb_ind_gc15` reader - "]
pub struct RF_TBB_IND_GC15_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC15_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc15` writer - "]
pub struct RF_TBB_IND_GC15_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc14` reader - "]
pub struct RF_TBB_IND_GC14_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC14_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc14` writer - "]
pub struct RF_TBB_IND_GC14_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc13` reader - "]
pub struct RF_TBB_IND_GC13_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC13_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc13` writer - "]
pub struct RF_TBB_IND_GC13_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc12` reader - "]
pub struct RF_TBB_IND_GC12_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC12_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc12` writer - "]
pub struct RF_TBB_IND_GC12_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc11` reader - "]
pub struct RF_TBB_IND_GC11_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC11_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc11` writer - "]
pub struct RF_TBB_IND_GC11_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc10` reader - "]
pub struct RF_TBB_IND_GC10_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC10_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc10` writer - "]
pub struct RF_TBB_IND_GC10_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc9` reader - "]
pub struct RF_TBB_IND_GC9_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC9_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc9` writer - "]
pub struct RF_TBB_IND_GC9_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `rf_tbb_ind_gc8` reader - "]
pub struct RF_TBB_IND_GC8_R(crate::FieldReader<u8, u8>);
impl RF_TBB_IND_GC8_R {
    pub(crate) fn new(bits: u8) -> Self {
        RF_TBB_IND_GC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF_TBB_IND_GC8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rf_tbb_ind_gc8` writer - "]
pub struct RF_TBB_IND_GC8_W<'a> {
    w: &'a mut W,
}
impl<'a> RF_TBB_IND_GC8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc15(&self) -> RF_TBB_IND_GC15_R {
        RF_TBB_IND_GC15_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc14(&self) -> RF_TBB_IND_GC14_R {
        RF_TBB_IND_GC14_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc13(&self) -> RF_TBB_IND_GC13_R {
        RF_TBB_IND_GC13_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc12(&self) -> RF_TBB_IND_GC12_R {
        RF_TBB_IND_GC12_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc11(&self) -> RF_TBB_IND_GC11_R {
        RF_TBB_IND_GC11_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc10(&self) -> RF_TBB_IND_GC10_R {
        RF_TBB_IND_GC10_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc9(&self) -> RF_TBB_IND_GC9_R {
        RF_TBB_IND_GC9_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc8(&self) -> RF_TBB_IND_GC8_R {
        RF_TBB_IND_GC8_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc15(&mut self) -> RF_TBB_IND_GC15_W {
        RF_TBB_IND_GC15_W { w: self }
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc14(&mut self) -> RF_TBB_IND_GC14_W {
        RF_TBB_IND_GC14_W { w: self }
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc13(&mut self) -> RF_TBB_IND_GC13_W {
        RF_TBB_IND_GC13_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc12(&mut self) -> RF_TBB_IND_GC12_W {
        RF_TBB_IND_GC12_W { w: self }
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc11(&mut self) -> RF_TBB_IND_GC11_W {
        RF_TBB_IND_GC11_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc10(&mut self) -> RF_TBB_IND_GC10_W {
        RF_TBB_IND_GC10_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc9(&mut self) -> RF_TBB_IND_GC9_W {
        RF_TBB_IND_GC9_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rf_tbb_ind_gc8(&mut self) -> RF_TBB_IND_GC8_W {
        RF_TBB_IND_GC8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_17.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_17](index.html) module"]
pub struct DFE_CTRL_17_SPEC;
impl crate::RegisterSpec for DFE_CTRL_17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_17::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_17::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_17_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dfe_ctrl_17 to value 0"]
impl crate::Resettable for DFE_CTRL_17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
