#[doc = "Register `TIMFCR` reader"]
pub struct R(crate::R<TIMFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMFCR` writer"]
pub struct W(crate::W<TIMFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMFCR_SPEC>;
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
impl From<crate::W<TIMFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CK_PSCx` reader - HRTIM Timer x Clock prescaler"]
pub type CK_PSCX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CK_PSCx` writer - HRTIM Timer x Clock prescaler"]
pub type CK_PSCX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMFCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CONT` reader - Continuous mode"]
pub type CONT_R = crate::BitReader<bool>;
#[doc = "Field `CONT` writer - Continuous mode"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `RETRIG` reader - Re-triggerable mode"]
pub type RETRIG_R = crate::BitReader<bool>;
#[doc = "Field `RETRIG` writer - Re-triggerable mode"]
pub type RETRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `HALF` reader - Half mode enable"]
pub type HALF_R = crate::BitReader<bool>;
#[doc = "Field `HALF` writer - Half mode enable"]
pub type HALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `PSHPLL` reader - Push-Pull mode enable"]
pub type PSHPLL_R = crate::BitReader<bool>;
#[doc = "Field `PSHPLL` writer - Push-Pull mode enable"]
pub type PSHPLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `INTLVD` reader - Interleaved mode"]
pub type INTLVD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INTLVD` writer - Interleaved mode"]
pub type INTLVD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMFCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSYNCU` reader - Re-Synchronized Update"]
pub type RSYNCU_R = crate::BitReader<bool>;
#[doc = "Field `RSYNCU` writer - Re-Synchronized Update"]
pub type RSYNCU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `SYNCRSTx` reader - Synchronization Resets Timer x"]
pub type SYNCRSTX_R = crate::BitReader<bool>;
#[doc = "Field `SYNCRSTx` writer - Synchronization Resets Timer x"]
pub type SYNCRSTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `SYNCSTRTx` reader - Synchronization Starts Timer x"]
pub type SYNCSTRTX_R = crate::BitReader<bool>;
#[doc = "Field `SYNCSTRTx` writer - Synchronization Starts Timer x"]
pub type SYNCSTRTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `DELCMP2` reader - Delayed CMP2 mode"]
pub type DELCMP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELCMP2` writer - Delayed CMP2 mode"]
pub type DELCMP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMFCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `DELCMP4` reader - Delayed CMP4 mode"]
pub type DELCMP4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELCMP4` writer - Delayed CMP4 mode"]
pub type DELCMP4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMFCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TxREPU` reader - Timer x Repetition update"]
pub type TX_REPU_R = crate::BitReader<bool>;
#[doc = "Field `TxREPU` writer - Timer x Repetition update"]
pub type TX_REPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `TxRSTU` reader - Timerx reset update"]
pub type TX_RSTU_R = crate::BitReader<bool>;
#[doc = "Field `TxRSTU` writer - Timerx reset update"]
pub type TX_RSTU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `TAU` reader - TAU"]
pub type TAU_R = crate::BitReader<bool>;
#[doc = "Field `TAU` writer - TAU"]
pub type TAU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `TBU` reader - TBU"]
pub type TBU_R = crate::BitReader<bool>;
#[doc = "Field `TBU` writer - TBU"]
pub type TBU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `TCU` reader - TCU"]
pub type TCU_R = crate::BitReader<bool>;
#[doc = "Field `TCU` writer - TCU"]
pub type TCU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `TDU` reader - TDU"]
pub type TDU_R = crate::BitReader<bool>;
#[doc = "Field `TDU` writer - TDU"]
pub type TDU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `MSTU` reader - Master Timer update"]
pub type MSTU_R = crate::BitReader<bool>;
#[doc = "Field `MSTU` writer - Master Timer update"]
pub type MSTU_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `DACSYNC` reader - AC Synchronization"]
pub type DACSYNC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACSYNC` writer - AC Synchronization"]
pub type DACSYNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMFCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `PREEN` reader - Preload enable"]
pub type PREEN_R = crate::BitReader<bool>;
#[doc = "Field `PREEN` writer - Preload enable"]
pub type PREEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMFCR_SPEC, bool, O>;
#[doc = "Field `UPDGAT` reader - Update Gating"]
pub type UPDGAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPDGAT` writer - Update Gating"]
pub type UPDGAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMFCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    pub fn ck_pscx(&self) -> CK_PSCX_R {
        CK_PSCX_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    pub fn pshpll(&self) -> PSHPLL_R {
        PSHPLL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - Interleaved mode"]
    #[inline(always)]
    pub fn intlvd(&self) -> INTLVD_R {
        INTLVD_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bit 9 - Re-Synchronized Update"]
    #[inline(always)]
    pub fn rsyncu(&self) -> RSYNCU_R {
        RSYNCU_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    pub fn syncrstx(&self) -> SYNCRSTX_R {
        SYNCRSTX_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    pub fn syncstrtx(&self) -> SYNCSTRTX_R {
        SYNCSTRTX_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    pub fn delcmp2(&self) -> DELCMP2_R {
        DELCMP2_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    pub fn delcmp4(&self) -> DELCMP4_R {
        DELCMP4_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    pub fn tx_repu(&self) -> TX_REPU_R {
        TX_REPU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    pub fn tx_rstu(&self) -> TX_RSTU_R {
        TX_RSTU_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TAU"]
    #[inline(always)]
    pub fn tau(&self) -> TAU_R {
        TAU_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    pub fn tcu(&self) -> TCU_R {
        TCU_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    pub fn tdu(&self) -> TDU_R {
        TDU_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    pub fn mstu(&self) -> MSTU_R {
        MSTU_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    pub fn updgat(&self) -> UPDGAT_R {
        UPDGAT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HRTIM Timer x Clock prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ck_pscx(&mut self) -> CK_PSCX_W<0> {
        CK_PSCX_W::new(self)
    }
    #[doc = "Bit 3 - Continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<3> {
        CONT_W::new(self)
    }
    #[doc = "Bit 4 - Re-triggerable mode"]
    #[inline(always)]
    #[must_use]
    pub fn retrig(&mut self) -> RETRIG_W<4> {
        RETRIG_W::new(self)
    }
    #[doc = "Bit 5 - Half mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn half(&mut self) -> HALF_W<5> {
        HALF_W::new(self)
    }
    #[doc = "Bit 6 - Push-Pull mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn pshpll(&mut self) -> PSHPLL_W<6> {
        PSHPLL_W::new(self)
    }
    #[doc = "Bits 7:8 - Interleaved mode"]
    #[inline(always)]
    #[must_use]
    pub fn intlvd(&mut self) -> INTLVD_W<7> {
        INTLVD_W::new(self)
    }
    #[doc = "Bit 9 - Re-Synchronized Update"]
    #[inline(always)]
    #[must_use]
    pub fn rsyncu(&mut self) -> RSYNCU_W<9> {
        RSYNCU_W::new(self)
    }
    #[doc = "Bit 10 - Synchronization Resets Timer x"]
    #[inline(always)]
    #[must_use]
    pub fn syncrstx(&mut self) -> SYNCRSTX_W<10> {
        SYNCRSTX_W::new(self)
    }
    #[doc = "Bit 11 - Synchronization Starts Timer x"]
    #[inline(always)]
    #[must_use]
    pub fn syncstrtx(&mut self) -> SYNCSTRTX_W<11> {
        SYNCSTRTX_W::new(self)
    }
    #[doc = "Bits 12:13 - Delayed CMP2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp2(&mut self) -> DELCMP2_W<12> {
        DELCMP2_W::new(self)
    }
    #[doc = "Bits 14:15 - Delayed CMP4 mode"]
    #[inline(always)]
    #[must_use]
    pub fn delcmp4(&mut self) -> DELCMP4_W<14> {
        DELCMP4_W::new(self)
    }
    #[doc = "Bit 17 - Timer x Repetition update"]
    #[inline(always)]
    #[must_use]
    pub fn tx_repu(&mut self) -> TX_REPU_W<17> {
        TX_REPU_W::new(self)
    }
    #[doc = "Bit 18 - Timerx reset update"]
    #[inline(always)]
    #[must_use]
    pub fn tx_rstu(&mut self) -> TX_RSTU_W<18> {
        TX_RSTU_W::new(self)
    }
    #[doc = "Bit 19 - TAU"]
    #[inline(always)]
    #[must_use]
    pub fn tau(&mut self) -> TAU_W<19> {
        TAU_W::new(self)
    }
    #[doc = "Bit 20 - TBU"]
    #[inline(always)]
    #[must_use]
    pub fn tbu(&mut self) -> TBU_W<20> {
        TBU_W::new(self)
    }
    #[doc = "Bit 21 - TCU"]
    #[inline(always)]
    #[must_use]
    pub fn tcu(&mut self) -> TCU_W<21> {
        TCU_W::new(self)
    }
    #[doc = "Bit 22 - TDU"]
    #[inline(always)]
    #[must_use]
    pub fn tdu(&mut self) -> TDU_W<22> {
        TDU_W::new(self)
    }
    #[doc = "Bit 24 - Master Timer update"]
    #[inline(always)]
    #[must_use]
    pub fn mstu(&mut self) -> MSTU_W<24> {
        MSTU_W::new(self)
    }
    #[doc = "Bits 25:26 - AC Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn dacsync(&mut self) -> DACSYNC_W<25> {
        DACSYNC_W::new(self)
    }
    #[doc = "Bit 27 - Preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn preen(&mut self) -> PREEN_W<27> {
        PREEN_W::new(self)
    }
    #[doc = "Bits 28:31 - Update Gating"]
    #[inline(always)]
    #[must_use]
    pub fn updgat(&mut self) -> UPDGAT_W<28> {
        UPDGAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timfcr](index.html) module"]
pub struct TIMFCR_SPEC;
impl crate::RegisterSpec for TIMFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timfcr::R](R) reader structure"]
impl crate::Readable for TIMFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timfcr::W](W) writer structure"]
impl crate::Writable for TIMFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMFCR to value 0"]
impl crate::Resettable for TIMFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
