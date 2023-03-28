#[doc = "Register `OTG_DOEPCTL0` reader"]
pub struct R(crate::R<OTG_DOEPCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DOEPCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DOEPCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DOEPCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_DOEPCTL0` writer"]
pub struct W(crate::W<OTG_DOEPCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_DOEPCTL0_SPEC>;
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
impl From<crate::W<OTG_DOEPCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_DOEPCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPSIZ` reader - MPSIZ"]
pub type MPSIZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBAEP` reader - USBAEP"]
pub type USBAEP_R = crate::BitReader<bool>;
#[doc = "Field `NAKSTS` reader - NAKSTS"]
pub type NAKSTS_R = crate::BitReader<bool>;
#[doc = "Field `EPTYP` reader - EPTYP"]
pub type EPTYP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNPM` reader - SNPM"]
pub type SNPM_R = crate::BitReader<bool>;
#[doc = "Field `SNPM` writer - SNPM"]
pub type SNPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPCTL0_SPEC, bool, O>;
#[doc = "Field `STALL` reader - STALL"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - STALL"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPCTL0_SPEC, bool, O>;
#[doc = "Field `CNAK` writer - CNAK"]
pub type CNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPCTL0_SPEC, bool, O>;
#[doc = "Field `SNAK` writer - SNAK"]
pub type SNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPCTL0_SPEC, bool, O>;
#[doc = "Field `EPDIS` reader - EPDIS"]
pub type EPDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPENA` writer - EPENA"]
pub type EPENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_DOEPCTL0_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USBAEP"]
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAKSTS"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - EPTYP"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - SNPM"]
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - EPDIS"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - SNPM"]
    #[inline(always)]
    #[must_use]
    pub fn snpm(&mut self) -> SNPM_W<20> {
        SNPM_W::new(self)
    }
    #[doc = "Bit 21 - STALL"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<21> {
        STALL_W::new(self)
    }
    #[doc = "Bit 26 - CNAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<26> {
        CNAK_W::new(self)
    }
    #[doc = "Bit 27 - SNAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<27> {
        SNAK_W::new(self)
    }
    #[doc = "Bit 31 - EPENA"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<31> {
        EPENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This section describes the OTG_DOEPCTL0 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_doepctl0](index.html) module"]
pub struct OTG_DOEPCTL0_SPEC;
impl crate::RegisterSpec for OTG_DOEPCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_doepctl0::R](R) reader structure"]
impl crate::Readable for OTG_DOEPCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_doepctl0::W](W) writer structure"]
impl crate::Writable for OTG_DOEPCTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_DOEPCTL0 to value 0x8000"]
impl crate::Resettable for OTG_DOEPCTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
