#[doc = "Register `SFR` reader"]
pub struct R(crate::R<SFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SFR` writer"]
pub struct W(crate::W<SFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SFR_SPEC>;
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
impl From<crate::W<SFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SFSA` reader - Secure flash start address"]
pub type SFSA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SFSA` writer - Secure flash start address"]
pub type SFSA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SFR_SPEC, u8, u8, 8, O>;
#[doc = "Field `FSD` reader - Flash security disable"]
pub type FSD_R = crate::BitReader<bool>;
#[doc = "Field `FSD` writer - Flash security disable"]
pub type FSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, bool, O>;
#[doc = "Field `DDS` reader - Disable Cortex M0 debug access"]
pub type DDS_R = crate::BitReader<bool>;
#[doc = "Field `DDS` writer - Disable Cortex M0 debug access"]
pub type DDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SFR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Secure flash start address"]
    #[inline(always)]
    pub fn sfsa(&self) -> SFSA_R {
        SFSA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Flash security disable"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Disable Cortex M0 debug access"]
    #[inline(always)]
    pub fn dds(&self) -> DDS_R {
        DDS_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secure flash start address"]
    #[inline(always)]
    #[must_use]
    pub fn sfsa(&mut self) -> SFSA_W<0> {
        SFSA_W::new(self)
    }
    #[doc = "Bit 8 - Flash security disable"]
    #[inline(always)]
    #[must_use]
    pub fn fsd(&mut self) -> FSD_W<8> {
        FSD_W::new(self)
    }
    #[doc = "Bit 12 - Disable Cortex M0 debug access"]
    #[inline(always)]
    #[must_use]
    pub fn dds(&mut self) -> DDS_W<12> {
        DDS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Secure flash start address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sfr](index.html) module"]
pub struct SFR_SPEC;
impl crate::RegisterSpec for SFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sfr::R](R) reader structure"]
impl crate::Readable for SFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sfr::W](W) writer structure"]
impl crate::Writable for SFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SFR to value 0xffff_ee00"]
impl crate::Resettable for SFR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ee00;
}
