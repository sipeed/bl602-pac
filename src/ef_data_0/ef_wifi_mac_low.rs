#[doc = "Register `ef_wifi_mac_low` reader"]
pub struct R(crate::R<EF_WIFI_MAC_LOW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EF_WIFI_MAC_LOW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EF_WIFI_MAC_LOW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EF_WIFI_MAC_LOW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ef_wifi_mac_low` writer"]
pub struct W(crate::W<EF_WIFI_MAC_LOW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EF_WIFI_MAC_LOW_SPEC>;
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
impl From<crate::W<EF_WIFI_MAC_LOW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EF_WIFI_MAC_LOW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ef_wifi_mac_low` reader - "]
pub struct EF_WIFI_MAC_LOW_R(crate::FieldReader<u32, u32>);
impl EF_WIFI_MAC_LOW_R {
    pub(crate) fn new(bits: u32) -> Self {
        EF_WIFI_MAC_LOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EF_WIFI_MAC_LOW_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ef_wifi_mac_low` writer - "]
pub struct EF_WIFI_MAC_LOW_W<'a> {
    w: &'a mut W,
}
impl<'a> EF_WIFI_MAC_LOW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_wifi_mac_low(&self) -> EF_WIFI_MAC_LOW_R {
        EF_WIFI_MAC_LOW_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ef_wifi_mac_low(&mut self) -> EF_WIFI_MAC_LOW_W {
        EF_WIFI_MAC_LOW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ef_wifi_mac_low.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ef_wifi_mac_low](index.html) module"]
pub struct EF_WIFI_MAC_LOW_SPEC;
impl crate::RegisterSpec for EF_WIFI_MAC_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ef_wifi_mac_low::R](R) reader structure"]
impl crate::Readable for EF_WIFI_MAC_LOW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ef_wifi_mac_low::W](W) writer structure"]
impl crate::Writable for EF_WIFI_MAC_LOW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ef_wifi_mac_low to value 0"]
impl crate::Resettable for EF_WIFI_MAC_LOW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
