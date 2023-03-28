#[doc = "Register `TAMP_SECCFGR` reader"]
pub struct R(crate::R<TAMP_SECCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_SECCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_SECCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_SECCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_SECCFGR` writer"]
pub struct W(crate::W<TAMP_SECCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_SECCFGR_SPEC>;
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
impl From<crate::W<TAMP_SECCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_SECCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKPRWSEC` reader - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
pub type BKPRWSEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKPRWSEC` writer - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
pub type BKPRWSEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_SECCFGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CNT1SEC` reader - Monotonic counter 1 secure protection"]
pub type CNT1SEC_R = crate::BitReader<bool>;
#[doc = "Field `CNT1SEC` writer - Monotonic counter 1 secure protection"]
pub type CNT1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SECCFGR_SPEC, bool, O>;
#[doc = "Field `BKPWSEC` reader - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
pub type BKPWSEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKPWSEC` writer - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
pub type BKPWSEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAMP_SECCFGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BHKLOCK` reader - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
pub type BHKLOCK_R = crate::BitReader<bool>;
#[doc = "Field `BHKLOCK` writer - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
pub type BHKLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SECCFGR_SPEC, bool, O>;
#[doc = "Field `TAMPSEC` reader - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
pub type TAMPSEC_R = crate::BitReader<bool>;
#[doc = "Field `TAMPSEC` writer - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
pub type TAMPSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAMP_SECCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    pub fn bkprwsec(&self) -> BKPRWSEC_R {
        BKPRWSEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Monotonic counter 1 secure protection"]
    #[inline(always)]
    pub fn cnt1sec(&self) -> CNT1SEC_R {
        CNT1SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    pub fn bkpwsec(&self) -> BKPWSEC_R {
        BKPWSEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
    #[inline(always)]
    pub fn bhklock(&self) -> BHKLOCK_R {
        BHKLOCK_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
    #[inline(always)]
    pub fn tampsec(&self) -> TAMPSEC_R {
        TAMPSEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Backup registers read/write protection offset Protection zone 1 is defined for backup registers from TAMP_BKP0R to TAMP_BKPxR (x = BKPRWSEC-1, from 0 to 128). if TZEN=1, these backup registers can be read and written only with secure access. If TZEN=0: the protection zone 1 can be read and written with non-secure access. If BKPRWSEC = 0: there is no protection zone 1. If BKPRWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    #[must_use]
    pub fn bkprwsec(&mut self) -> BKPRWSEC_W<0> {
        BKPRWSEC_W::new(self)
    }
    #[doc = "Bit 15 - Monotonic counter 1 secure protection"]
    #[inline(always)]
    #[must_use]
    pub fn cnt1sec(&mut self) -> CNT1SEC_W<15> {
        CNT1SEC_W::new(self)
    }
    #[doc = "Bits 16:23 - Backup registers write protection offset Protection zone 2 is defined for backup registers from TAMP_BKPyR (y = BKPRWSEC, from 0 to 128) to TAMP_BKPzR (z = BKPWSEC-1, from 0 to 128, BKPWSEC ≥ BKPRWSEC): if TZEN=1, these backup registers can be written only with secure access. They can be read with secure or non-secure access. Protection zone 3 defined for backup registers from TAMP_BKPtR (t = BKPWSEC, from 0 to 127). They can be read or written with secure or non-secure access. If TZEN=0: the protection zone 2 can be read and written with non-secure access. If BKPWSEC = 0 or if BKPWSEC ≤ BKPRWSEC: there is no protection zone 2. If BKPWPRIV is set, BKPRWSEC\\[7:0\\]
can be written only in privileged mode."]
    #[inline(always)]
    #[must_use]
    pub fn bkpwsec(&mut self) -> BKPWSEC_W<16> {
        BKPWSEC_W::new(self)
    }
    #[doc = "Bit 30 - Boot hardware key lock This bit can be read and can only be written to 1 by software. It is cleared by hardware together with the backup registers following a tamper detection event or when the readout protection (RDP) is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn bhklock(&mut self) -> BHKLOCK_W<30> {
        BHKLOCK_W::new(self)
    }
    #[doc = "Bit 31 - Tamper protection (excluding monotonic counters and backup registers) Note: Refer to for details on the read protection."]
    #[inline(always)]
    #[must_use]
    pub fn tampsec(&mut self) -> TAMPSEC_W<31> {
        TAMPSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP secure mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_seccfgr](index.html) module"]
pub struct TAMP_SECCFGR_SPEC;
impl crate::RegisterSpec for TAMP_SECCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_seccfgr::R](R) reader structure"]
impl crate::Readable for TAMP_SECCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_seccfgr::W](W) writer structure"]
impl crate::Writable for TAMP_SECCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAMP_SECCFGR to value 0"]
impl crate::Resettable for TAMP_SECCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
