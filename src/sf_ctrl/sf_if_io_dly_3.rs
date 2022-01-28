#[doc = "Register `sf_if_io_dly_3` reader"]
pub struct R(crate::R<SF_IF_IO_DLY_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SF_IF_IO_DLY_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SF_IF_IO_DLY_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SF_IF_IO_DLY_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `sf_if_io_dly_3` writer"]
pub struct W(crate::W<SF_IF_IO_DLY_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SF_IF_IO_DLY_3_SPEC>;
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
impl From<crate::W<SF_IF_IO_DLY_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SF_IF_IO_DLY_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sf_io_2_do_dly_sel` reader - "]
pub struct SF_IO_2_DO_DLY_SEL_R(crate::FieldReader<u8, u8>);
impl SF_IO_2_DO_DLY_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IO_2_DO_DLY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IO_2_DO_DLY_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_io_2_do_dly_sel` writer - "]
pub struct SF_IO_2_DO_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IO_2_DO_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `sf_io_2_di_dly_sel` reader - "]
pub struct SF_IO_2_DI_DLY_SEL_R(crate::FieldReader<u8, u8>);
impl SF_IO_2_DI_DLY_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IO_2_DI_DLY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IO_2_DI_DLY_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_io_2_di_dly_sel` writer - "]
pub struct SF_IO_2_DI_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IO_2_DI_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "Field `sf_io_2_oe_dly_sel` reader - "]
pub struct SF_IO_2_OE_DLY_SEL_R(crate::FieldReader<u8, u8>);
impl SF_IO_2_OE_DLY_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SF_IO_2_OE_DLY_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SF_IO_2_OE_DLY_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sf_io_2_oe_dly_sel` writer - "]
pub struct SF_IO_2_OE_DLY_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SF_IO_2_OE_DLY_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf_io_2_do_dly_sel(&self) -> SF_IO_2_DO_DLY_SEL_R {
        SF_IO_2_DO_DLY_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_2_di_dly_sel(&self) -> SF_IO_2_DI_DLY_SEL_R {
        SF_IO_2_DI_DLY_SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_2_oe_dly_sel(&self) -> SF_IO_2_OE_DLY_SEL_R {
        SF_IO_2_OE_DLY_SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sf_io_2_do_dly_sel(&mut self) -> SF_IO_2_DO_DLY_SEL_W {
        SF_IO_2_DO_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sf_io_2_di_dly_sel(&mut self) -> SF_IO_2_DI_DLY_SEL_W {
        SF_IO_2_DI_DLY_SEL_W { w: self }
    }
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sf_io_2_oe_dly_sel(&mut self) -> SF_IO_2_OE_DLY_SEL_W {
        SF_IO_2_OE_DLY_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sf_if_io_dly_3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sf_if_io_dly_3](index.html) module"]
pub struct SF_IF_IO_DLY_3_SPEC;
impl crate::RegisterSpec for SF_IF_IO_DLY_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sf_if_io_dly_3::R](R) reader structure"]
impl crate::Readable for SF_IF_IO_DLY_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sf_if_io_dly_3::W](W) writer structure"]
impl crate::Writable for SF_IF_IO_DLY_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets sf_if_io_dly_3 to value 0"]
impl crate::Resettable for SF_IF_IO_DLY_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
