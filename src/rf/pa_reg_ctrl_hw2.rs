#[doc = "Register `pa_reg_ctrl_hw2` reader"]
pub struct R(crate::R<PA_REG_CTRL_HW2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PA_REG_CTRL_HW2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PA_REG_CTRL_HW2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PA_REG_CTRL_HW2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pa_reg_ctrl_hw2` writer"]
pub struct W(crate::W<PA_REG_CTRL_HW2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PA_REG_CTRL_HW2_SPEC>;
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
impl From<crate::W<PA_REG_CTRL_HW2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PA_REG_CTRL_HW2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pa_vbcas_11b` reader - "]
pub struct PA_VBCAS_11B_R(crate::FieldReader<u8, u8>);
impl PA_VBCAS_11B_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_VBCAS_11B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_VBCAS_11B_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_vbcas_11b` writer - "]
pub struct PA_VBCAS_11B_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCAS_11B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `pa_vbcore_11b` reader - "]
pub struct PA_VBCORE_11B_R(crate::FieldReader<u8, u8>);
impl PA_VBCORE_11B_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_VBCORE_11B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_VBCORE_11B_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_vbcore_11b` writer - "]
pub struct PA_VBCORE_11B_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCORE_11B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `pa_iet_11b` reader - "]
pub struct PA_IET_11B_R(crate::FieldReader<u8, u8>);
impl PA_IET_11B_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_IET_11B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_IET_11B_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_iet_11b` writer - "]
pub struct PA_IET_11B_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IET_11B_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `pa_vbcas_11g` reader - "]
pub struct PA_VBCAS_11G_R(crate::FieldReader<u8, u8>);
impl PA_VBCAS_11G_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_VBCAS_11G_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_VBCAS_11G_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_vbcas_11g` writer - "]
pub struct PA_VBCAS_11G_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCAS_11G_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `pa_vbcore_11g` reader - "]
pub struct PA_VBCORE_11G_R(crate::FieldReader<u8, u8>);
impl PA_VBCORE_11G_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_VBCORE_11G_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_VBCORE_11G_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_vbcore_11g` writer - "]
pub struct PA_VBCORE_11G_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_VBCORE_11G_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `pa_iet_11g` reader - "]
pub struct PA_IET_11G_R(crate::FieldReader<u8, u8>);
impl PA_IET_11G_R {
    pub(crate) fn new(bits: u8) -> Self {
        PA_IET_11G_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PA_IET_11G_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pa_iet_11g` writer - "]
pub struct PA_IET_11G_W<'a> {
    w: &'a mut W,
}
impl<'a> PA_IET_11G_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11b(&self) -> PA_VBCAS_11B_R {
        PA_VBCAS_11B_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11b(&self) -> PA_VBCORE_11B_R {
        PA_VBCORE_11B_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11b(&self) -> PA_IET_11B_R {
        PA_IET_11B_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pa_vbcas_11g(&self) -> PA_VBCAS_11G_R {
        PA_VBCAS_11G_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_vbcore_11g(&self) -> PA_VBCORE_11G_R {
        PA_VBCORE_11G_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pa_iet_11g(&self) -> PA_IET_11G_R {
        PA_IET_11G_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn pa_vbcas_11b(&mut self) -> PA_VBCAS_11B_W {
        PA_VBCAS_11B_W { w: self }
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pa_vbcore_11b(&mut self) -> PA_VBCORE_11B_W {
        PA_VBCORE_11B_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn pa_iet_11b(&mut self) -> PA_IET_11B_W {
        PA_IET_11B_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn pa_vbcas_11g(&mut self) -> PA_VBCAS_11G_W {
        PA_VBCAS_11G_W { w: self }
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn pa_vbcore_11g(&mut self) -> PA_VBCORE_11G_W {
        PA_VBCORE_11G_W { w: self }
    }
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pa_iet_11g(&mut self) -> PA_IET_11G_W {
        PA_IET_11G_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "pa_reg_ctrl_hw2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pa_reg_ctrl_hw2](index.html) module"]
pub struct PA_REG_CTRL_HW2_SPEC;
impl crate::RegisterSpec for PA_REG_CTRL_HW2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pa_reg_ctrl_hw2::R](R) reader structure"]
impl crate::Readable for PA_REG_CTRL_HW2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pa_reg_ctrl_hw2::W](W) writer structure"]
impl crate::Writable for PA_REG_CTRL_HW2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pa_reg_ctrl_hw2 to value 0"]
impl crate::Resettable for PA_REG_CTRL_HW2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
