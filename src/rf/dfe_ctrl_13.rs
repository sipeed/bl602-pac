#[doc = "Register `dfe_ctrl_13` reader"]
pub struct R(crate::R<DFE_CTRL_13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFE_CTRL_13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFE_CTRL_13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFE_CTRL_13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `dfe_ctrl_13` writer"]
pub struct W(crate::W<DFE_CTRL_13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFE_CTRL_13_SPEC>;
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
impl From<crate::W<DFE_CTRL_13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFE_CTRL_13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_dvga_gain_qdb_gc7` reader - "]
pub struct TX_DVGA_GAIN_QDB_GC7_R(crate::FieldReader<u8, u8>);
impl TX_DVGA_GAIN_QDB_GC7_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_DVGA_GAIN_QDB_GC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DVGA_GAIN_QDB_GC7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dvga_gain_qdb_gc7` writer - "]
pub struct TX_DVGA_GAIN_QDB_GC7_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | ((value as u32 & 0x7f) << 24);
        self.w
    }
}
#[doc = "Field `tx_dvga_gain_qdb_gc6` reader - "]
pub struct TX_DVGA_GAIN_QDB_GC6_R(crate::FieldReader<u8, u8>);
impl TX_DVGA_GAIN_QDB_GC6_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_DVGA_GAIN_QDB_GC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DVGA_GAIN_QDB_GC6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dvga_gain_qdb_gc6` writer - "]
pub struct TX_DVGA_GAIN_QDB_GC6_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `tx_dvga_gain_qdb_gc5` reader - "]
pub struct TX_DVGA_GAIN_QDB_GC5_R(crate::FieldReader<u8, u8>);
impl TX_DVGA_GAIN_QDB_GC5_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_DVGA_GAIN_QDB_GC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DVGA_GAIN_QDB_GC5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dvga_gain_qdb_gc5` writer - "]
pub struct TX_DVGA_GAIN_QDB_GC5_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `tx_dvga_gain_qdb_gc4` reader - "]
pub struct TX_DVGA_GAIN_QDB_GC4_R(crate::FieldReader<u8, u8>);
impl TX_DVGA_GAIN_QDB_GC4_R {
    pub(crate) fn new(bits: u8) -> Self {
        TX_DVGA_GAIN_QDB_GC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DVGA_GAIN_QDB_GC4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_dvga_gain_qdb_gc4` writer - "]
pub struct TX_DVGA_GAIN_QDB_GC4_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DVGA_GAIN_QDB_GC4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc7(&self) -> TX_DVGA_GAIN_QDB_GC7_R {
        TX_DVGA_GAIN_QDB_GC7_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc6(&self) -> TX_DVGA_GAIN_QDB_GC6_R {
        TX_DVGA_GAIN_QDB_GC6_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc5(&self) -> TX_DVGA_GAIN_QDB_GC5_R {
        TX_DVGA_GAIN_QDB_GC5_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc4(&self) -> TX_DVGA_GAIN_QDB_GC4_R {
        TX_DVGA_GAIN_QDB_GC4_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc7(&mut self) -> TX_DVGA_GAIN_QDB_GC7_W {
        TX_DVGA_GAIN_QDB_GC7_W { w: self }
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc6(&mut self) -> TX_DVGA_GAIN_QDB_GC6_W {
        TX_DVGA_GAIN_QDB_GC6_W { w: self }
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc5(&mut self) -> TX_DVGA_GAIN_QDB_GC5_W {
        TX_DVGA_GAIN_QDB_GC5_W { w: self }
    }
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn tx_dvga_gain_qdb_gc4(&mut self) -> TX_DVGA_GAIN_QDB_GC4_W {
        TX_DVGA_GAIN_QDB_GC4_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "dfe_ctrl_13.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfe_ctrl_13](index.html) module"]
pub struct DFE_CTRL_13_SPEC;
impl crate::RegisterSpec for DFE_CTRL_13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfe_ctrl_13::R](R) reader structure"]
impl crate::Readable for DFE_CTRL_13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfe_ctrl_13::W](W) writer structure"]
impl crate::Writable for DFE_CTRL_13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets dfe_ctrl_13 to value 0"]
impl crate::Resettable for DFE_CTRL_13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
