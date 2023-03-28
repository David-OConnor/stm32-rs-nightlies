#[doc = "Register `RCC_RDLSICR` reader"]
pub struct R(crate::R<RCC_RDLSICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_RDLSICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_RDLSICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_RDLSICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_RDLSICR` writer"]
pub struct W(crate::W<RCC_RDLSICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_RDLSICR_SPEC>;
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
impl From<crate::W<RCC_RDLSICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_RDLSICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSION` reader - LSION"]
pub type LSION_R = crate::BitReader<bool>;
#[doc = "Field `LSION` writer - LSION"]
pub type LSION_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_RDLSICR_SPEC, bool, O>;
#[doc = "Field `LSIRDY` reader - LSIRDY"]
pub type LSIRDY_R = crate::BitReader<bool>;
#[doc = "Field `MRD` reader - MRD"]
pub type MRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MRD` writer - MRD"]
pub type MRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_RDLSICR_SPEC, u8, u8, 5, O>;
#[doc = "Field `EADLY` reader - EADLY"]
pub type EADLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EADLY` writer - EADLY"]
pub type EADLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_RDLSICR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SPARE` reader - SPARE"]
pub type SPARE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE` writer - SPARE"]
pub type SPARE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_RDLSICR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - LSION"]
    #[inline(always)]
    pub fn lsion(&self) -> LSION_R {
        LSION_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSIRDY"]
    #[inline(always)]
    pub fn lsirdy(&self) -> LSIRDY_R {
        LSIRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:20 - MRD"]
    #[inline(always)]
    pub fn mrd(&self) -> MRD_R {
        MRD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - EADLY"]
    #[inline(always)]
    pub fn eadly(&self) -> EADLY_R {
        EADLY_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:31 - SPARE"]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LSION"]
    #[inline(always)]
    #[must_use]
    pub fn lsion(&mut self) -> LSION_W<0> {
        LSION_W::new(self)
    }
    #[doc = "Bits 16:20 - MRD"]
    #[inline(always)]
    #[must_use]
    pub fn mrd(&mut self) -> MRD_W<16> {
        MRD_W::new(self)
    }
    #[doc = "Bits 24:26 - EADLY"]
    #[inline(always)]
    #[must_use]
    pub fn eadly(&mut self) -> EADLY_W<24> {
        EADLY_W::new(self)
    }
    #[doc = "Bits 27:31 - SPARE"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<27> {
        SPARE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the minimum NRST active duration and LSI function.0 to 7 wait states are inserted for word, half-word and byte accesses. Wait states are inserted in case of successive accesses to this register.This register is reset by the por_rst reset, and it is located into the VDD domain. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rdlsicr](index.html) module"]
pub struct RCC_RDLSICR_SPEC;
impl crate::RegisterSpec for RCC_RDLSICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_rdlsicr::R](R) reader structure"]
impl crate::Readable for RCC_RDLSICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_rdlsicr::W](W) writer structure"]
impl crate::Writable for RCC_RDLSICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCC_RDLSICR to value 0"]
impl crate::Resettable for RCC_RDLSICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
