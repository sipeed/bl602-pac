#[doc = "Register `spi_prd_0` reader"]
pub struct R(crate::R<SPI_PRD_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_PRD_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_PRD_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_PRD_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `spi_prd_0` writer"]
pub struct W(crate::W<SPI_PRD_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_PRD_0_SPEC>;
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
impl From<crate::W<SPI_PRD_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_PRD_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `cr_spi_prd_s` reader - "]
pub type CR_SPI_PRD_S_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_spi_prd_s` writer - "]
pub type CR_SPI_PRD_S_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_PRD_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `cr_spi_prd_p` reader - "]
pub type CR_SPI_PRD_P_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_spi_prd_p` writer - "]
pub type CR_SPI_PRD_P_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_PRD_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `cr_spi_prd_d_ph_0` reader - "]
pub type CR_SPI_PRD_D_PH_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_spi_prd_d_ph_0` writer - "]
pub type CR_SPI_PRD_D_PH_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_PRD_0_SPEC, u8, u8, 8, O>;
#[doc = "Field `cr_spi_prd_d_ph_1` reader - "]
pub type CR_SPI_PRD_D_PH_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `cr_spi_prd_d_ph_1` writer - "]
pub type CR_SPI_PRD_D_PH_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_PRD_0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn cr_spi_prd_s(&self) -> CR_SPI_PRD_S_R {
        CR_SPI_PRD_S_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn cr_spi_prd_p(&self) -> CR_SPI_PRD_P_R {
        CR_SPI_PRD_P_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_0(&self) -> CR_SPI_PRD_D_PH_0_R {
        CR_SPI_PRD_D_PH_0_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn cr_spi_prd_d_ph_1(&self) -> CR_SPI_PRD_D_PH_1_R {
        CR_SPI_PRD_D_PH_1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_prd_s(&mut self) -> CR_SPI_PRD_S_W<0> {
        CR_SPI_PRD_S_W::new(self)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_prd_p(&mut self) -> CR_SPI_PRD_P_W<8> {
        CR_SPI_PRD_P_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_prd_d_ph_0(&mut self) -> CR_SPI_PRD_D_PH_0_W<16> {
        CR_SPI_PRD_D_PH_0_W::new(self)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn cr_spi_prd_d_ph_1(&mut self) -> CR_SPI_PRD_D_PH_1_W<24> {
        CR_SPI_PRD_D_PH_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "spi_prd_0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_prd_0](index.html) module"]
pub struct SPI_PRD_0_SPEC;
impl crate::RegisterSpec for SPI_PRD_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_prd_0::R](R) reader structure"]
impl crate::Readable for SPI_PRD_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_prd_0::W](W) writer structure"]
impl crate::Writable for SPI_PRD_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets spi_prd_0 to value 0x0f0f_0f0f"]
impl crate::Resettable for SPI_PRD_0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0f_0f0f;
}
