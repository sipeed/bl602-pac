#[doc = "Register `HBN_PIR_VTH` reader"]
pub struct R(crate::R<HBN_PIR_VTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HBN_PIR_VTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HBN_PIR_VTH_SPEC>> for R {
    fn from(reader: crate::R<HBN_PIR_VTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HBN_PIR_VTH` writer"]
pub struct W(crate::W<HBN_PIR_VTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HBN_PIR_VTH_SPEC>;
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
impl core::convert::From<crate::W<HBN_PIR_VTH_SPEC>> for W {
    fn from(writer: crate::W<HBN_PIR_VTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pir_vth` reader - "]
pub struct PIR_VTH_R(crate::FieldReader<u16, u16>);
impl PIR_VTH_R {
    pub(crate) fn new(bits: u16) -> Self {
        PIR_VTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIR_VTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pir_vth` writer - "]
pub struct PIR_VTH_W<'a> {
    w: &'a mut W,
}
impl<'a> PIR_VTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pir_vth(&self) -> PIR_VTH_R {
        PIR_VTH_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn pir_vth(&mut self) -> PIR_VTH_W {
        PIR_VTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HBN_PIR_VTH.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hbn_pir_vth](index.html) module"]
pub struct HBN_PIR_VTH_SPEC;
impl crate::RegisterSpec for HBN_PIR_VTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hbn_pir_vth::R](R) reader structure"]
impl crate::Readable for HBN_PIR_VTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hbn_pir_vth::W](W) writer structure"]
impl crate::Writable for HBN_PIR_VTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HBN_PIR_VTH to value 0x03ff"]
impl crate::Resettable for HBN_PIR_VTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
