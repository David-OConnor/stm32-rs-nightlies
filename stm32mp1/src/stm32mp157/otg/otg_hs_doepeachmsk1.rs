#[doc = "Register `OTG_HS_DOEPEACHMSK1` reader"]
pub struct R(crate::R<OTG_HS_DOEPEACHMSK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HS_DOEPEACHMSK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HS_DOEPEACHMSK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HS_DOEPEACHMSK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HS_DOEPEACHMSK1` writer"]
pub struct W(crate::W<OTG_HS_DOEPEACHMSK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HS_DOEPEACHMSK1_SPEC>;
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
impl From<crate::W<OTG_HS_DOEPEACHMSK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HS_DOEPEACHMSK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFRCM` reader - XFRCM"]
pub type XFRCM_R = crate::BitReader<bool>;
#[doc = "Field `XFRCM` writer - XFRCM"]
pub type XFRCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HS_DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `EPDM` reader - EPDM"]
pub type EPDM_R = crate::BitReader<bool>;
#[doc = "Field `EPDM` writer - EPDM"]
pub type EPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HS_DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `AHBERRM` reader - AHBERRM"]
pub type AHBERRM_R = crate::BitReader<bool>;
#[doc = "Field `AHBERRM` writer - AHBERRM"]
pub type AHBERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HS_DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `STUPM` reader - STUPM"]
pub type STUPM_R = crate::BitReader<bool>;
#[doc = "Field `STUPM` writer - STUPM"]
pub type STUPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HS_DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `OTEPDM` reader - OTEPDM"]
pub type OTEPDM_R = crate::BitReader<bool>;
#[doc = "Field `OTEPDM` writer - OTEPDM"]
pub type OTEPDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HS_DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `B2BSTUPM` reader - B2BSTUPM"]
pub type B2BSTUPM_R = crate::BitReader<bool>;
#[doc = "Field `B2BSTUPM` writer - B2BSTUPM"]
pub type B2BSTUPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HS_DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `OUTPKTERRM` reader - OUTPKTERRM"]
pub type OUTPKTERRM_R = crate::BitReader<bool>;
#[doc = "Field `OUTPKTERRM` writer - OUTPKTERRM"]
pub type OUTPKTERRM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OTG_HS_DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `BNAM` reader - BNAM"]
pub type BNAM_R = crate::BitReader<bool>;
#[doc = "Field `BNAM` writer - BNAM"]
pub type BNAM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HS_DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `BERRM` reader - BERRM"]
pub type BERRM_R = crate::BitReader<bool>;
#[doc = "Field `BERRM` writer - BERRM"]
pub type BERRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HS_DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `NAKMSK` reader - NAKMSK"]
pub type NAKMSK_R = crate::BitReader<bool>;
#[doc = "Field `NAKMSK` writer - NAKMSK"]
pub type NAKMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HS_DOEPEACHMSK1_SPEC, bool, O>;
#[doc = "Field `NYETMSK` reader - NYETMSK"]
pub type NYETMSK_R = crate::BitReader<bool>;
#[doc = "Field `NYETMSK` writer - NYETMSK"]
pub type NYETMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTG_HS_DOEPEACHMSK1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    pub fn xfrcm(&self) -> XFRCM_R {
        XFRCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPDM"]
    #[inline(always)]
    pub fn epdm(&self) -> EPDM_R {
        EPDM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    pub fn ahberrm(&self) -> AHBERRM_R {
        AHBERRM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STUPM"]
    #[inline(always)]
    pub fn stupm(&self) -> STUPM_R {
        STUPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OTEPDM"]
    #[inline(always)]
    pub fn otepdm(&self) -> OTEPDM_R {
        OTEPDM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - B2BSTUPM"]
    #[inline(always)]
    pub fn b2bstupm(&self) -> B2BSTUPM_R {
        B2BSTUPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUTPKTERRM"]
    #[inline(always)]
    pub fn outpkterrm(&self) -> OUTPKTERRM_R {
        OUTPKTERRM_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BNAM"]
    #[inline(always)]
    pub fn bnam(&self) -> BNAM_R {
        BNAM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - BERRM"]
    #[inline(always)]
    pub fn berrm(&self) -> BERRM_R {
        BERRM_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAKMSK"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - NYETMSK"]
    #[inline(always)]
    pub fn nyetmsk(&self) -> NYETMSK_R {
        NYETMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XFRCM"]
    #[inline(always)]
    #[must_use]
    pub fn xfrcm(&mut self) -> XFRCM_W<0> {
        XFRCM_W::new(self)
    }
    #[doc = "Bit 1 - EPDM"]
    #[inline(always)]
    #[must_use]
    pub fn epdm(&mut self) -> EPDM_W<1> {
        EPDM_W::new(self)
    }
    #[doc = "Bit 2 - AHBERRM"]
    #[inline(always)]
    #[must_use]
    pub fn ahberrm(&mut self) -> AHBERRM_W<2> {
        AHBERRM_W::new(self)
    }
    #[doc = "Bit 3 - STUPM"]
    #[inline(always)]
    #[must_use]
    pub fn stupm(&mut self) -> STUPM_W<3> {
        STUPM_W::new(self)
    }
    #[doc = "Bit 4 - OTEPDM"]
    #[inline(always)]
    #[must_use]
    pub fn otepdm(&mut self) -> OTEPDM_W<4> {
        OTEPDM_W::new(self)
    }
    #[doc = "Bit 6 - B2BSTUPM"]
    #[inline(always)]
    #[must_use]
    pub fn b2bstupm(&mut self) -> B2BSTUPM_W<6> {
        B2BSTUPM_W::new(self)
    }
    #[doc = "Bit 8 - OUTPKTERRM"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterrm(&mut self) -> OUTPKTERRM_W<8> {
        OUTPKTERRM_W::new(self)
    }
    #[doc = "Bit 9 - BNAM"]
    #[inline(always)]
    #[must_use]
    pub fn bnam(&mut self) -> BNAM_W<9> {
        BNAM_W::new(self)
    }
    #[doc = "Bit 12 - BERRM"]
    #[inline(always)]
    #[must_use]
    pub fn berrm(&mut self) -> BERRM_W<12> {
        BERRM_W::new(self)
    }
    #[doc = "Bit 13 - NAKMSK"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<13> {
        NAKMSK_W::new(self)
    }
    #[doc = "Bit 14 - NYETMSK"]
    #[inline(always)]
    #[must_use]
    pub fn nyetmsk(&mut self) -> NYETMSK_W<14> {
        NYETMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register works with the OTG_DOEPINT1 register to generate a dedicated interrupt OTG_HS_EP1_OUT for endpoint #1. The OUT endpoint interrupt for a specific status in the OTG_DOEPINT1 register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_doepeachmsk1](index.html) module"]
pub struct OTG_HS_DOEPEACHMSK1_SPEC;
impl crate::RegisterSpec for OTG_HS_DOEPEACHMSK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hs_doepeachmsk1::R](R) reader structure"]
impl crate::Readable for OTG_HS_DOEPEACHMSK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hs_doepeachmsk1::W](W) writer structure"]
impl crate::Writable for OTG_HS_DOEPEACHMSK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTG_HS_DOEPEACHMSK1 to value 0"]
impl crate::Resettable for OTG_HS_DOEPEACHMSK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
