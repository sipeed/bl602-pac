#[doc = "Register `cip` reader"]
pub struct R(crate::R<CIP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CIP_SPEC>> for R {
    fn from(reader: crate::R<CIP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `cip` writer"]
pub struct W(crate::W<CIP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIP_SPEC>;
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
impl core::convert::From<crate::W<CIP_SPEC>> for W {
    fn from(writer: crate::W<CIP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `vg13_sel` reader - "]
pub struct VG13_SEL_R(crate::FieldReader<u8, u8>);
impl VG13_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VG13_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VG13_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vg13_sel` writer - "]
pub struct VG13_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VG13_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `vg11_sel` reader - "]
pub struct VG11_SEL_R(crate::FieldReader<u8, u8>);
impl VG11_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VG11_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VG11_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vg11_sel` writer - "]
pub struct VG11_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VG11_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vg13_sel(&self) -> VG13_SEL_R {
        VG13_SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vg11_sel(&self) -> VG11_SEL_R {
        VG11_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn vg13_sel(&mut self) -> VG13_SEL_W {
        VG13_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn vg11_sel(&mut self) -> VG11_SEL_W {
        VG11_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RX normal bias mode registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cip](index.html) module"]
pub struct CIP_SPEC;
impl crate::RegisterSpec for CIP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cip::R](R) reader structure"]
impl crate::Readable for CIP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cip::W](W) writer structure"]
impl crate::Writable for CIP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets cip to value 0"]
impl crate::Resettable for CIP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
