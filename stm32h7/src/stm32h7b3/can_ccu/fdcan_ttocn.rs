#[doc = "Register `FDCAN_TTOCN` reader"]
pub struct R(crate::R<FDCAN_TTOCN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_TTOCN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_TTOCN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_TTOCN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_TTOCN` writer"]
pub struct W(crate::W<FDCAN_TTOCN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_TTOCN_SPEC>;
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
impl From<crate::W<FDCAN_TTOCN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_TTOCN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SGT` reader - Set Global time"]
pub type SGT_R = crate::BitReader<bool>;
#[doc = "Field `SGT` writer - Set Global time"]
pub type SGT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
#[doc = "Field `ECS` reader - External Clock Synchronization"]
pub type ECS_R = crate::BitReader<bool>;
#[doc = "Field `ECS` writer - External Clock Synchronization"]
pub type ECS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
#[doc = "Field `SWP` reader - Stop Watch Polarity"]
pub type SWP_R = crate::BitReader<bool>;
#[doc = "Field `SWP` writer - Stop Watch Polarity"]
pub type SWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
#[doc = "Field `SWS` reader - Stop Watch Source."]
pub type SWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWS` writer - Stop Watch Source."]
pub type SWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCN_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTIE` reader - Register Time Mark Interrupt Pulse Enable"]
pub type RTIE_R = crate::BitReader<bool>;
#[doc = "Field `RTIE` writer - Register Time Mark Interrupt Pulse Enable"]
pub type RTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
#[doc = "Field `TMC` reader - Register Time Mark Compare"]
pub type TMC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMC` writer - Register Time Mark Compare"]
pub type TMC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_TTOCN_SPEC, u8, u8, 2, O>;
#[doc = "Field `TTIE` reader - Trigger Time Mark Interrupt Pulse Enable"]
pub type TTIE_R = crate::BitReader<bool>;
#[doc = "Field `TTIE` writer - Trigger Time Mark Interrupt Pulse Enable"]
pub type TTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
#[doc = "Field `GCS` reader - Gap Control Select"]
pub type GCS_R = crate::BitReader<bool>;
#[doc = "Field `GCS` writer - Gap Control Select"]
pub type GCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
#[doc = "Field `FGP` reader - Finish Gap."]
pub type FGP_R = crate::BitReader<bool>;
#[doc = "Field `FGP` writer - Finish Gap."]
pub type FGP_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
#[doc = "Field `TMG` reader - Time Mark Gap"]
pub type TMG_R = crate::BitReader<bool>;
#[doc = "Field `TMG` writer - Time Mark Gap"]
pub type TMG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
#[doc = "Field `NIG` reader - Next is Gap"]
pub type NIG_R = crate::BitReader<bool>;
#[doc = "Field `NIG` writer - Next is Gap"]
pub type NIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
#[doc = "Field `ESCN` reader - External Synchronization Control"]
pub type ESCN_R = crate::BitReader<bool>;
#[doc = "Field `ESCN` writer - External Synchronization Control"]
pub type ESCN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
#[doc = "Field `LCKC` reader - TT Operation Control Register Locked"]
pub type LCKC_R = crate::BitReader<bool>;
#[doc = "Field `LCKC` writer - TT Operation Control Register Locked"]
pub type LCKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_TTOCN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Set Global time"]
    #[inline(always)]
    pub fn sgt(&self) -> SGT_R {
        SGT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Clock Synchronization"]
    #[inline(always)]
    pub fn ecs(&self) -> ECS_R {
        ECS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Watch Polarity"]
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Stop Watch Source."]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Register Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    pub fn rtie(&self) -> RTIE_R {
        RTIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Register Time Mark Compare"]
    #[inline(always)]
    pub fn tmc(&self) -> TMC_R {
        TMC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Trigger Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    pub fn ttie(&self) -> TTIE_R {
        TTIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Gap Control Select"]
    #[inline(always)]
    pub fn gcs(&self) -> GCS_R {
        GCS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Finish Gap."]
    #[inline(always)]
    pub fn fgp(&self) -> FGP_R {
        FGP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time Mark Gap"]
    #[inline(always)]
    pub fn tmg(&self) -> TMG_R {
        TMG_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Next is Gap"]
    #[inline(always)]
    pub fn nig(&self) -> NIG_R {
        NIG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Synchronization Control"]
    #[inline(always)]
    pub fn escn(&self) -> ESCN_R {
        ESCN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - TT Operation Control Register Locked"]
    #[inline(always)]
    pub fn lckc(&self) -> LCKC_R {
        LCKC_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set Global time"]
    #[inline(always)]
    #[must_use]
    pub fn sgt(&mut self) -> SGT_W<0> {
        SGT_W::new(self)
    }
    #[doc = "Bit 1 - External Clock Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn ecs(&mut self) -> ECS_W<1> {
        ECS_W::new(self)
    }
    #[doc = "Bit 2 - Stop Watch Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn swp(&mut self) -> SWP_W<2> {
        SWP_W::new(self)
    }
    #[doc = "Bits 3:4 - Stop Watch Source."]
    #[inline(always)]
    #[must_use]
    pub fn sws(&mut self) -> SWS_W<3> {
        SWS_W::new(self)
    }
    #[doc = "Bit 5 - Register Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtie(&mut self) -> RTIE_W<5> {
        RTIE_W::new(self)
    }
    #[doc = "Bits 6:7 - Register Time Mark Compare"]
    #[inline(always)]
    #[must_use]
    pub fn tmc(&mut self) -> TMC_W<6> {
        TMC_W::new(self)
    }
    #[doc = "Bit 8 - Trigger Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ttie(&mut self) -> TTIE_W<8> {
        TTIE_W::new(self)
    }
    #[doc = "Bit 9 - Gap Control Select"]
    #[inline(always)]
    #[must_use]
    pub fn gcs(&mut self) -> GCS_W<9> {
        GCS_W::new(self)
    }
    #[doc = "Bit 10 - Finish Gap."]
    #[inline(always)]
    #[must_use]
    pub fn fgp(&mut self) -> FGP_W<10> {
        FGP_W::new(self)
    }
    #[doc = "Bit 11 - Time Mark Gap"]
    #[inline(always)]
    #[must_use]
    pub fn tmg(&mut self) -> TMG_W<11> {
        TMG_W::new(self)
    }
    #[doc = "Bit 12 - Next is Gap"]
    #[inline(always)]
    #[must_use]
    pub fn nig(&mut self) -> NIG_W<12> {
        NIG_W::new(self)
    }
    #[doc = "Bit 13 - External Synchronization Control"]
    #[inline(always)]
    #[must_use]
    pub fn escn(&mut self) -> ESCN_W<13> {
        ESCN_W::new(self)
    }
    #[doc = "Bit 15 - TT Operation Control Register Locked"]
    #[inline(always)]
    #[must_use]
    pub fn lckc(&mut self) -> LCKC_W<15> {
        LCKC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ttocn](index.html) module"]
pub struct FDCAN_TTOCN_SPEC;
impl crate::RegisterSpec for FDCAN_TTOCN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ttocn::R](R) reader structure"]
impl crate::Readable for FDCAN_TTOCN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_ttocn::W](W) writer structure"]
impl crate::Writable for FDCAN_TTOCN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDCAN_TTOCN to value 0"]
impl crate::Resettable for FDCAN_TTOCN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
