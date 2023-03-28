#[doc = "Register `SBS_PMCR` reader"]
pub struct R(crate::R<SBS_PMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBS_PMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBS_PMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBS_PMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBS_PMCR` writer"]
pub struct W(crate::W<SBS_PMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBS_PMCR_SPEC>;
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
impl From<crate::W<SBS_PMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBS_PMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOSTEN` reader - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
pub type BOOSTEN_R = crate::BitReader<bool>;
#[doc = "Field `BOOSTEN` writer - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
pub type BOOSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_PMCR_SPEC, bool, O>;
#[doc = "Field `BOOSTVDDSEL` reader - booster VDD selection Note: Booster must not be used when VDDA < 2.7 V, but VDD > 2.7 V (add current consumption). When both VDD < 2.7 V and VDDA < 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
pub type BOOSTVDDSEL_R = crate::BitReader<bool>;
#[doc = "Field `BOOSTVDDSEL` writer - booster VDD selection Note: Booster must not be used when VDDA < 2.7 V, but VDD > 2.7 V (add current consumption). When both VDD < 2.7 V and VDDA < 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
pub type BOOSTVDDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_PMCR_SPEC, bool, O>;
#[doc = "Field `PB6_FMPLUS` reader - Fast-mode Plus command on PB(6)"]
pub type PB6_FMPLUS_R = crate::BitReader<bool>;
#[doc = "Field `PB6_FMPLUS` writer - Fast-mode Plus command on PB(6)"]
pub type PB6_FMPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_PMCR_SPEC, bool, O>;
#[doc = "Field `PB7_FMPLUS` reader - Fast-mode Plus command on PB(7)"]
pub type PB7_FMPLUS_R = crate::BitReader<bool>;
#[doc = "Field `PB7_FMPLUS` writer - Fast-mode Plus command on PB(7)"]
pub type PB7_FMPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_PMCR_SPEC, bool, O>;
#[doc = "Field `PB8_FMPLUS` reader - Fast-mode Plus command on PB(8)"]
pub type PB8_FMPLUS_R = crate::BitReader<bool>;
#[doc = "Field `PB8_FMPLUS` writer - Fast-mode Plus command on PB(8)"]
pub type PB8_FMPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_PMCR_SPEC, bool, O>;
#[doc = "Field `PB9_FMPLUS` reader - Fast-mode Plus command on PB(9)"]
pub type PB9_FMPLUS_R = crate::BitReader<bool>;
#[doc = "Field `PB9_FMPLUS` writer - Fast-mode Plus command on PB(9)"]
pub type PB9_FMPLUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SBS_PMCR_SPEC, bool, O>;
#[doc = "Field `ETH_SEL_PHY` reader - Ethernet PHY interface selection Other: reserved"]
pub type ETH_SEL_PHY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ETH_SEL_PHY` writer - Ethernet PHY interface selection Other: reserved"]
pub type ETH_SEL_PHY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SBS_PMCR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - booster VDD selection Note: Booster must not be used when VDDA < 2.7 V, but VDD > 2.7 V (add current consumption). When both VDD < 2.7 V and VDDA < 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
    #[inline(always)]
    pub fn boostvddsel(&self) -> BOOSTVDDSEL_R {
        BOOSTVDDSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast-mode Plus command on PB(6)"]
    #[inline(always)]
    pub fn pb6_fmplus(&self) -> PB6_FMPLUS_R {
        PB6_FMPLUS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast-mode Plus command on PB(7)"]
    #[inline(always)]
    pub fn pb7_fmplus(&self) -> PB7_FMPLUS_R {
        PB7_FMPLUS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast-mode Plus command on PB(8)"]
    #[inline(always)]
    pub fn pb8_fmplus(&self) -> PB8_FMPLUS_R {
        PB8_FMPLUS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast-mode Plus command on PB(9)"]
    #[inline(always)]
    pub fn pb9_fmplus(&self) -> PB9_FMPLUS_R {
        PB9_FMPLUS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Ethernet PHY interface selection Other: reserved"]
    #[inline(always)]
    pub fn eth_sel_phy(&self) -> ETH_SEL_PHY_R {
        ETH_SEL_PHY_R::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 8 - booster enable Set this bit to reduce the total harmonic distortion of the analog switch when the processor supply is below 2.7 V. The booster can be activated to guaranty AC performance on analog switch when the supply is below 2.7 V. When the booster is activated, the analog switch performances are the same as with the full voltage range."]
    #[inline(always)]
    #[must_use]
    pub fn boosten(&mut self) -> BOOSTEN_W<8> {
        BOOSTEN_W::new(self)
    }
    #[doc = "Bit 9 - booster VDD selection Note: Booster must not be used when VDDA < 2.7 V, but VDD > 2.7 V (add current consumption). When both VDD < 2.7 V and VDDA < 2.7 V, booster is needed to get full AC performances from I/O analog switches."]
    #[inline(always)]
    #[must_use]
    pub fn boostvddsel(&mut self) -> BOOSTVDDSEL_W<9> {
        BOOSTVDDSEL_W::new(self)
    }
    #[doc = "Bit 16 - Fast-mode Plus command on PB(6)"]
    #[inline(always)]
    #[must_use]
    pub fn pb6_fmplus(&mut self) -> PB6_FMPLUS_W<16> {
        PB6_FMPLUS_W::new(self)
    }
    #[doc = "Bit 17 - Fast-mode Plus command on PB(7)"]
    #[inline(always)]
    #[must_use]
    pub fn pb7_fmplus(&mut self) -> PB7_FMPLUS_W<17> {
        PB7_FMPLUS_W::new(self)
    }
    #[doc = "Bit 18 - Fast-mode Plus command on PB(8)"]
    #[inline(always)]
    #[must_use]
    pub fn pb8_fmplus(&mut self) -> PB8_FMPLUS_W<18> {
        PB8_FMPLUS_W::new(self)
    }
    #[doc = "Bit 19 - Fast-mode Plus command on PB(9)"]
    #[inline(always)]
    #[must_use]
    pub fn pb9_fmplus(&mut self) -> PB9_FMPLUS_W<19> {
        PB9_FMPLUS_W::new(self)
    }
    #[doc = "Bits 21:23 - Ethernet PHY interface selection Other: reserved"]
    #[inline(always)]
    #[must_use]
    pub fn eth_sel_phy(&mut self) -> ETH_SEL_PHY_W<21> {
        ETH_SEL_PHY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SBS product mode and configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sbs_pmcr](index.html) module"]
pub struct SBS_PMCR_SPEC;
impl crate::RegisterSpec for SBS_PMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sbs_pmcr::R](R) reader structure"]
impl crate::Readable for SBS_PMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sbs_pmcr::W](W) writer structure"]
impl crate::Writable for SBS_PMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBS_PMCR to value 0"]
impl crate::Resettable for SBS_PMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
